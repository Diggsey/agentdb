//! Contains functions for reading and writing "blobs", or binary large objects.
//!
//! Agent state and messages are all stored as blobs by AgentDB.

use std::sync::Arc;

use foundationdb::{
    options::{ConflictRangeType, MutationType},
    TransactOption, Transaction,
};
use futures::{stream, Future, FutureExt, Stream, TryFutureExt, TryStreamExt};
use uuid::Uuid;

use crate::{
    directories::{Global, RootSpace},
    error::Error,
    utils::next_key,
};

// Store blobs in blocks of 16kb to avoid hitting value size limits (typically 100kb)
const BLOB_STRIPE_SIZE: usize = 1024 * 16;

pub(crate) async fn load_internal(
    tx: &Transaction,
    root: &RootSpace,
    blob_id: Uuid,
    snapshot: bool,
) -> Result<Option<Vec<u8>>, Error> {
    let range = root.blob_data.nested_range(&(blob_id,)).into();

    // Only look at the modified key for potential conflicts, not the entire blob
    if !snapshot {
        let modified_range = root.blob_modified.pack(&blob_id);
        tx.add_conflict_range(
            &modified_range,
            &next_key(&modified_range),
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

pub(crate) fn store_internal(tx: &Transaction, root: &RootSpace, blob_id: Uuid, data: &[u8]) {
    let modified_key = root.blob_modified.pack(&blob_id);
    let range = root.blob_data.nested_range(&(blob_id,));

    tx.atomic_op(
        &modified_key,
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        MutationType::SetVersionstampedValue,
    );
    tx.clear_range(&range.0, &range.1);
    for (index, chunk) in data.chunks(BLOB_STRIPE_SIZE).enumerate() {
        let key = root.blob_data.pack(&(blob_id, index as u32));
        tx.set(&key, chunk);
    }
}

pub(crate) fn delete_internal(tx: &Transaction, root: &RootSpace, blob_id: Uuid) {
    let modified_key = root.blob_modified.pack(&blob_id);
    let range = root.blob_data.nested_range(&(blob_id,));
    tx.clear(&modified_key);
    tx.clear_range(&range.0, &range.1);
}

pub(crate) fn watch_internal(
    tx: &Transaction,
    root: &RootSpace,
    blob_id: Uuid,
) -> impl Future<Output = Result<(), Error>> + 'static {
    let modified_key = root.blob_modified.pack(&blob_id);
    tx.watch(&modified_key).err_into()
}

pub(crate) fn watch_stream_internal(
    global: Arc<Global>,
    root: Arc<RootSpace>,
    blob_id: Uuid,
) -> impl Stream<Item = Result<Option<Vec<u8>>, Error>> + 'static {
    stream::try_unfold(None, move |maybe_fut| {
        let root = root.clone();
        let global = global.clone();
        async move {
            if let Some(fut) = maybe_fut {
                fut.await?;
            }
            Ok::<_, Error>(Some(
                global
                    .db()
                    .transact_boxed(
                        root,
                        move |tx, root| {
                            async move {
                                let blob = load_internal(tx, root, blob_id, true).await?;
                                let fut = watch_internal(tx, root, blob_id);
                                Ok::<_, Error>((blob, Some(fut)))
                            }
                            .boxed()
                        },
                        TransactOption::idempotent(),
                    )
                    .await?,
            ))
        }
    })
}

/// Load a blob. Returns `None` if the blob does not exist.
pub async fn load(
    tx: &Transaction,
    global: &Global,
    root: &str,
    blob_id: Uuid,
    snapshot: bool,
) -> Result<Option<Vec<u8>>, Error> {
    let root = global.root(root).await?;
    load_internal(tx, &root, blob_id, snapshot).await
}

/// Store a blob. Will overwrite any existing blob with this ID.
pub async fn store(
    tx: &Transaction,
    global: &Global,
    root: &str,
    blob_id: Uuid,
    data: &[u8],
) -> Result<(), Error> {
    let root = global.root(root).await?;
    store_internal(tx, &root, blob_id, data);
    Ok(())
}

/// Delete a blob. Will have no effect if the blob does not exist.
pub async fn delete(
    tx: &Transaction,
    global: &Global,
    root: &str,
    blob_id: Uuid,
) -> Result<(), Error> {
    let root = global.root(root).await?;
    delete_internal(tx, &root, blob_id);
    Ok(())
}

/// Watch for changes to a blob. The returned future will resolve when
/// the blob first changes.
pub async fn watch(
    tx: &Transaction,
    global: &Global,
    root: &str,
    blob_id: Uuid,
) -> Result<impl Future<Output = Result<(), Error>> + 'static, Error> {
    let root = global.root(root).await?;
    Ok(watch_internal(tx, &root, blob_id))
}

/// Watch for multiple changes to a blob. The returned stream will yield
/// an item whenever the blob is changed. If the blob is changed multiple
/// times in quick succession, it is not guaranteed that an item will be
/// returned for every change, but at least one item will be returned after
/// the last change.
pub fn watch_stream(
    global: Arc<Global>,
    root: &str,
    blob_id: Uuid,
) -> impl Stream<Item = Result<Option<Vec<u8>>, Error>> + 'static {
    let root = root.to_owned();
    let global2 = global.clone();
    async move { global2.root(&root).await }
        .map_ok(move |root| watch_stream_internal(global, root, blob_id))
        .try_flatten_stream()
}
