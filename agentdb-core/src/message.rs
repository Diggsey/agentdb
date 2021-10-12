use std::collections::{hash_map, HashMap, HashSet};

use byteorder::{ByteOrder, LittleEndian};
use foundationdb::{options::MutationType, tuple::Versionstamp, Transaction};
use uuid::Uuid;

use crate::{
    blob,
    client::PARTITION_COUNT_SEND,
    error::Error,
    partition::{PARTITION_MESSAGE_SPACE, PARTITION_MODIFIED},
    MessageHeader, MessageToSend, DEFAULT_PARTITION_COUNT,
};

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
    let mut partition_modified = HashSet::new();

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
