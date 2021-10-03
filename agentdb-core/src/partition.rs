use std::{future::Future, marker::PhantomData, sync::Arc, time::Duration};

use anyhow::anyhow;
use chrono::{DateTime, TimeZone, Utc};
use foundationdb::{
    options::MutationType, tuple::Versionstamp, Database, KeySelector, RangeOption, TransactOption,
};
use futures::{future::FusedFuture, select, FutureExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    blob, cancellation::Cancellation, error::Error, message::send_messages, subspace::Subspace,
    utils::get_first_in_range, HookContext, MessageHeader, StateFn, StateFnInput, MAX_BATCH_SIZE,
};

pub static PARTITION_SPACE: Subspace<(u32,)> = Subspace::new(b"p");
pub static PARTITION_MODIFIED: Subspace<(u32,)> = PARTITION_SPACE.subspace::<()>(b"mod");
pub static PARTITION_MESSAGE_SPACE: Subspace<(u32, DateTime<Utc>, Versionstamp, u32)> =
    PARTITION_SPACE.subspace::<(DateTime<Utc>, Versionstamp, u32)>(b"m");
pub static PARTITION_BATCH_SPACE: Subspace<(u32, Uuid, Versionstamp)> =
    PARTITION_SPACE.subspace::<(Uuid, Versionstamp)>(b"bt");
pub static PARTITION_AGENT_RETRY: Subspace<(u32, Uuid)> =
    PARTITION_SPACE.subspace::<(Uuid,)>(b"retry");

const MAX_POLL_INTERVAL_SECS: u64 = 120;

#[derive(Debug, thiserror::Error)]
#[error("State function returned an error")]
struct StateFnError;

struct PartitionState {
    db: Arc<Database>,
    root: Vec<u8>,
    partition: u32,
    cancellation: Cancellation,
    partition_modified_key: Vec<u8>,
    state_fn: StateFn,
}

#[derive(Debug, Copy, Clone)]
struct FoundRecipient {
    id: Uuid,
    retry_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
struct RetryAtState {
    retry_at: DateTime<Utc>,
    backoff: Duration,
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
                        let mut delay = Duration::from_secs(MAX_POLL_INTERVAL_SECS);
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
    async fn find_batch_to_process(
        &mut self,
        batch_range: RangeOption<'static>,
    ) -> Result<Option<FoundRecipient>, Error> {
        self.db
            .transact_boxed(
                (),
                |tx, ()| {
                    let root = self.root.clone();
                    let partition = self.partition;
                    let batch_range = batch_range.clone();

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

                        let retry_at_key =
                            PARTITION_AGENT_RETRY.key(&root, (partition, recipient_id));

                        let retry_at_state =
                            if let Some(retry_at_bytes) = tx.get(&retry_at_key, false).await? {
                                let mut retry_at_state: RetryAtState =
                                    postcard::from_bytes(&retry_at_bytes)?;
                                if retry_at_state.retry_at > Utc::now() {
                                    return Ok(Some(FoundRecipient {
                                        id: recipient_id,
                                        retry_at: Some(retry_at_state.retry_at),
                                    }));
                                } else {
                                    retry_at_state.retry_at = retry_at_state.retry_at
                                        + chrono::Duration::from_std(retry_at_state.backoff)?;
                                    retry_at_state.backoff += retry_at_state.backoff;
                                }
                                retry_at_state
                            } else {
                                RetryAtState {
                                    retry_at: Utc::now(),
                                    backoff: Duration::from_secs(1),
                                }
                            };

                        let retry_at_bytes = postcard::to_stdvec(&retry_at_state)?;
                        tx.set(&retry_at_key, &retry_at_bytes);

                        Ok::<_, Error>(Some(FoundRecipient {
                            id: recipient_id,
                            retry_at: None,
                        }))
                    })
                },
                TransactOption::idempotent(),
            )
            .await
    }

    async fn process_batch(
        &mut self,
        batch_range: RangeOption<'static>,
    ) -> Result<Option<FoundRecipient>, Error> {
        let recipient = if let Some(recipient) = self.find_batch_to_process(batch_range).await? {
            recipient
        } else {
            return Ok(None);
        };

        let max_batch_size = MAX_BATCH_SIZE;
        match self
            .db
            .transact_boxed(
                max_batch_size,
                |tx, max_batch_size| {
                    let root = self.root.clone();
                    let partition = self.partition;
                    let state_fn = self.state_fn.clone();
                    let retry_at_key = PARTITION_AGENT_RETRY.key(&root, (partition, recipient.id));

                    // Automatically reduce batch size on failure
                    if *max_batch_size > 1 {
                        *max_batch_size >>= 1;
                    }

                    // Prepare a bunch of variables for the async block
                    Box::pin(async move {
                        // If this transaction commits, clear the "retry_at" flag from this agent
                        tx.clear(&retry_at_key);

                        // Load the initial agent state
                        let recipient_state = blob::load(tx, &root, recipient.id, false).await?;

                        // Determine the range of keys where messages are batched
                        let mut recipient_range: RangeOption = PARTITION_BATCH_SPACE
                            .range::<_, (Versionstamp,)>(&root, (partition, recipient.id))
                            .into();
                        recipient_range.limit = Some(*max_batch_size);

                        // Load and clear all the message IDs
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

                        // Load all the message contents
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
                            root: &root,
                            tx,
                            id: recipient.id,
                            state: recipient_state,
                            messages: all_msgs,
                        };
                        let state_fn_output =
                            state_fn(state_fn_input).await.map_err(|_| StateFnError)?;

                        if let Some(state) = state_fn_output.state {
                            blob::store(tx, &root, recipient.id, &state);
                        } else {
                            blob::delete(tx, &root, recipient.id);
                        }

                        send_messages(tx, &state_fn_output.messages, 0).await?;

                        Ok::<_, Error>(state_fn_output.commit_hook)
                    })
                },
                TransactOption::idempotent(),
            )
            .await
        {
            Ok(commit_hook) => {
                // Call the commit hook
                commit_hook(&HookContext {
                    db: self.db.clone(),
                    phantom: PhantomData,
                });
            }
            Err(e) if e.0.is::<StateFnError>() => {
                // Skip
            }
            Err(e) => return Err(e),
        }

        Ok(Some(recipient))
    }
    async fn process_batches(&mut self) -> Result<Option<DateTime<Utc>>, Error> {
        // Begin with the entire partition range
        let mut batch_range: RangeOption = PARTITION_BATCH_SPACE
            .range::<_, (Uuid, Versionstamp)>(&self.root, (self.partition,))
            .into();

        // If no messages found, retry after the maximum interval
        let mut overall_retry_at =
            Some(Utc::now() + chrono::Duration::seconds(MAX_POLL_INTERVAL_SECS as i64));

        while let Some(recipient) = self.process_batch(batch_range.clone()).await? {
            // If we found and processed a batch, advance our range to exclude that agent
            batch_range.begin = KeySelector::first_greater_or_equal(
                PARTITION_BATCH_SPACE
                    .range::<_, (Versionstamp,)>(&self.root, (self.partition, recipient.id))
                    .1,
            );

            // Use the smallest retry interval of all batches we process, or `None` if any batch can
            // be retried immediately.
            overall_retry_at = overall_retry_at.min(recipient.retry_at);
        }
        Ok(overall_retry_at)
    }
    async fn step(&mut self) -> Result<(), Error> {
        let watch_fut = self.rollup_messages().await?;
        let maybe_retry_at = self.process_batches().await?;

        // If there was nothing to process, sleep until there is a new message
        if let Some(retry_at) = maybe_retry_at {
            if let Ok(duration) = retry_at.signed_duration_since(Utc::now()).to_std() {
                select! {
                    _ = tokio::time::timeout(duration, watch_fut).fuse() => {},
                    _ = &mut self.cancellation => {},
                }
            }
        }
        Ok(())
    }
    pub async fn run(mut self) -> Result<(), Error> {
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