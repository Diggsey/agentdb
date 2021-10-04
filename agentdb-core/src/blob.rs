use std::sync::Arc;

use foundationdb::{
    options::{ConflictRangeType, MutationType},
    Database, TransactOption, Transaction,
};
use futures::{stream, Future, FutureExt, Stream, TryFutureExt, TryStreamExt};
use uuid::Uuid;

use crate::{error::Error, subspace::Subspace};

static BLOB_SPACE: Subspace<(Uuid, u32)> = Subspace::new(b"b");
static BLOB_MODIFIED_SPACE: Subspace<(Uuid,)> = Subspace::new(b"bv");

// Store blobs in blocks of 16kb to avoid hitting value size limits (typically 100kb)
const BLOB_STRIPE_SIZE: usize = 1024 * 16;

pub async fn load(
    tx: &Transaction,
    root: &[u8],
    blob_id: Uuid,
    snapshot: bool,
) -> Result<Option<Vec<u8>>, Error> {
    let range = BLOB_SPACE.range::<_, (u32,)>(root, (blob_id,)).into();

    // Only look at the modified key for potential conflicts, not the entire blob
    if !snapshot {
        let modified_range = BLOB_MODIFIED_SPACE.range::<_, ()>(root, (blob_id,));
        tx.add_conflict_range(
            &modified_range.0,
            &modified_range.1,
            ConflictRangeType::Read,
        )?;
    }

    let mut stream = tx.get_ranges(range, true);
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
    let modified_key = BLOB_MODIFIED_SPACE.key(root, (blob_id,));
    let range = BLOB_SPACE.range::<_, (u32,)>(root, (blob_id,));

    tx.atomic_op(
        &modified_key,
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        MutationType::SetVersionstampedValue,
    );
    tx.clear_range(&range.0, &range.1);
    for (index, chunk) in data.chunks(BLOB_STRIPE_SIZE).enumerate() {
        let key = BLOB_SPACE.key(root, (blob_id, index as u32));
        tx.set(&key, chunk);
    }
}

pub fn delete(tx: &Transaction, root: &[u8], blob_id: Uuid) {
    let modified_key = BLOB_MODIFIED_SPACE.key(root, (blob_id,));
    let range = BLOB_SPACE.range::<_, (u32,)>(root, (blob_id,));
    tx.clear(&modified_key);
    tx.clear_range(&range.0, &range.1);
}

pub fn watch(
    tx: &Transaction,
    root: &[u8],
    blob_id: Uuid,
) -> impl Future<Output = Result<(), Error>> + 'static {
    let modified_key = BLOB_MODIFIED_SPACE.key(root, (blob_id,));
    tx.watch(&modified_key).err_into()
}

pub fn watch_stream(
    db: Arc<Database>,
    root: &[u8],
    blob_id: Uuid,
) -> impl Stream<Item = Result<Option<Vec<u8>>, Error>> + 'static {
    let root = Arc::new(root.to_vec());
    stream::try_unfold(None, move |maybe_fut| {
        let root = root.clone();
        let db = db.clone();
        async move {
            if let Some(fut) = maybe_fut {
                fut.await?;
            }
            Ok::<_, Error>(Some(
                db.transact_boxed_local(
                    root,
                    move |tx, root| {
                        async move {
                            let blob = load(tx, &root, blob_id, true).await?;
                            let fut = watch(tx, &root, blob_id);
                            Ok::<_, Error>((blob, Some(fut)))
                        }
                        .boxed_local()
                    },
                    TransactOption::idempotent(),
                )
                .await?,
            ))
        }
    })
}
