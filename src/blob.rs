use foundationdb::Transaction;
use futures::TryStreamExt;
use uuid::Uuid;

use crate::{subspace::Subspace, Error};

static BLOB_SPACE: Subspace<(Uuid, u32)> = Subspace::new(b"b");

// Store blobs in stripes of 16kb to avoid hitting value size limits (typically 100kb)
const BLOB_STRIPE_SIZE: usize = 1024 * 16;

pub async fn load(
    tx: &Transaction,
    root: &[u8],
    blob_id: Uuid,
    snapshot: bool,
) -> Result<Option<Vec<u8>>, Error> {
    let range = BLOB_SPACE.range::<_, (u32,)>(root, (blob_id,)).into();
    let mut stream = tx.get_ranges(range, snapshot);
    let mut res = Vec::new();
    let mut exists = false;
    while let Some(values) = stream.try_next().await? {
        for value in values {
            exists = true;
            res.extend_from_slice(value.value());
        }
    }
    if exists {
        Ok(Some(res))
    } else {
        Ok(None)
    }
}

pub fn store(tx: &Transaction, root: &[u8], blob_id: Uuid, data: &[u8]) {
    delete(tx, root, blob_id);
    for (index, chunk) in data.chunks(BLOB_STRIPE_SIZE).enumerate() {
        let key = BLOB_SPACE.key(root, (blob_id, index as u32));
        tx.set(&key, chunk);
    }
}

pub fn delete(tx: &Transaction, root: &[u8], blob_id: Uuid) {
    let range = BLOB_SPACE.range::<_, (u32,)>(root, (blob_id,));
    tx.clear_range(&range.0, &range.1);
}
