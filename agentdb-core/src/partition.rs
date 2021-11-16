use std::{future::Future, sync::Arc, time::Duration};

use anyhow::anyhow;
use foundationdb::{
    future::FdbValue, options::MutationType, tuple::Versionstamp, Database, KeySelector,
    RangeOption, TransactOption, Transaction,
};
use futures::{future::FusedFuture, select, FutureExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    blob,
    cancellation::Cancellation,
    client::{PartitionRange, AGENT_SPACE, PARTITION_COUNT_RECV, PARTITION_COUNT_SEND},
    error::Error,
    message::send_messages,
    subspace::Subspace,
    utils::{
        get_first_in_range, load_partition_range, load_value, partition_for_recipient, save_value,
        Timestamp,
    },
    HookContext, MessageHeader, StateFn, StateFnInput, MAX_BATCH_SIZE,
};

pub static PARTITION_SPACE: Subspace<(u32,)> = Subspace::new(b"p");
pub static PARTITION_MODIFIED: Subspace<(u32,)> = PARTITION_SPACE.subspace(b"mod");
pub static PARTITION_AGENT_COUNT: Subspace<(u32,)> = PARTITION_SPACE.subspace(b"agent_count");
pub static PARTITION_MESSAGE_SPACE: Subspace<(u32, Timestamp, Versionstamp, u32)> =
    PARTITION_SPACE.subspace(b"m");
pub static PARTITION_BATCH_SPACE: Subspace<(u32, Uuid, Versionstamp)> =
    PARTITION_SPACE.subspace(b"bt");
pub static PARTITION_AGENT_RETRY: Subspace<(u32, Uuid)> = PARTITION_SPACE.subspace(b"retry");

const MAX_POLL_INTERVAL_SECS: u64 = 120;

#[derive(Debug, thiserror::Error)]
#[error("State function returned an error")]
struct StateFnError;

struct PartitionState {
    db: Arc<Database>,
    root: Vec<u8>,
    partition: u32,
    cancellation: Cancellation,
    state_fn: StateFn,
}

#[derive(Debug, Copy, Clone)]
struct FoundRecipient {
    id: Uuid,
    retry_at: Option<Timestamp>,
}

#[derive(Serialize, Deserialize)]
struct RetryAtState {
    retry_at: Timestamp,
    backoff: Duration,
}

pub fn mark_partition_modified(tx: &Transaction, root: &[u8], partition: u32) {
    let modified_key = PARTITION_MODIFIED.key(root, (partition,));
    tx.atomic_op(
        &modified_key,
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        MutationType::SetVersionstampedValue,
    );
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
            state_fn,
            root,
        }
    }
    async fn rollup_messages(&mut self) -> Result<impl Future + FusedFuture, Error> {
        // Roll up all the messages in the partition into batches, and get back a future that
        // will resolve when either a new message is added, or a scheduled message becomes ready.
        self.db
            .transact_boxed(
                (&self.root, self.partition),
                |tx, &mut (root, partition)| {
                    async move {
                        let ts = Timestamp::now();
                        let mut past_message_range: RangeOption = PARTITION_MESSAGE_SPACE
                            .subrange(root, (partition, Timestamp::zero()), (partition, ts))
                            .into();
                        past_message_range.limit = Some(u16::MAX as usize);
                        let future_message_range: RangeOption = PARTITION_MESSAGE_SPACE
                            .subrange(root, (partition, ts), (partition + 1, Timestamp::zero()))
                            .into();

                        // Find all messages which are ready to be received
                        let mut msg_stream = tx.get_ranges(past_message_range.clone(), true);

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
                            get_first_in_range(tx, future_message_range.clone(), true).await?
                        {
                            if let Some(tuple) = PARTITION_MESSAGE_SPACE.decode(&root, msg.key()) {
                                delay = tuple.1 - ts;
                            }
                        }

                        Ok::<_, Error>(
                            tokio::time::timeout(
                                delay,
                                tx.watch(&PARTITION_MODIFIED.key(&root, (partition,))),
                            )
                            .fuse(),
                        )
                    }
                    .boxed()
                },
                TransactOption::idempotent(),
            )
            .await
    }

    // Finds the next recipient in the range with pending batched messages. Returns
    // the recipient even if it's not ready to retry. The caller should skip those
    // recipients.
    async fn find_next_recipient(
        &mut self,
        batch_range: RangeOption<'static>,
    ) -> Result<Option<FoundRecipient>, Error> {
        self.db
            .transact_boxed(
                (&self.root, self.partition, batch_range),
                |tx, &mut (root, partition, ref batch_range)| {
                    async move {
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

                        let retry_at_state = if let Some(mut retry_at_state) =
                            load_value::<RetryAtState>(tx, &retry_at_key, false).await?
                        {
                            if retry_at_state.retry_at > Timestamp::now() {
                                return Ok(Some(FoundRecipient {
                                    id: recipient_id,
                                    retry_at: Some(retry_at_state.retry_at),
                                }));
                            } else {
                                retry_at_state.retry_at += retry_at_state.backoff;
                                retry_at_state.backoff += retry_at_state.backoff;
                            }
                            retry_at_state
                        } else {
                            RetryAtState {
                                retry_at: Timestamp::now(),
                                backoff: Duration::from_secs(1),
                            }
                        };

                        save_value(tx, &retry_at_key, &retry_at_state);

                        Ok::<_, Error>(Some(FoundRecipient {
                            id: recipient_id,
                            retry_at: None,
                        }))
                    }
                    .boxed()
                },
                TransactOption::idempotent(),
            )
            .await
    }

    async fn process_batch(
        &mut self,
        batch_range: RangeOption<'static>,
    ) -> Result<Option<FoundRecipient>, Error> {
        let recipient = if let Some(recipient) = self.find_next_recipient(batch_range).await? {
            recipient
        } else {
            return Ok(None);
        };

        // Skip the recipient if it's not ready to retry from an error
        if recipient.retry_at.is_some() {
            return Ok(Some(recipient));
        }

        let max_batch_size = MAX_BATCH_SIZE;
        match self
            .db
            .transact_boxed(
                (&self.root, self.partition, &self.state_fn, max_batch_size),
                |tx, &mut (root, partition, ref state_fn, ref mut max_batch_size)| {
                    async move {
                        // Automatically reduce batch size on failure
                        if *max_batch_size > 1 {
                            *max_batch_size >>= 1;
                        }

                        // Load the initial agent state
                        let recipient_state = blob::load(tx, &root, recipient.id, false).await?;

                        // Determine the range of keys where messages are batched
                        let mut recipient_range: RangeOption = PARTITION_BATCH_SPACE
                            .range(&root, (partition, recipient.id))
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
                            blob::delete(tx, &root, blob_id);
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
                        let exist_before = state_fn_input.state.is_some();
                        let state_fn_output =
                            state_fn(state_fn_input).await.map_err(|_| StateFnError)?;
                        let exist_after = state_fn_output.state.is_some();

                        if let Some(state) = state_fn_output.state {
                            blob::store(tx, &root, recipient.id, &state);
                        } else {
                            blob::delete(tx, &root, recipient.id);
                        }

                        send_messages(tx, &state_fn_output.messages, 0).await?;

                        // Clear the "retry_at" flag from this agent
                        tx.clear(&PARTITION_AGENT_RETRY.key(&root, (partition, recipient.id)));

                        // If agent was created or destroyed
                        if exist_before != exist_after {
                            let agent_key = AGENT_SPACE.key(&root, (recipient.id,));
                            let agent_count_delta: i64 = if exist_after {
                                tx.set(&agent_key, &[]);
                                1
                            } else {
                                tx.clear(&agent_key);
                                -1
                            };
                            // Update partition agent count
                            tx.atomic_op(
                                &PARTITION_AGENT_COUNT.key(&root, (partition,)),
                                &agent_count_delta.to_le_bytes(),
                                MutationType::Add,
                            );
                        }

                        Ok::<_, Error>(state_fn_output.commit_hook)
                    }
                    .boxed()
                },
                TransactOption {
                    retry_limit: Some(5),
                    time_out: None,
                    is_idempotent: true,
                },
            )
            .await
        {
            Ok(commit_hook) => {
                // Call the commit hook
                commit_hook(HookContext {
                    db: self.db.clone(),
                });
            }
            Err(e) if e.0.is::<StateFnError>() => {
                // If the error was returned by the state function,
                // there's nothing we can do to progress the agent,
                // so just move on.
                log::error!("{}", e);
            }
            Err(e) => return Err(e),
        }

        Ok(Some(recipient))
    }
    async fn process_batches(&mut self) -> Result<Option<Timestamp>, Error> {
        // Begin with the entire partition range
        let mut batch_range: RangeOption = PARTITION_BATCH_SPACE
            .range(&self.root, (self.partition,))
            .into();

        // If no messages found, retry after the maximum interval
        let mut overall_retry_at =
            Some(Timestamp::now() + Duration::from_secs(MAX_POLL_INTERVAL_SECS));

        while let Some(recipient) = self.process_batch(batch_range.clone()).await? {
            // If we found and processed a batch, advance our range to exclude that agent
            batch_range.begin = KeySelector::first_greater_or_equal(
                PARTITION_BATCH_SPACE
                    .range(&self.root, (self.partition, recipient.id))
                    .1,
            );

            // Use the smallest retry interval of all batches we process, or `None` if any batch can
            // be retried immediately.
            overall_retry_at = overall_retry_at.min(recipient.retry_at);
        }
        Ok(overall_retry_at)
    }
    async fn move_messages(
        &self,
        range: RangeOption<'_>,
        conv: impl FnMut(&FdbValue) -> Result<Vec<u8>, Error> + Send,
    ) -> Result<bool, Error> {
        self.db
            .transact_boxed(
                (range, conv),
                |tx, (range, conv)| {
                    async move {
                        let mut msg_stream = tx.get_ranges(range.clone(), false);
                        let mut more = false;
                        while let Some(batch) = msg_stream.try_next().await? {
                            more = batch.more();
                            for item in batch {
                                tx.clear(item.key());
                                let new_key = conv(&item)?;
                                tx.set(&new_key, item.value());
                            }
                        }

                        Ok::<_, Error>(more)
                    }
                    .boxed()
                },
                TransactOption::idempotent(),
            )
            .await
    }
    async fn migrate_messages(&self, partition_range_send: PartitionRange) -> Result<(), Error> {
        // Migrate unbatched messages
        let mut partition_message_range: RangeOption = PARTITION_MESSAGE_SPACE
            .range(&self.root, (self.partition,))
            .into();
        partition_message_range.limit = Some(u16::MAX as usize);
        while self
            .move_messages(partition_message_range.clone(), |item| {
                let mut key_parts = PARTITION_MESSAGE_SPACE
                    .decode(&self.root, item.key())
                    .ok_or_else(|| {
                        Error(anyhow!(
                            "Failed to decode message key whilst migrating partitions"
                        ))
                    })?;
                let msg_hdr = postcard::from_bytes::<MessageHeader>(item.value())?;
                let new_partition =
                    partition_for_recipient(msg_hdr.recipient_id, partition_range_send);
                key_parts.0 = new_partition;
                let new_key = PARTITION_MESSAGE_SPACE.key(&self.root, key_parts);
                Ok(new_key)
            })
            .await?
        {}
        // Migrate batched messages
        let mut partition_message_range: RangeOption = PARTITION_BATCH_SPACE
            .range(&self.root, (self.partition,))
            .into();
        partition_message_range.limit = Some(u16::MAX as usize);
        while self
            .move_messages(partition_message_range.clone(), |item| {
                let mut key_parts = PARTITION_BATCH_SPACE
                    .decode(&self.root, item.key())
                    .ok_or_else(|| {
                        Error(anyhow!(
                            "Failed to decode message key whilst migrating partitions"
                        ))
                    })?;
                let new_partition = partition_for_recipient(key_parts.1, partition_range_send);
                key_parts.0 = new_partition;
                let new_key = PARTITION_BATCH_SPACE.key(&self.root, key_parts);
                Ok(new_key)
            })
            .await?
        {}
        Ok(())
    }
    async fn maybe_migrate_messages(&self) -> Result<(), Error> {
        // Load the partition send and receive ranges
        let (partition_range_recv, partition_range_send) = self
            .db
            .transact_boxed(
                &self.root,
                |tx, &mut root| {
                    async move {
                        let partition_range_recv =
                            load_partition_range(tx, root, &PARTITION_COUNT_RECV, true).await?;
                        let partition_range_send =
                            load_partition_range(tx, root, &PARTITION_COUNT_SEND, true).await?;
                        Ok::<_, Error>((partition_range_recv, partition_range_send))
                    }
                    .boxed()
                },
                TransactOption::idempotent(),
            )
            .await?;

        if partition_range_recv != partition_range_send {
            self.migrate_messages(partition_range_send).await?;
        }

        Ok(())
    }
    async fn step(&mut self) -> Result<(), Error> {
        self.maybe_migrate_messages().await?;
        let watch_fut = self.rollup_messages().await?;
        let maybe_retry_at = self.process_batches().await?;

        // If there was nothing to process, sleep until there is a new message
        if let Some(retry_at) = maybe_retry_at {
            let duration = retry_at - Timestamp::now();
            select! {
                _ = tokio::time::timeout(duration, watch_fut).fuse() => {},
                _ = &mut self.cancellation => {},
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
