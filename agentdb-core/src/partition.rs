use std::{future::Future, sync::Arc, time::Duration};

use anyhow::anyhow;
use foundationdb::{
    directory::Directory, options::MutationType, tuple::Versionstamp, KeySelector, RangeOption,
    TransactOption, Transaction,
};
use futures::{future::FusedFuture, select, FutureExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    blob,
    cancellation::Cancellation,
    client::PartitionRange,
    directories::{Global, PartitionSpace, RootSpace},
    error::Error,
    message::send_messages,
    utils::{
        get_first_in_range, load_partition_range, load_value, move_entries,
        partition_for_recipient, save_value, Timestamp,
    },
    HookContext, InboundMessage, MessageHeader, StateFn, StateFnInput, MAX_BATCH_SIZE,
};

const MAX_POLL_INTERVAL_SECS: u64 = 120;
const MAX_AGENT_COUNTS: u32 = 256;

#[derive(Debug, thiserror::Error)]
#[error("State function returned an error")]
struct StateFnError;

struct PartitionState {
    global: Arc<Global>,
    root: Arc<RootSpace>,
    partition: Arc<PartitionSpace>,
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

pub(crate) fn mark_partition_modified(tx: &Transaction, partition: &PartitionSpace) {
    tx.atomic_op(
        &partition.modified,
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        MutationType::SetVersionstampedValue,
    );
}

impl PartitionState {
    pub fn new(
        global: Arc<Global>,
        root: Arc<RootSpace>,
        partition: Arc<PartitionSpace>,
        state_fn: StateFn,
        cancellation: Cancellation,
    ) -> Self {
        Self {
            global,
            root,
            partition,
            cancellation,
            state_fn,
        }
    }
    async fn rollup_messages(&mut self) -> Result<impl Future + FusedFuture, Error> {
        // Roll up all the messages in the partition into batches, and get back a future that
        // will resolve when either a new message is added, or a scheduled message becomes ready.
        self.global
            .db()
            .transact_boxed(
                &self.partition,
                |tx, &mut partition| {
                    async move {
                        let ts = Timestamp::now();
                        let mut past_message_range: RangeOption =
                            partition.message.nested_range2(&(), &(ts,)).into();
                        past_message_range.limit = Some(u16::MAX as usize);
                        let future_message_range: RangeOption =
                            partition.message.nested_range2(&(ts,), &()).into();

                        // Find all messages which are ready to be received
                        let mut msg_stream = tx.get_ranges(past_message_range.clone(), true);

                        // Group the messages by recipient
                        let mut msg_index = 0;
                        while let Some(msgs) = msg_stream.try_next().await? {
                            for msg in msgs {
                                // Decode the message header
                                let msg_hdr: MessageHeader = postcard::from_bytes(msg.value())?;

                                // Figure out where the message should be batched.
                                let batch_key = partition.batch.pack(&(
                                    msg_hdr.recipient_id,
                                    Versionstamp::incomplete(msg_index),
                                ));
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
                            partition.partition
                        );

                        // Find out how long to wait for the next scheduled message.
                        // Or just wait two minutes if there's no scheduled message.
                        let mut delay = Duration::from_secs(MAX_POLL_INTERVAL_SECS);
                        if let Some(msg) =
                            get_first_in_range(tx, future_message_range.clone(), true).await?
                        {
                            if let Ok((next_ts, _, _)) = partition.message.unpack(msg.key()) {
                                delay = next_ts - ts;
                            }
                        }

                        Ok::<_, Error>(
                            tokio::time::timeout(delay, tx.watch(&partition.modified)).fuse(),
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
        self.global
            .db()
            .transact_boxed(
                (&self.partition, batch_range),
                |tx, &mut (partition, ref batch_range)| {
                    async move {
                        let (recipient_id, _) = if let Some(msg) =
                            get_first_in_range(tx, batch_range.clone(), false).await?
                        {
                            partition.batch.unpack(msg.key())?
                        } else {
                            return Ok(None);
                        };

                        let retry_at_key = partition.agent_retry.pack(&recipient_id);
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
            .global
            .db()
            .transact_boxed(
                (
                    &self.global,
                    &self.root,
                    &self.partition,
                    &self.state_fn,
                    max_batch_size,
                ),
                |tx, &mut (global, root, partition, state_fn, ref mut max_batch_size)| {
                    async move {
                        // Automatically reduce batch size on failure
                        if *max_batch_size > 1 {
                            *max_batch_size >>= 1;
                        }

                        // Load the initial agent state
                        let recipient_state =
                            blob::load_internal(tx, root, recipient.id, false).await?;

                        // Determine the range of keys where messages are batched
                        let mut recipient_range: RangeOption =
                            partition.batch.nested_range(&(recipient.id,)).into();
                        recipient_range.limit = Some(*max_batch_size);

                        // Load and clear all the message IDs
                        let mut all_msg_hdrs = Vec::new();
                        let mut msg_stream = tx.get_ranges(recipient_range, false);
                        while let Some(msgs) = msg_stream.try_next().await? {
                            for msg in msgs {
                                // Decode the message header
                                let msg_hdr: MessageHeader = postcard::from_bytes(msg.value())?;
                                tx.clear(msg.key());
                                all_msg_hdrs.push(msg_hdr);
                            }
                        }

                        // Load all the message contents
                        let mut all_msgs = Vec::with_capacity(all_msg_hdrs.len());
                        for msg_hdr in all_msg_hdrs {
                            let inbound_msg = InboundMessage {
                                operation_id: msg_hdr.operation_id,
                                data: blob::load_internal(tx, root, msg_hdr.blob_id, true)
                                    .await?
                                    .ok_or_else(|| {
                                        Error(anyhow!("Blob not found: {}", msg_hdr.blob_id))
                                    })?,
                            };
                            all_msgs.push(inbound_msg);
                            blob::delete_internal(tx, root, msg_hdr.blob_id);
                        }

                        log::info!(
                            "Loaded {} message(s) in partition {}",
                            all_msgs.len(),
                            partition.partition
                        );

                        let state_fn_input = StateFnInput {
                            user_dir: &root.user_dir,
                            operation_ts: &root.operation_ts,
                            root: &root.root,
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
                            blob::store_internal(tx, root, recipient.id, &state);
                        } else {
                            blob::delete_internal(tx, root, recipient.id);

                            // Also clean up anything this agent stored in its user directory
                            root.user_dir
                                .remove_if_exists(tx, vec![recipient.id.to_string()])
                                .await
                                .map_err(Error::from_dir)?;
                        }

                        send_messages(tx, global, &state_fn_output.messages, 0).await?;

                        // Clear the "retry_at" flag from this agent
                        tx.clear(&partition.agent_retry.pack(&recipient.id));

                        // If agent was created or destroyed
                        if exist_before != exist_after {
                            let agent_key = root.agents.pack(&recipient.id);
                            let agent_count_delta: i64 = if exist_after {
                                tx.set(&agent_key, &[]);
                                1
                            } else {
                                tx.clear(&agent_key);
                                -1
                            };
                            // Update partition agent count
                            let agent_count_key = root
                                .agent_counts
                                .pack(&(partition.partition % MAX_AGENT_COUNTS));
                            tx.atomic_op(
                                &agent_count_key,
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
                    global: self.global.clone(),
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
        let mut batch_range: RangeOption = self.partition.batch.range().into();

        // If no messages found, retry after the maximum interval
        let mut overall_retry_at =
            Some(Timestamp::now() + Duration::from_secs(MAX_POLL_INTERVAL_SECS));

        while let Some(recipient) = self.process_batch(batch_range.clone()).await? {
            // If we found and processed a batch, advance our range to exclude that agent
            batch_range.begin = KeySelector::first_greater_or_equal(
                self.partition.batch.nested_range(&(recipient.id,)).1,
            );

            // Use the smallest retry interval of all batches we process, or `None` if any batch can
            // be retried immediately.
            overall_retry_at = overall_retry_at.min(recipient.retry_at);
        }
        Ok(overall_retry_at)
    }
    async fn migrate_messages(&self, partition_range_send: PartitionRange) -> Result<(), Error> {
        log::info!(
            "Migrating messages from partition {} to new partitions",
            self.partition.partition
        );
        // Migrate unbatched messages
        let mut partition_message_range: RangeOption = self.partition.message.range().into();
        partition_message_range.limit = Some(u16::MAX as usize);

        while move_entries(
            self.global.db(),
            self,
            partition_message_range.clone(),
            |item, this| {
                async move {
                    let key_parts = this.partition.message.unpack(item.key())?;
                    let msg_hdr = postcard::from_bytes::<MessageHeader>(item.value())?;
                    let new_partition_idx =
                        partition_for_recipient(msg_hdr.recipient_id, partition_range_send);
                    let new_partition =
                        this.root.partition(&this.global, new_partition_idx).await?;
                    let new_key = new_partition.message.pack(&key_parts);
                    Ok(new_key)
                }
                .boxed()
            },
        )
        .await?
        {}

        // Migrate batched messages
        let mut partition_batch_range: RangeOption = self.partition.batch.range().into();
        partition_batch_range.limit = Some(u16::MAX as usize);

        while move_entries(
            self.global.db(),
            self,
            partition_batch_range.clone(),
            |item, this| {
                async move {
                    let key_parts = this.partition.batch.unpack(item.key())?;
                    let new_partition_idx =
                        partition_for_recipient(key_parts.0, partition_range_send);
                    let new_partition =
                        this.root.partition(&this.global, new_partition_idx).await?;
                    let new_key = new_partition.batch.pack(&key_parts);
                    Ok(new_key)
                }
                .boxed()
            },
        )
        .await?
        {}

        Ok(())
    }
    async fn maybe_migrate_messages(&self) -> Result<(), Error> {
        // Load the partition send and receive ranges
        let (partition_range_recv, partition_range_send) = self
            .global
            .db()
            .transact_boxed(
                &self.root,
                |tx, &mut root| {
                    async move {
                        let partition_range_recv =
                            load_partition_range(tx, &root.partition_range_recv, true).await?;
                        let partition_range_send =
                            load_partition_range(tx, &root.partition_range_send, true).await?;
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

pub(crate) async fn partition_task_inner(
    global: Arc<Global>,
    root: Arc<RootSpace>,
    partition: u32,
    state_fn: StateFn,
    cancellation: Cancellation,
) -> Result<(), Error> {
    let partition = root.partition(&global, partition).await?;
    let partition_state = PartitionState::new(
        global,
        root,
        partition,
        state_fn.clone(),
        cancellation.clone(),
    );
    partition_state.run().await
}

pub(crate) async fn partition_task(
    global: Arc<Global>,
    root: Arc<RootSpace>,
    partition: u32,
    state_fn: StateFn,
    cancellation: Cancellation,
) {
    log::info!("Starting partition {}", partition);
    while !cancellation.is_cancelled() {
        if let Err(e) = partition_task_inner(
            global.clone(),
            root.clone(),
            partition,
            state_fn.clone(),
            cancellation.clone(),
        )
        .await
        {
            log::error!("Failed to run partition {}: {:?}", partition, e);
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
    log::info!("Stopping partition {}", partition);
}
