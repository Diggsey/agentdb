use std::{
    collections::{btree_map::Entry, BTreeMap},
    ops::Range,
    sync::Arc,
};

use byteorder::{ByteOrder, LittleEndian};
use foundationdb::{tuple::Versionstamp, Database, RangeOption, TransactOption, Transaction};
use futures::{stream, FutureExt, Stream, TryStreamExt};
use uuid::Uuid;

use crate::{
    client::{
        ClientValue, PartitionAssignment, PartitionRange, CLIENT_SPACE, PARTITION_COUNT_RECV,
        PARTITION_COUNT_SEND,
    },
    partition::{PARTITION_AGENT_COUNT, PARTITION_BATCH_SPACE, PARTITION_MESSAGE_SPACE},
    utils::load_partition_range,
    Error, MessageHeader, Timestamp,
};

async fn find_next_root(db: &Database, from: &[u8]) -> Result<Option<Vec<u8>>, Error> {
    db.transact_boxed(
        from,
        |tx, &mut from| {
            async move {
                let mut ranges = tx.get_ranges((from..b"\xFF").into(), true);
                while let Some(range) = ranges.try_next().await? {
                    for kv in range {
                        let k = kv.key();
                        if let Some(prefix) = k.strip_suffix(b".agentdb/") {
                            if kv.value() == b"agentdb" {
                                return Ok(Some(prefix.to_vec()));
                            }
                        }
                    }
                }
                Ok(None)
            }
            .boxed()
        },
        TransactOption::idempotent(),
    )
    .await
}

pub fn search_for_roots(db: Arc<Database>) -> impl Stream<Item = Result<Vec<u8>, Error>> {
    stream::try_unfold(Vec::new(), move |from| {
        let db = db.clone();
        async move {
            let next_root = find_next_root(&db, &from).await?;
            Ok(next_root.map(|root| {
                let mut new_from = root.clone();
                // Skip over all keys beginning with a '.'
                new_from.push(b'.' + 1);
                (root, new_from)
            }))
        }
    })
}

#[derive(Debug, Clone)]
pub struct ClientDesc {
    last_active_ts: Timestamp,
    name: String,
    partitions: Range<u32>,
}

impl ClientDesc {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn last_active_ts(&self) -> Timestamp {
        self.last_active_ts
    }
    pub fn partitions(&self) -> Range<u32> {
        self.partitions.clone()
    }
}

#[derive(Debug, Clone)]
pub struct MessageDesc {
    message_id: Uuid,
    recipient_id: Uuid,
    scheduled_for: Option<Timestamp>,
}

impl MessageDesc {
    pub fn message_id(&self) -> Uuid {
        self.message_id
    }
    pub fn recipient_id(&self) -> Uuid {
        self.recipient_id
    }
    pub fn scheduled_for(&self) -> Option<Timestamp> {
        self.scheduled_for
    }
}

#[derive(Debug, Clone)]
pub struct PartitionDesc {
    agent_count: i64,
    pending_messages: Vec<MessageDesc>,
    pending_messages_overflow: bool,
    batched_messages: Vec<MessageDesc>,
    batched_messages_overflow: bool,
}

impl PartitionDesc {
    pub fn agent_count(&self) -> i64 {
        self.agent_count
    }
    pub fn pending_messages(&self) -> &[MessageDesc] {
        &self.pending_messages
    }
    pub fn pending_messages_overflow(&self) -> bool {
        self.pending_messages_overflow
    }
    pub fn batched_messages(&self) -> &[MessageDesc] {
        &self.batched_messages
    }
    pub fn batched_messages_overflow(&self) -> bool {
        self.batched_messages_overflow
    }
}

#[derive(Debug, Clone)]
pub struct RootDesc {
    partition_range_recv: Range<u32>,
    partition_range_send: Range<u32>,
    clients: Vec<ClientDesc>,
    partitions: BTreeMap<u32, PartitionDesc>,
}

impl RootDesc {
    pub fn partition_range_recv(&self) -> Range<u32> {
        self.partition_range_recv.clone()
    }
    pub fn partition_range_send(&self) -> Range<u32> {
        self.partition_range_send.clone()
    }
    pub fn clients(&self) -> &[ClientDesc] {
        &self.clients
    }
    pub fn partitions(&self) -> &BTreeMap<u32, PartitionDesc> {
        &self.partitions
    }
}

async fn describe_clients(
    tx: &Transaction,
    root: &[u8],
    partition_range: PartitionRange,
) -> Result<Vec<ClientDesc>, Error> {
    // Scan for all the active clients
    let client_range = CLIENT_SPACE.range(&root, ()).into();
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
                partition_range: partition_range.clone(),
            }
            .range(),
        })
        .collect();
    Ok(clients)
}

const DESC_LIMIT: usize = 1000;

async fn describe_partition(
    tx: &Transaction,
    root: &[u8],
    partition: u32,
) -> Result<PartitionDesc, Error> {
    let agent_count_key = PARTITION_AGENT_COUNT.key(root, (partition,));
    let agent_count_bytes = tx.get(&agent_count_key, true).await?;
    let agent_count = agent_count_bytes
        .map(|slice| LittleEndian::read_i64(&slice))
        .unwrap_or(0);

    // Load pending messages
    let mut pending_messages_range: RangeOption = PARTITION_MESSAGE_SPACE
        .range::<_, (Timestamp, Versionstamp, u32)>(root, (partition,))
        .into();
    pending_messages_range.limit = Some(DESC_LIMIT);
    let mut pending_message_stream = tx.get_ranges(pending_messages_range, true);
    let mut pending_messages = Vec::new();
    let mut pending_messages_overflow = true;
    while let Some(batch) = pending_message_stream.try_next().await? {
        pending_messages_overflow &= batch.more();
        for item in batch {
            if let Some((_, ts, _, _)) = PARTITION_MESSAGE_SPACE.decode(root, item.key()) {
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
    let mut batched_messages_range: RangeOption = PARTITION_BATCH_SPACE
        .range::<_, (Uuid, Versionstamp)>(root, (partition,))
        .into();
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
        agent_count,
        pending_messages,
        pending_messages_overflow,
        batched_messages,
        batched_messages_overflow,
    })
}

async fn describe_partitions(
    tx: &Transaction,
    root: &[u8],
    partition_range_recv: Range<u32>,
    partition_range_send: Range<u32>,
) -> Result<BTreeMap<u32, PartitionDesc>, Error> {
    let mut result = BTreeMap::new();
    for partition in partition_range_recv.chain(partition_range_send) {
        if let Entry::Vacant(vac) = result.entry(partition) {
            vac.insert(describe_partition(tx, root, partition).await?);
        }
    }
    Ok(result)
}

fn convert_range(range: PartitionRange) -> Range<u32> {
    range.offset..(range.offset + range.count)
}

pub async fn describe_root(db: Arc<Database>, root: &[u8]) -> Result<RootDesc, Error> {
    db.transact_boxed(
        root,
        |tx, &mut root| {
            async move {
                let partition_range_send =
                    load_partition_range(tx, root, &PARTITION_COUNT_SEND).await?;
                let partition_range_recv =
                    load_partition_range(tx, root, &PARTITION_COUNT_RECV).await?;

                let clients = describe_clients(tx, root, partition_range_recv).await?;
                let partitions = describe_partitions(
                    tx,
                    root,
                    convert_range(partition_range_recv),
                    convert_range(partition_range_send),
                )
                .await?;

                Ok(RootDesc {
                    partition_range_send: convert_range(partition_range_send),
                    partition_range_recv: convert_range(partition_range_recv),
                    clients,
                    partitions,
                })
            }
            .boxed()
        },
        TransactOption::idempotent(),
    )
    .await
}
