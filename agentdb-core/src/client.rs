use std::{collections::BTreeMap, ops::Range, sync::Arc};

use byteorder::{ByteOrder, LittleEndian};
use chrono::{TimeZone, Utc};
use foundationdb::{Database, FdbError, RangeOption, TransactOption};
use futures::{select, FutureExt, TryStreamExt};
use uuid::Uuid;

use crate::{
    cancellation::{spawn_cancellable, CancellableHandle, Cancellation},
    partition::partition_task,
    subspace::Subspace,
    StateFn, DEFAULT_PARTITION_COUNT, HEARTBEAT_INTERVAL,
};

pub static CLIENT_SPACE: Subspace<(Uuid,)> = Subspace::new(b"c");
pub static PARTITION_COUNT: Subspace<()> = Subspace::new(b"pc");
pub static PARTITION_COUNT_SEND: Subspace<()> = PARTITION_COUNT.subspace(b"s");
pub static PARTITION_COUNT_RECV: Subspace<()> = PARTITION_COUNT.subspace(b"r");

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
struct PartitionAssignment {
    partition_count: u32,
    client_count: u32,
    client_index: u32,
}

impl PartitionAssignment {
    fn offset(&self, index: u32) -> u32 {
        (self.partition_count * index) / self.client_count
    }
    pub fn range(&self) -> Range<u32> {
        self.offset(self.client_index)..self.offset(self.client_index + 1)
    }
}

struct ClientState {
    db: Arc<Database>,
    root: Vec<u8>,
    client_key: Vec<u8>,
    client_range: RangeOption<'static>,
    partition_count_recv_key: Vec<u8>,
    partition_assignment: PartitionAssignment,
    partition_tasks: BTreeMap<u32, CancellableHandle<()>>,
    state_fn: StateFn,
}

impl ClientState {
    fn new(db: Arc<Database>, root: Vec<u8>, client_id: Uuid, state_fn: StateFn) -> Self {
        Self {
            db,
            client_key: CLIENT_SPACE.key(&root, (client_id,)),
            client_range: CLIENT_SPACE.range(&root, ()).into(),
            partition_count_recv_key: PARTITION_COUNT_RECV.key(&root, ()),
            partition_assignment: PartitionAssignment::default(),
            partition_tasks: BTreeMap::new(),
            state_fn,
            root,
        }
    }
    async fn tick(&mut self) -> anyhow::Result<()> {
        log::info!("Client tick");

        // Update our own timestamp
        let current_ts = Utc::now();
        self.db
            .transact_boxed(
                (),
                |tx, ()| {
                    let client_key = self.client_key.clone();
                    Box::pin(async move {
                        tx.set(&client_key, &current_ts.timestamp_millis().to_le_bytes());
                        Ok::<_, FdbError>(())
                    })
                },
                TransactOption::idempotent(),
            )
            .await?;

        // Check for changed client list
        let expired_ts = current_ts - chrono::Duration::from_std(HEARTBEAT_INTERVAL * 2)?;
        let new_partition_assignment = self
            .db
            .transact_boxed(
                (),
                |tx, ()| {
                    let client_range = self.client_range.clone();
                    let client_key = self.client_key.clone();
                    let partition_count_recv_key = self.partition_count_recv_key.clone();
                    Box::pin(async move {
                        // Scan for all the active clients
                        let mut kv_stream = tx.get_ranges(client_range, true);
                        let mut client_count = 0;
                        let mut client_index = 0;
                        while let Some(kvs) = kv_stream.try_next().await? {
                            for kv in kvs {
                                let ts = Utc.timestamp_millis(LittleEndian::read_i64(kv.value()));
                                if ts < expired_ts {
                                    tx.clear(kv.key());
                                } else {
                                    if kv.key() == client_key {
                                        client_index = client_count;
                                    }
                                    client_count += 1;
                                }
                            }
                        }

                        // Read the partition count
                        let partition_count = tx
                            .get(&partition_count_recv_key, true)
                            .await?
                            .as_deref()
                            .map(LittleEndian::read_u32)
                            .unwrap_or(DEFAULT_PARTITION_COUNT);

                        // Return the new partition assignment
                        Ok::<_, FdbError>(PartitionAssignment {
                            partition_count,
                            client_index,
                            client_count,
                        })
                    })
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
                let db = &self.db;
                let root = &self.root;
                let state_fn = &self.state_fn;
                self.partition_tasks.entry(partition).or_insert_with(|| {
                    spawn_cancellable(|c| {
                        partition_task(db.clone(), root.clone(), partition, state_fn.clone(), c)
                    })
                });
            }
        }

        Ok(())
    }
}

pub async fn client_task(
    db: Arc<Database>,
    root: Vec<u8>,
    state_fn: StateFn,
    mut cancellation: Cancellation,
) -> anyhow::Result<()> {
    let client_id = Uuid::new_v4();
    let mut client_state = ClientState::new(db, root, client_id, state_fn);
    let mut interval = tokio::time::interval(HEARTBEAT_INTERVAL);
    log::info!("Starting client...");

    loop {
        select! {
            _ = cancellation => break,
            _ = interval.tick().fuse() => client_state.tick().await?,
        }
    }
    Ok(())
}
