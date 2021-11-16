use std::collections::{hash_map, HashMap, HashSet};

use foundationdb::{options::MutationType, tuple::Versionstamp, Transaction};
use uuid::Uuid;

use crate::{
    blob,
    client::PARTITION_COUNT_SEND,
    error::Error,
    partition::{mark_partition_modified, PARTITION_MESSAGE_SPACE},
    utils::{load_partition_range, partition_for_recipient},
    MessageHeader, MessageToSend,
};

pub async fn send_messages(
    tx: &Transaction,
    msgs: &[MessageToSend],
    user_version: u16,
) -> Result<(), Error> {
    let mut partition_counts = HashMap::new();
    let mut partition_modified = HashSet::new();

    for (idx, msg) in msgs.into_iter().enumerate() {
        let entry = partition_counts.entry(&msg.recipient_root);
        let partition_range = match entry {
            hash_map::Entry::Occupied(occ) => *occ.get(),
            hash_map::Entry::Vacant(vac) => *vac.insert(
                load_partition_range(tx, &msg.recipient_root, &PARTITION_COUNT_SEND, false).await?,
            ),
        };

        let msg_id = Uuid::new_v4();
        blob::store(tx, &msg.recipient_root, msg_id, &msg.content);
        let msg_hdr = postcard::to_stdvec(&MessageHeader {
            recipient_id: msg.recipient_id,
            blob_id: msg_id,
        })?;

        let partition = partition_for_recipient(msg.recipient_id, partition_range);

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

        // Mark the partition as modified
        if partition_modified.insert((&msg.recipient_root, partition)) {
            mark_partition_modified(tx, &msg.recipient_root, partition);
        }
    }

    Ok(())
}
