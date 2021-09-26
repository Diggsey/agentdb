use std::{
    collections::{hash_map, BTreeMap, HashMap},
    convert::TryFrom,
    fmt::{Debug, Display},
    future::Future,
    ops::Range,
    sync::Arc,
    time::Duration,
};

use anyhow::anyhow;
use byteorder::{ByteOrder, LittleEndian};
use chrono::{DateTime, TimeZone, Utc};
use foundationdb::{
    future::FdbValue, options::MutationType, tuple::Versionstamp, Database, FdbError, RangeOption,
    TransactOption, Transaction,
};
use futures::{future::FusedFuture, pin_mut, select, FutureExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use subspace::Subspace;
use uuid::Uuid;

use cancellation::{spawn_cancellable, CancellableHandle, Cancellation};

mod blob;
pub mod cancellation;
mod subspace;

const DEFAULT_PARTITION_COUNT: u32 = 100;

static CLIENT_SPACE: Subspace<(Uuid,)> = Subspace::new(b"c");
static PARTITION_COUNT: Subspace<()> = Subspace::new(b"pc");
static PARTITION_COUNT_SEND: Subspace<()> = PARTITION_COUNT.subspace(b"s");
static PARTITION_COUNT_RECV: Subspace<()> = PARTITION_COUNT.subspace(b"r");
static PARTITION_SPACE: Subspace<(u32,)> = Subspace::new(b"p");
static PARTITION_MODIFIED: Subspace<(u32,)> = PARTITION_SPACE.subspace::<()>(b"mod");
static PARTITION_MESSAGE_SPACE: Subspace<(u32, DateTime<Utc>, Versionstamp, u32)> =
    PARTITION_SPACE.subspace::<(DateTime<Utc>, Versionstamp, u32)>(b"m");
static PARTITION_BATCH_SPACE: Subspace<(u32, Uuid, Versionstamp)> =
    PARTITION_SPACE.subspace::<(Uuid, Versionstamp)>(b"bt");

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const MAX_BATCH_SIZE: usize = 100;

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

struct PartitionState {
    db: Arc<Database>,
    root: Vec<u8>,
    partition: u32,
    cancellation: Cancellation,
    partition_modified_key: Vec<u8>,
    state_fn: StateFn,
}

#[derive(Serialize, Deserialize)]
struct MessageHeader {
    recipient_id: Uuid,
    blob_id: Uuid,
}

pub struct Error(pub anyhow::Error);

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl TryFrom<Error> for FdbError {
    type Error = Error;

    fn try_from(value: Error) -> Result<Self, Self::Error> {
        value.0.downcast().map_err(Error)
    }
}

impl<E> From<E> for Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

impl From<Error> for anyhow::Error {
    fn from(e: Error) -> Self {
        e.0
    }
}

async fn get_first_in_range(
    tx: &Transaction,
    mut range: RangeOption<'_>,
    snapshot: bool,
) -> Result<Option<FdbValue>, Error> {
    range.limit = Some(1);
    let mut stream = tx.get_ranges(range, snapshot);
    while let Some(values) = stream.try_next().await? {
        for value in values {
            return Ok(Some(value));
        }
    }
    Ok(None)
}

#[derive(Debug)]
pub struct StateFnInput {
    pub id: Uuid,
    pub state: Option<Vec<u8>>,
    pub messages: Vec<Vec<u8>>,
}

pub struct MessageToSend {
    pub recipient_root: Vec<u8>,
    pub recipient_id: Uuid,
    pub when: DateTime<Utc>,
    pub content: Vec<u8>,
}

#[derive(Clone)]
pub struct Context {
    db: Arc<Database>,
}

pub type CommitHook = Box<dyn FnOnce(&Context) + Send + Sync + 'static>;

pub struct StateFnOutput {
    pub state: Option<Vec<u8>>,
    pub messages: Vec<MessageToSend>,
    pub commit_hook: CommitHook,
}

pub type StateFn = Arc<dyn Fn(StateFnInput) -> StateFnOutput + Send + Sync>;

fn partition_for_recipient(recipient_id: Uuid, partition_count: u32) -> u32 {
    let hash = recipient_id.as_u128();
    let hash = ((hash >> 64) ^ hash) as u64;
    let hash = ((hash >> 32) ^ hash) as u32;
    hash % partition_count
}

pub async fn send_messages(
    tx: &Transaction,
    msgs: &[MessageToSend],
    user_version: u16,
) -> Result<(), Error> {
    let mut partition_counts = HashMap::new();

    for (idx, msg) in msgs.into_iter().enumerate() {
        let entry = partition_counts.entry(&msg.recipient_root);
        let partition_count = match entry {
            hash_map::Entry::Occupied(occ) => *occ.get(),
            hash_map::Entry::Vacant(vac) => *vac.insert(
                tx.get(&PARTITION_COUNT_SEND.key(&msg.recipient_root, ()), true)
                    .await?
                    .as_deref()
                    .map(LittleEndian::read_u32)
                    .unwrap_or(DEFAULT_PARTITION_COUNT),
            ),
        };

        let msg_id = Uuid::new_v4();
        blob::store(tx, &msg.recipient_root, msg_id, &msg.content);
        let msg_hdr = postcard::to_stdvec(&MessageHeader {
            recipient_id: msg.recipient_id,
            blob_id: msg_id,
        })?;

        let partition = partition_for_recipient(msg.recipient_id, partition_count);

        let key = PARTITION_MESSAGE_SPACE.key(
            &msg.recipient_root,
            (
                partition,
                msg.when,
                Versionstamp::incomplete(user_version),
                idx as u32,
            ),
        );
        tx.atomic_op(&key, &msg_hdr, MutationType::SetVersionstampedKey);
    }

    Ok(())
}

impl PartitionState {
    pub fn new(
        db: Arc<Database>,
        root: Vec<u8>,
        partition: u32,
        state_fn: StateFn,
        cancellation: Cancellation,
    ) -> Self {
        Self {
            db,
            partition,
            cancellation,
            partition_modified_key: PARTITION_MODIFIED.key(&root, (partition,)),
            state_fn,
            root,
        }
    }
    async fn rollup_messages(&mut self) -> Result<impl Future + FusedFuture, Error> {
        // Roll up all the messages in the partition into batches, and get back a future that
        // will resolve when either a new message is added, or a scheduled message becomes ready.
        self.db
            .transact_boxed(
                (),
                |tx, ()| {
                    // Prepare a bunch of variables for the async block
                    let root = self.root.clone();
                    let partition = self.partition;
                    let partition_modified_key = self.partition_modified_key.clone();
                    let ts = Utc::now();
                    let mut partition_message_subrange: RangeOption = PARTITION_MESSAGE_SPACE
                        .subrange::<_, (Versionstamp, u32), _, (Versionstamp, u32)>(
                            &self.root,
                            (self.partition, Utc.timestamp_millis(0)),
                            (self.partition, ts),
                        )
                        .into();
                    partition_message_subrange.limit = Some(u16::MAX as usize);
                    let partition_message_fut_range: RangeOption = PARTITION_MESSAGE_SPACE
                        .subrange::<_, (Versionstamp, u32), _, (Versionstamp, u32)>(
                            &self.root,
                            (self.partition, ts),
                            (self.partition + 1, Utc.timestamp_millis(0)),
                        )
                        .into();

                    // Transaction
                    Box::pin(async move {
                        // Find all messages which are ready to be received
                        let mut msg_stream =
                            tx.get_ranges(partition_message_subrange.clone(), true);

                        // Group the messages by recipient
                        let mut msg_index = 0;
                        while let Some(msgs) = msg_stream.try_next().await? {
                            for msg in msgs {
                                // Decode the message header
                                let msg_hdr: MessageHeader = postcard::from_bytes(msg.value())?;

                                // Figure out where the message should be batched.
                                let batch_key = PARTITION_BATCH_SPACE.key(
                                    &root,
                                    (
                                        partition,
                                        msg_hdr.recipient_id,
                                        Versionstamp::incomplete(msg_index),
                                    ),
                                );
                                msg_index += 1;
                                tx.atomic_op(
                                    &batch_key,
                                    msg.value(),
                                    MutationType::SetVersionstampedKey,
                                );
                                tx.clear(msg.key());
                            }
                        }
                        log::info!(
                            "Rolled up {} message(s) in partition {}",
                            msg_index,
                            partition
                        );

                        // Find out how long to wait for the next scheduled message.
                        // Or just wait two minutes if there's no scheduled message.
                        let mut delay = Duration::from_secs(120);
                        if let Some(msg) =
                            get_first_in_range(tx, partition_message_fut_range.clone(), true)
                                .await?
                        {
                            if let Some(tuple) = PARTITION_MESSAGE_SPACE.decode(&root, msg.key()) {
                                delay = (tuple.1 - ts).to_std().expect("Timestamp in the future");
                            }
                        }

                        Ok::<_, Error>(
                            tokio::time::timeout(delay, tx.watch(&partition_modified_key)).fuse(),
                        )
                    })
                },
                TransactOption::idempotent(),
            )
            .await
    }

    async fn process_batch(
        &mut self,
        batch_range: RangeOption<'static>,
    ) -> Result<Option<RangeOption<'static>>, Error> {
        let max_batch_size = MAX_BATCH_SIZE;
        let res = self
            .db
            .transact_boxed(
                max_batch_size,
                |tx, max_batch_size| {
                    let root = self.root.clone();
                    let partition = self.partition;
                    let mut batch_range = batch_range.clone();
                    let state_fn = self.state_fn.clone();

                    // Automatically reduce batch size on failure
                    if *max_batch_size > 1 {
                        *max_batch_size >>= 1;
                    }

                    // Prepare a bunch of variables for the async block
                    Box::pin(async move {
                        let (_, recipient_id, _) = if let Some(msg) =
                            get_first_in_range(tx, batch_range.clone(), false).await?
                        {
                            PARTITION_BATCH_SPACE
                                .decode(&root, msg.key())
                                .ok_or_else(|| Error(anyhow!("Failed to decode batch key")))?
                        } else {
                            return Ok(None);
                        };

                        let recipient_state = blob::load(tx, &root, recipient_id, false).await?;

                        let mut recipient_range: RangeOption = PARTITION_BATCH_SPACE
                            .range::<_, (Versionstamp,)>(&root, (partition, recipient_id))
                            .into();
                        recipient_range.limit = Some(*max_batch_size);
                        batch_range.begin = recipient_range.end.clone();

                        let mut all_blob_ids = Vec::new();
                        let mut msg_stream = tx.get_ranges(recipient_range, false);
                        while let Some(msgs) = msg_stream.try_next().await? {
                            for msg in msgs {
                                // Decode the message header
                                let msg_hdr: MessageHeader = postcard::from_bytes(msg.value())?;
                                tx.clear(msg.key());
                                all_blob_ids.push(msg_hdr.blob_id);
                            }
                        }

                        let mut all_msgs = Vec::with_capacity(all_blob_ids.len());
                        for blob_id in all_blob_ids {
                            all_msgs.push(
                                blob::load(tx, &root, blob_id, true)
                                    .await?
                                    .ok_or_else(|| Error(anyhow!("Blob not found: {}", blob_id)))?,
                            );
                        }

                        log::info!(
                            "Loaded {} message(s) in partition {}",
                            all_msgs.len(),
                            partition
                        );

                        let state_fn_input = StateFnInput {
                            id: recipient_id,
                            state: recipient_state,
                            messages: all_msgs,
                        };
                        let state_fn_output = state_fn(state_fn_input);

                        if let Some(state) = state_fn_output.state {
                            blob::store(tx, &root, recipient_id, &state);
                        } else {
                            blob::delete(tx, &root, recipient_id);
                        }

                        send_messages(tx, &state_fn_output.messages, 0).await?;

                        Ok::<_, Error>(Some((batch_range, state_fn_output.commit_hook)))
                    })
                },
                TransactOption::idempotent(),
            )
            .await?;

        Ok(if let Some((batch_range, commit_hook)) = res {
            commit_hook(&Context {
                db: self.db.clone(),
            });
            Some(batch_range)
        } else {
            None
        })
    }
    async fn process_batches(&mut self) -> Result<bool, Error> {
        let mut batch_range = PARTITION_BATCH_SPACE
            .range::<_, (Uuid, Versionstamp)>(&self.root, (self.partition,))
            .into();
        let mut was_empty = true;
        while let Some(new_range) = self.process_batch(batch_range).await? {
            batch_range = new_range;
            was_empty = false;
        }
        Ok(was_empty)
    }
    async fn step(&mut self) -> anyhow::Result<()> {
        let watch_fut = self.rollup_messages().await?;
        pin_mut!(watch_fut);

        let was_empty = self.process_batches().await?;

        // If there was nothing to process, sleep until there is a new message
        if was_empty {
            select! {
                _ = watch_fut => {},
                _ = &mut self.cancellation => {},
            }
        }
        Ok(())
    }
    pub async fn run(mut self) -> anyhow::Result<()> {
        while !self.cancellation.is_cancelled() {
            self.step().await?;
        }
        Ok(())
    }
}

pub async fn partition_task(
    db: Arc<Database>,
    root: Vec<u8>,
    partition: u32,
    state_fn: StateFn,
    cancellation: Cancellation,
) {
    log::info!("Starting partition {}", partition);
    while !cancellation.is_cancelled() {
        let partition_state = PartitionState::new(
            db.clone(),
            root.clone(),
            partition,
            state_fn.clone(),
            cancellation.clone(),
        );
        if let Err(e) = partition_state.run().await {
            log::error!("Failed to run partition {}: {:?}", partition, e);
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
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

pub async fn run(db: Arc<Database>, root: Vec<u8>, state_fn: StateFn) {
    let handle = spawn_cancellable(|c| client_task(db, root, state_fn, c));
    handle.await.unwrap().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
