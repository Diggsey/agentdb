use std::{collections::BTreeMap, ops::Range, sync::Arc};

use byteorder::{ByteOrder, LittleEndian};
use foundationdb::options::MutationType;
use foundationdb::{FdbError, RangeOption, TransactOption, Transaction};
use futures::{select, FutureExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cancellation::{spawn_cancellable, CancellableHandle, Cancellation};
use crate::directories::{Global, RootSpace};
use crate::message::INITIAL_TS_OFFSET;
use crate::partition::partition_task;
use crate::utils::load_partition_range;
use crate::{Error, StateFn, Timestamp, GC_INTERVAL, HEARTBEAT_INTERVAL};

// Collect operations more than 5 minutes old
const GC_AGE_MS: i64 = (1000 * 60 * 5) + INITIAL_TS_OFFSET;
const GC_COUNT_PER_CLIENT: usize = 256;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct PartitionAssignment {
    pub partition_range: PartitionRange,
    pub client_count: u32,
    pub client_index: u32,
}

impl PartitionAssignment {
    fn offset(&self, index: u32) -> u32 {
        (self.partition_range.count * index) / self.client_count + self.partition_range.offset
    }
    pub fn range(&self) -> Range<u32> {
        self.offset(self.client_index)..self.offset(self.client_index + 1)
    }
}

struct ClientState {
    name: String,
    id: Uuid,
    global: Arc<Global>,
    root: Arc<RootSpace>,
    partition_assignment: PartitionAssignment,
    partition_tasks: BTreeMap<u32, CancellableHandle<()>>,
    state_fn: StateFn,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PartitionRange {
    pub offset: u32,
    pub count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ClientValue {
    pub last_active_ts: Timestamp,
    pub name: String,
}

impl ClientState {
    fn new(
        name: String,
        global: Arc<Global>,
        root: Arc<RootSpace>,
        client_id: Uuid,
        state_fn: StateFn,
    ) -> Self {
        Self {
            name,
            id: client_id,
            global,
            root,
            partition_assignment: PartitionAssignment::default(),
            partition_tasks: BTreeMap::new(),
            state_fn,
        }
    }
    async fn tick(&mut self) -> Result<(), Error> {
        log::info!("Client tick");

        // Update our own timestamp
        let current_ts = Timestamp::now();
        let client_value = ClientValue {
            last_active_ts: current_ts,
            name: self.name.clone(),
        };
        let client_key = self.root.clients.pack(&self.id);
        let client_bytes = postcard::to_stdvec(&client_value).expect("Infallible serialization");
        self.global
            .db()
            .transact_boxed(
                (&client_key, client_bytes),
                |tx, &mut (client_key, ref client_bytes)| {
                    async move {
                        tx.set(client_key, client_bytes);
                        Ok::<_, FdbError>(())
                    }
                    .boxed()
                },
                TransactOption::idempotent(),
            )
            .await?;

        // Check for changed client list
        let expired_ts = current_ts - HEARTBEAT_INTERVAL * 2;
        let new_partition_assignment = self
            .global
            .db()
            .transact_boxed(
                (&self.root, &client_key),
                |tx, &mut (root, client_key)| {
                    async move {
                        // Scan for all the active clients
                        let mut kv_stream = tx.get_ranges(root.clients.range().into(), true);
                        let mut client_count = 0;
                        let mut client_index = 0;
                        while let Some(kvs) = kv_stream.try_next().await? {
                            for kv in kvs {
                                if let Ok(client_value) =
                                    postcard::from_bytes::<ClientValue>(kv.value())
                                {
                                    if client_value.last_active_ts < expired_ts {
                                        tx.clear(kv.key());
                                    } else {
                                        if kv.key() == client_key {
                                            client_index = client_count;
                                        }
                                        client_count += 1;
                                    }
                                }
                            }
                        }

                        // Read the partition count
                        let partition_range =
                            load_partition_range(tx, &root.partition_range_recv, true).await?;

                        // Return the new partition assignment
                        Ok::<_, FdbError>(PartitionAssignment {
                            partition_range,
                            client_index,
                            client_count,
                        })
                    }
                    .boxed()
                },
                TransactOption::idempotent(),
            )
            .await?;

        if new_partition_assignment != self.partition_assignment {
            log::info!("Partition assignment changed");

            self.partition_assignment = new_partition_assignment;
            let partition_range = self.partition_assignment.range();

            // Stop tasks for partitions we no longer own
            self.partition_tasks = self.partition_tasks.split_off(&partition_range.start);
            self.partition_tasks.split_off(&partition_range.end);

            // Start tasks for partitions we now own
            for partition in partition_range {
                let global = self.global.clone();
                let root = self.root.clone();
                let state_fn = self.state_fn.clone();
                self.partition_tasks.entry(partition).or_insert_with(|| {
                    spawn_cancellable(|c| partition_task(global, root, partition, state_fn, c))
                });
            }
        }

        Ok(())
    }

    async fn gc_range(
        tx: &Transaction,
        gc_ts: i64,
        range: impl Into<RangeOption<'_>>,
        reverse: bool,
    ) -> Result<(), Error> {
        let mut range = range.into();
        range.limit = Some(GC_COUNT_PER_CLIENT);
        range.reverse = reverse;
        let mut stream = tx.get_ranges(range, true);
        while let Some(values) = stream.try_next().await? {
            for value in values {
                let ts = LittleEndian::read_i64(value.value());
                if ts < gc_ts {
                    tx.atomic_op(value.key(), value.value(), MutationType::CompareAndClear)
                }
            }
        }
        Ok(())
    }

    async fn gc(&mut self) -> Result<(), Error> {
        let gc_id = Uuid::new_v4();
        let current_ts = Timestamp::now().millis();
        let gc_ts = current_ts - GC_AGE_MS;

        self.global
            .db()
            .transact_boxed(
                &self.root,
                |tx, &mut root| {
                    async move {
                        Self::gc_range(tx, gc_ts, root.operation_ts.subrange(gc_id..), false)
                            .await?;
                        Self::gc_range(tx, gc_ts, root.operation_ts.subrange(..gc_id), true)
                            .await?;

                        Ok::<_, Error>(())
                    }
                    .boxed()
                },
                TransactOption::idempotent(),
            )
            .await?;

        Ok(())
    }
}

pub async fn client_task(
    name: String,
    global: Arc<Global>,
    root: String,
    state_fn: StateFn,
    mut cancellation: Cancellation,
) -> Result<(), Error> {
    let root = global.root(&root).await?;
    let client_id = Uuid::new_v4();
    let mut client_state = ClientState::new(name, global, root, client_id, state_fn);
    let mut interval = tokio::time::interval(HEARTBEAT_INTERVAL);
    let mut gc_interval = tokio::time::interval(GC_INTERVAL);
    log::info!("Starting client...");

    loop {
        select! {
            _ = cancellation => break,
            _ = interval.tick().fuse() => client_state.tick().await?,
            _ = gc_interval.tick().fuse() => client_state.gc().await?,
        }
    }
    Ok(())
}
