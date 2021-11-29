//! The admin module contains functions for performing various administrative
//! tasks, such as locating AgentDB roots within a FoundationDB cluster, obtaining
//! statistical information about an AgentDB root, etc.

use std::{
    collections::{btree_map::Entry, BTreeMap},
    ops::Range,
    sync::Arc,
    time::Duration,
};

use anyhow::anyhow;
use byteorder::{ByteOrder, LittleEndian};
use foundationdb::{
    directory::Directory, options::StreamingMode, RangeOption, TransactOption, Transaction,
};
use futures::{stream, FutureExt, Stream, StreamExt, TryFutureExt, TryStreamExt};
use uuid::Uuid;

use crate::{
    client::{ClientValue, PartitionAssignment, PartitionRange},
    directories::{Global, PartitionSpace, RootSpace, AGENTDB_LAYER},
    partition::mark_partition_modified,
    utils::{load_partition_range, range_is_empty, save_value},
    Error, MessageHeader, Timestamp,
};

/// Look for AgentDB roots present in the provided database.
pub fn search_for_roots(global: &Arc<Global>) -> impl Stream<Item = Result<String, Error>> + '_ {
    global
        .db()
        .transact_boxed(
            global.clone(),
            |tx, global| {
                async move {
                    let global = global.clone();
                    let names = global
                        .dir
                        .list(tx, Vec::new())
                        .await
                        .map_err(Error::from_dir)?;
                    Ok(stream::iter(names).then(move |name| {
                        let global = global.clone();
                        async move {
                            let dir = global
                                .db()
                                .transact_boxed(
                                    (name.clone(), global.clone()),
                                    |tx, (name, global)| {
                                        global
                                            .dir
                                            .open(tx, vec![name.clone()], None)
                                            .map_err(Error::from_dir)
                                            .boxed()
                                    },
                                    TransactOption::idempotent(),
                                )
                                .await?;
                            Ok((name, dir.get_layer() == AGENTDB_LAYER))
                        }
                    }))
                }
                .boxed()
            },
            TransactOption::idempotent(),
        )
        .try_flatten_stream()
        .try_filter(|&(_, is_agentdb_root)| async move { is_agentdb_root })
        .map_ok(|(name, _)| name)
}

/// Information about an AgentDB client.
#[derive(Debug, Clone)]
pub struct ClientDesc {
    last_active_ts: Timestamp,
    name: String,
    partitions: Range<u32>,
}

impl ClientDesc {
    /// The self-determined name of the client.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// The timestamp when this client last sent a heartbeat.
    pub fn last_active_ts(&self) -> Timestamp {
        self.last_active_ts
    }
    /// The range of partitions owned by this client.
    pub fn partitions(&self) -> Range<u32> {
        self.partitions.clone()
    }
}

/// Information about a single in-flight message.
#[derive(Debug, Clone)]
pub struct MessageDesc {
    message_id: Uuid,
    recipient_id: Uuid,
    scheduled_for: Option<Timestamp>,
}

impl MessageDesc {
    /// The ID of the message.
    pub fn message_id(&self) -> Uuid {
        self.message_id
    }
    /// The ID of the receiving agent.
    pub fn recipient_id(&self) -> Uuid {
        self.recipient_id
    }
    /// The time when this message is scheduled to be delivered,
    /// or `None` if the message should be delivered immediately.
    pub fn scheduled_for(&self) -> Option<Timestamp> {
        self.scheduled_for
    }
}

/// Information about a partition within an AgentDB root.
#[derive(Debug, Clone)]
pub struct PartitionDesc {
    pending_messages: Vec<MessageDesc>,
    pending_messages_overflow: bool,
    batched_messages: Vec<MessageDesc>,
    batched_messages_overflow: bool,
}

impl PartitionDesc {
    /// A list of the first N pending messages within this partition. These
    /// are messages which have been sent to agents within the partition, but
    /// have not yet been routed to the appropriate agent. This can include
    /// messages which are scheduled to be delivered in the future.
    pub fn pending_messages(&self) -> &[MessageDesc] {
        &self.pending_messages
    }
    /// Returns `true` if the `pending_messages` list was cut off because there
    /// were too many messages to return.
    pub fn pending_messages_overflow(&self) -> bool {
        self.pending_messages_overflow
    }
    /// A list of the first N batched messages within this partition. These
    /// are messages which have already been routed to the correct agent, but
    /// have not yet been processed. Only messages which are ready to be processed
    /// will appear in this list.
    pub fn batched_messages(&self) -> &[MessageDesc] {
        &self.batched_messages
    }
    /// Returns `true` if the `batched_messages` list was cut off because there
    /// were too many messages to return.
    pub fn batched_messages_overflow(&self) -> bool {
        self.batched_messages_overflow
    }
}

/// Information about an AgentDB root.
#[derive(Debug, Clone)]
pub struct RootDesc {
    partition_range_recv: Range<u32>,
    partition_range_send: Range<u32>,
    clients: Vec<ClientDesc>,
    partitions: BTreeMap<u32, PartitionDesc>,
    agent_count: i64,
}

impl RootDesc {
    /// The range of partitions for clients to use when receiving messages. This
    /// will be the same as `partition_range_send` during normal operation, but
    /// will lag behind when a re-partition operation is in progress, to ensure that
    /// all messages from the old partitions are processed or moved before attempting
    /// to process messages from the new partitions.
    pub fn partition_range_recv(&self) -> Range<u32> {
        self.partition_range_recv.clone()
    }
    /// The range of partitions to use when sending messages to agents within this
    /// root.
    pub fn partition_range_send(&self) -> Range<u32> {
        self.partition_range_send.clone()
    }
    /// The list of clients currently connected to this root.
    pub fn clients(&self) -> &[ClientDesc] {
        &self.clients
    }
    /// A mapping from partition index to partition, containing all partitions
    /// within this root.
    pub fn partitions(&self) -> &BTreeMap<u32, PartitionDesc> {
        &self.partitions
    }
    /// The total number of agents stored within this root.
    pub fn agent_count(&self) -> i64 {
        self.agent_count
    }
}

async fn describe_clients(
    tx: &Transaction,
    root: &RootSpace,
    partition_range: PartitionRange,
) -> Result<Vec<ClientDesc>, Error> {
    // Scan for all the active clients
    let client_range = root.clients.range().into();
    let mut kv_stream = tx.get_ranges(client_range, true);
    let mut clients = Vec::new();
    while let Some(kvs) = kv_stream.try_next().await? {
        for kv in kvs {
            if let Ok(client_value) = postcard::from_bytes::<ClientValue>(kv.value()) {
                clients.push(client_value);
            }
        }
    }
    let client_count = clients.len() as u32;
    let clients = clients
        .into_iter()
        .enumerate()
        .map(|(i, client_value)| ClientDesc {
            name: client_value.name,
            last_active_ts: client_value.last_active_ts,
            partitions: PartitionAssignment {
                client_count,
                client_index: i as u32,
                partition_range,
            }
            .range(),
        })
        .collect();
    Ok(clients)
}

const DESC_LIMIT: usize = 1000;

async fn describe_partition(
    tx: &Transaction,
    partition: &PartitionSpace,
) -> Result<PartitionDesc, Error> {
    // Load pending messages
    let mut pending_messages_range: RangeOption = partition.message.range().into();
    pending_messages_range.limit = Some(DESC_LIMIT);
    let mut pending_message_stream = tx.get_ranges(pending_messages_range, true);
    let mut pending_messages = Vec::new();
    let mut pending_messages_overflow = true;
    while let Some(batch) = pending_message_stream.try_next().await? {
        pending_messages_overflow &= batch.more();
        for item in batch {
            if let Ok((ts, _, _)) = partition.message.unpack(item.key()) {
                if let Ok(msg_hdr) = postcard::from_bytes::<MessageHeader>(item.value()) {
                    pending_messages.push(MessageDesc {
                        message_id: msg_hdr.blob_id,
                        recipient_id: msg_hdr.recipient_id,
                        scheduled_for: if ts == Timestamp::zero() {
                            None
                        } else {
                            Some(ts)
                        },
                    });
                }
            }
        }
    }

    // Load batched messages
    let mut batched_messages_range: RangeOption = partition.batch.range().into();
    batched_messages_range.limit = Some(DESC_LIMIT);
    let mut batched_message_stream = tx.get_ranges(batched_messages_range, true);
    let mut batched_messages = Vec::new();
    let mut batched_messages_overflow = true;
    while let Some(batch) = batched_message_stream.try_next().await? {
        batched_messages_overflow &= batch.more();
        for item in batch {
            if let Ok(msg_hdr) = postcard::from_bytes::<MessageHeader>(item.value()) {
                batched_messages.push(MessageDesc {
                    message_id: msg_hdr.blob_id,
                    recipient_id: msg_hdr.recipient_id,
                    scheduled_for: None,
                });
            }
        }
    }

    Ok(PartitionDesc {
        pending_messages,
        pending_messages_overflow,
        batched_messages,
        batched_messages_overflow,
    })
}

async fn describe_partitions(
    tx: &Transaction,
    global: &Global,
    root: &RootSpace,
    partition_range_recv: Range<u32>,
    partition_range_send: Range<u32>,
) -> Result<BTreeMap<u32, PartitionDesc>, Error> {
    let mut result = BTreeMap::new();
    for partition_idx in partition_range_recv.chain(partition_range_send) {
        if let Entry::Vacant(vac) = result.entry(partition_idx) {
            let partition = root.partition(global, partition_idx).await?;
            vac.insert(describe_partition(tx, &partition).await?);
        }
    }
    Ok(result)
}

fn convert_range(range: PartitionRange) -> Range<u32> {
    range.offset..(range.offset + range.count)
}

async fn calculate_agent_count(tx: &Transaction, root: &RootSpace) -> Result<i64, Error> {
    let agent_count_range: RangeOption = root.agent_counts.range().into();
    let mut stream = tx.get_ranges(agent_count_range, true);
    let mut agent_count = 0;
    while let Some(item) = stream.try_next().await? {
        for value in item {
            agent_count += LittleEndian::read_i64(value.value());
        }
    }
    Ok(agent_count)
}

/// Obtain information about a given root.
pub async fn describe_root(global: &Global, root: &str) -> Result<RootDesc, Error> {
    let root = global.root(root).await?;
    global
        .db()
        .transact_boxed(
            (global, &root),
            |tx, &mut (global, root)| {
                async move {
                    let partition_range_send =
                        load_partition_range(tx, &root.partition_range_send, true).await?;
                    let partition_range_recv =
                        load_partition_range(tx, &root.partition_range_recv, true).await?;

                    let clients = describe_clients(tx, root, partition_range_recv).await?;
                    let partitions = describe_partitions(
                        tx,
                        global,
                        root,
                        convert_range(partition_range_recv),
                        convert_range(partition_range_send),
                    )
                    .await?;

                    let agent_count = calculate_agent_count(tx, root).await?;

                    Ok(RootDesc {
                        partition_range_send: convert_range(partition_range_send),
                        partition_range_recv: convert_range(partition_range_recv),
                        clients,
                        partitions,
                        agent_count,
                    })
                }
                .boxed()
            },
            TransactOption::idempotent(),
        )
        .await
}

async fn wait_for_empty_partitions(
    global: &Global,
    root: &RootSpace,
    partition_range: PartitionRange,
) -> Result<(), Error> {
    while !global
        .db()
        .transact_boxed(
            (global, root),
            |tx, &mut (global, root)| {
                async move {
                    for partition_idx in
                        partition_range.offset..(partition_range.offset + partition_range.count)
                    {
                        let partition = root.partition(global, partition_idx).await?;
                        let message_range: RangeOption = partition.message.range().into();
                        let batch_range: RangeOption = partition.batch.range().into();
                        if !range_is_empty(tx, message_range, true).await?
                            || !range_is_empty(tx, batch_range, true).await?
                        {
                            return Ok(false);
                        }
                    }

                    Ok::<_, Error>(true)
                }
                .boxed()
            },
            TransactOption::idempotent(),
        )
        .await?
    {
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    Ok(())
}

/// Change the number of partitions within a root. If this operation is interrupted,
/// it should be retried with the same arguments, or else it will return an error.
pub async fn change_partitions(
    global: &Global,
    root: &str,
    desired_partition_range: Range<u32>,
) -> Result<(), Error> {
    let root = global.root(root).await?;
    let desired_partition_range = PartitionRange {
        offset: desired_partition_range.start,
        count: desired_partition_range.end - desired_partition_range.start,
    };
    if let Some(old_partition_range) = global
        .db()
        .transact_boxed(
            (global, &root),
            |tx, &mut (global, root)| {
                async move {
                    let partition_range_recv =
                        load_partition_range(tx, &root.partition_range_recv, false).await?;
                    let partition_range_send =
                        load_partition_range(tx, &root.partition_range_send, false).await?;

                    if partition_range_recv == partition_range_send {
                        // No partition change operation in progress
                        if partition_range_recv == desired_partition_range {
                            // Already complete...
                            return Ok(None);
                        }
                        // Begin a new partition change operation
                        save_value(tx, &root.partition_range_send, &desired_partition_range);

                        // Wake up all the old partitions
                        for partition_idx in partition_range_recv.offset
                            ..(partition_range_recv.offset + partition_range_recv.count)
                        {
                            let partition = root.partition(global, partition_idx).await?;
                            mark_partition_modified(tx, &partition);
                        }
                    } else {
                        // Partition change already in progress
                        if partition_range_send != desired_partition_range {
                            return Err(Error(anyhow!(
                            "Partition change operation already in progress with different target"
                        )));
                        }
                    }

                    Ok::<_, Error>(Some(partition_range_recv))
                }
                .boxed()
            },
            TransactOption::idempotent(),
        )
        .await?
    {
        // Wait for all messages to be migrated away from the old partitions
        wait_for_empty_partitions(global, &root, old_partition_range).await?;

        // Allow clients to begin processing from the new partitions
        global
            .db()
            .transact_boxed(
                &root,
                |tx, &mut root| {
                    async move {
                        let partition_range_recv =
                            load_partition_range(tx, &root.partition_range_recv, false).await?;
                        if partition_range_recv == old_partition_range {
                            save_value(tx, &root.partition_range_recv, &desired_partition_range);
                        }
                        Ok::<_, Error>(())
                    }
                    .boxed()
                },
                TransactOption::idempotent(),
            )
            .await?;
    }
    Ok(())
}

/// List the agents within a given root starting from the provided ID.
pub async fn list_agents(
    global: &Global,
    root: &str,
    from: Uuid,
    limit: usize,
    reverse: bool,
) -> Result<Vec<Uuid>, Error> {
    let root = global.root(root).await?;
    global
        .db()
        .transact_boxed(
            &root,
            |tx, &mut root| {
                async move {
                    let mut range: RangeOption = if reverse {
                        root.agents.subrange(..=from)
                    } else {
                        root.agents.subrange(from..)
                    }
                    .into();
                    range.limit = Some(limit);
                    range.mode = StreamingMode::WantAll;
                    range.reverse = reverse;
                    let values = tx.get_range(&range, 0, true).await?;
                    Ok(values
                        .into_iter()
                        .flat_map(|value| root.agents.unpack(value.key()))
                        .collect())
                }
                .boxed()
            },
            TransactOption::idempotent(),
        )
        .await
}
