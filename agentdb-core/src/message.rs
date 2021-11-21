use std::collections::{hash_map, HashMap, HashSet};

use foundationdb::{options::MutationType, tuple::Versionstamp, Transaction};
use uuid::Uuid;

use crate::{
    blob,
    directories::Global,
    error::Error,
    partition::mark_partition_modified,
    utils::{load_partition_range, partition_for_recipient},
    MessageHeader, MessageToSend,
};

pub async fn send_messages(
    tx: &Transaction,
    global: &Global,
    msgs: &[MessageToSend],
    user_version: u16,
) -> Result<(), Error> {
    let mut partition_counts = HashMap::new();
    let mut partition_modified = HashSet::new();

    for (idx, msg) in msgs.into_iter().enumerate() {
        let recipient_root = global.root(&msg.recipient_root).await?;
        let entry = partition_counts.entry(&msg.recipient_root);
        let partition_range = match entry {
            hash_map::Entry::Occupied(occ) => *occ.get(),
            hash_map::Entry::Vacant(vac) => *vac.insert(
                load_partition_range(tx, &recipient_root.partition_range_send, false).await?,
            ),
        };

        let msg_id = Uuid::new_v4();
        blob::store_internal(tx, &recipient_root, msg_id, &msg.content);
        let msg_hdr = postcard::to_stdvec(&MessageHeader {
            recipient_id: msg.recipient_id,
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

    Ok(())
}
