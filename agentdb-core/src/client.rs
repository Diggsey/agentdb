use std::{collections::BTreeMap, ops::Range, sync::Arc};

use foundationdb::{Database, FdbError, RangeOption, TransactOption};
use futures::{select, FutureExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::cancellation::{spawn_cancellable, CancellableHandle, Cancellation};
use crate::partition::partition_task;
use crate::subspace::Subspace;
use crate::utils::load_partition_range;
use crate::{Error, StateFn, Timestamp, HEARTBEAT_INTERVAL};

pub static MAGIC: Subspace<()> = Subspace::new(b"agentdb");
pub static CLIENT_SPACE: Subspace<(Uuid,)> = Subspace::new(b"c");
pub static PARTITION_COUNT: Subspace<()> = Subspace::new(b"pc");
pub static PARTITION_COUNT_SEND: Subspace<()> = PARTITION_COUNT.subspace(b"s");
pub static PARTITION_COUNT_RECV: Subspace<()> = PARTITION_COUNT.subspace(b"r");

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
    db: Arc<Database>,
    root: Vec<u8>,
    client_key: Vec<u8>,
    client_range: RangeOption<'static>,
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
        db: Arc<Database>,
        root: Vec<u8>,
        client_id: Uuid,
        state_fn: StateFn,
    ) -> Self {
        Self {
            name,
            db,
            client_key: CLIENT_SPACE.key(&root, (client_id,)),
            client_range: CLIENT_SPACE.range(&root, ()).into(),
            partition_assignment: PartitionAssignment::default(),
            partition_tasks: BTreeMap::new(),
            state_fn,
            root,
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
        self.db
            .transact_boxed(
                (),
                |tx, ()| {
                    let client_key = self.client_key.clone();
                    let client_bytes =
                        postcard::to_stdvec(&client_value).expect("Infallible serialization");
                    Box::pin(async move {
                        tx.set(&client_key, &client_bytes);
                        Ok::<_, FdbError>(())
                    })
                },
                TransactOption::idempotent(),
            )
            .await?;

        // Check for changed client list
        let expired_ts = current_ts - HEARTBEAT_INTERVAL * 2;
        let new_partition_assignment = self
            .db
            .transact_boxed(
                (),
                |tx, ()| {
                    let root = self.root.clone();
                    let client_range = self.client_range.clone();
                    let client_key = self.client_key.clone();
                    Box::pin(async move {
                        // Scan for all the active clients
                        let mut kv_stream = tx.get_ranges(client_range, true);
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
                            load_partition_range(tx, &root, &PARTITION_COUNT_RECV).await?;

                        // Return the new partition assignment
                        Ok::<_, FdbError>(PartitionAssignment {
                            partition_range,
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
    name: String,
    db: Arc<Database>,
    root: Vec<u8>,
    state_fn: StateFn,
    mut cancellation: Cancellation,
) -> Result<(), Error> {
    db.transact_boxed(
        (),
        |tx, ()| {
            let magic_key = MAGIC.key(&root, ());

            async move {
                tx.set(&magic_key, b"agentdb");
                Ok::<_, FdbError>(())
            }
            .boxed()
        },
        TransactOption::idempotent(),
    )
    .await?;

    let client_id = Uuid::new_v4();
    let mut client_state = ClientState::new(name, db, root, client_id, state_fn);
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
