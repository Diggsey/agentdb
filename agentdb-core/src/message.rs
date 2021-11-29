use std::collections::{hash_map, HashMap, HashSet};

use anyhow::anyhow;
use byteorder::{ByteOrder, LittleEndian};
use foundationdb::{options::MutationType, tuple::Versionstamp, Transaction};
use uuid::Uuid;

use crate::{
    blob,
    directories::Global,
    error::Error,
    partition::mark_partition_modified,
    utils::{load_partition_range, partition_for_recipient},
    MessageHeader, OutboundMessage, Timestamp,
};

// Don't let any single operation exceed one message per second over long time scales.
pub(crate) const MS_PER_MSG_PER_OP: i64 = 1000;
// Cannot "burst" more than 1000 messages per operation.
const MAX_MSG_BURST: i64 = 1000;

pub(crate) const INITIAL_TS_OFFSET: i64 = MS_PER_MSG_PER_OP * MAX_MSG_BURST;

/// Send messages into the AgentDB system.
pub async fn send_messages(
    tx: &Transaction,
    global: &Global,
    msgs: &[OutboundMessage],
    user_version: u16,
) -> Result<(), Error> {
    let mut partition_counts = HashMap::new();
    let mut partition_modified = HashSet::new();
    let mut operations = HashMap::<_, i64>::new();

    for (idx, msg) in msgs.iter().enumerate() {
        let recipient_root = global.root(&msg.recipient_root).await?;
        let entry = partition_counts.entry(&msg.recipient_root);
        let partition_range = match entry {
            hash_map::Entry::Occupied(occ) => *occ.get(),
            hash_map::Entry::Vacant(vac) => *vac.insert(
                load_partition_range(tx, &recipient_root.partition_range_send, false).await?,
            ),
        };
        *operations
            .entry((&msg.recipient_root, msg.operation_id))
            .or_default() += 1;

        let msg_id = Uuid::new_v4();
        blob::store_internal(tx, &recipient_root, msg_id, &msg.content);
        let msg_hdr = postcard::to_stdvec(&MessageHeader {
            recipient_id: msg.recipient_id,
            operation_id: msg.operation_id,
            blob_id: msg_id,
        })?;

        let partition_idx = partition_for_recipient(msg.recipient_id, partition_range);
        let partition = recipient_root.partition(global, partition_idx).await?;

        let key =
            partition
                .message
                .pack(&(msg.when, Versionstamp::incomplete(user_version), idx as u32));
        tx.atomic_op(&key, &msg_hdr, MutationType::SetVersionstampedKey);

        // Mark the partition as modified
        if partition_modified.insert((&msg.recipient_root, partition_idx)) {
            mark_partition_modified(tx, &partition);
        }
    }

    // Make sure all the involved operations have sufficient budget to continue
    let current_ts = Timestamp::now().millis();
    let initial_operation_ts = current_ts - INITIAL_TS_OFFSET;
    for ((recipient_root, operation_id), count) in operations {
        let root = global.root(recipient_root).await?;
        let key = root.operation_ts.pack(&operation_id);
        let operation_ts = tx
            .get(&key, true)
            .await?
            .map(|slice| LittleEndian::read_i64(&slice))
            .unwrap_or(initial_operation_ts);
        let allowed_count = (current_ts - operation_ts) / MS_PER_MSG_PER_OP;
        if allowed_count < count {
            return Err(Error(anyhow!(
                "Budget exceeded for operation {}",
                operation_id
            )));
        }

        tx.atomic_op(&key, &initial_operation_ts.to_le_bytes(), MutationType::Max);
        tx.atomic_op(
            &key,
            &(count * MS_PER_MSG_PER_OP).to_le_bytes(),
            MutationType::Add,
        );
    }

    Ok(())
}
