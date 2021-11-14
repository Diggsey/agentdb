use std::collections::{hash_map, HashMap, HashSet};

use foundationdb::{options::MutationType, tuple::Versionstamp, Transaction};
use uuid::Uuid;

use crate::{
    blob,
    client::{PartitionRange, PARTITION_COUNT_SEND},
    error::Error,
    partition::{PARTITION_MESSAGE_SPACE, PARTITION_MODIFIED},
    utils::load_partition_range,
    MessageHeader, MessageToSend,
};

fn partition_for_recipient(recipient_id: Uuid, partition_range: PartitionRange) -> u32 {
    let hash = recipient_id.as_u128();
    let hash = ((hash >> 64) ^ hash) as u64;
    let hash = ((hash >> 32) ^ hash) as u32;
    (hash % partition_range.count) + partition_range.offset
}

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
                load_partition_range(tx, &msg.recipient_root, &PARTITION_COUNT_SEND).await?,
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
            let modified_key = PARTITION_MODIFIED.key(&msg.recipient_root, (partition,));
            tx.atomic_op(
                &modified_key,
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                MutationType::SetVersionstampedValue,
            );
        }
    }

    Ok(())
}
