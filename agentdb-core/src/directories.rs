use std::{
    collections::HashMap,
    marker::PhantomData,
    ops::{Bound, RangeBounds},
    sync::Arc,
};

use foundationdb::{
    directory::{directory_layer::DirectoryLayer, Directory, DirectoryOutput},
    tuple::{Subspace, TuplePack, TupleUnpack, Versionstamp},
    Database, TransactOption, Transaction,
};
use futures::FutureExt;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const AGENTDB_LAYER: &[u8] = b"agentdb";

use crate::{subspace::HasPrefix, utils::next_key, Error, Timestamp};

pub struct TypedSubspace<T> {
    inner: Subspace,
    phantom: PhantomData<T>,
}

impl<T> Serialize for TypedSubspace<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.bytes().serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for TypedSubspace<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        Ok(Self {
            inner: Subspace::from_bytes(&bytes),
            phantom: PhantomData,
        })
    }
}

impl<T> TypedSubspace<T> {
    pub async fn open_or_create(
        tx: &Transaction,
        dir: &(dyn Directory + Send + Sync),
        name: &str,
    ) -> Result<Self, Error> {
        let subdir = dir
            .create_or_open(tx, vec![name.into()], None, None)
            .await
            .map_err(Error::from_dir)?;
        Ok(Self {
            inner: Subspace::from_bytes(subdir.bytes()),
            phantom: PhantomData,
        })
    }
    pub fn range(&self) -> (Vec<u8>, Vec<u8>) {
        self.inner.range()
    }
}

fn advance_tuple_key(key: &mut [u8]) {
    // Replace the NIL terminator with 0xFF.
    *key.last_mut().unwrap() = 0xFF;
}

impl<T: TuplePack> TypedSubspace<T> {
    pub fn pack(&self, t: &T) -> Vec<u8> {
        self.inner.pack(t)
    }
    pub fn subrange(&self, r: impl RangeBounds<T>) -> (Vec<u8>, Vec<u8>) {
        let a = match r.start_bound() {
            Bound::Included(x) => self.inner.pack(x),
            Bound::Excluded(x) => next_key(&self.inner.pack(x)),
            Bound::Unbounded => self.inner.range().0,
        };
        let b = match r.end_bound() {
            Bound::Included(y) => next_key(&self.inner.pack(y)),
            Bound::Excluded(y) => self.inner.pack(y),
            Bound::Unbounded => self.inner.range().1,
        };
        (a, b)
    }
    pub fn nested_range<U: TuplePack>(&self, t: &U) -> (Vec<u8>, Vec<u8>)
    where
        T: HasPrefix<U>,
    {
        let a = self.inner.pack(t);
        let mut b = a.clone();
        advance_tuple_key(&mut b);
        (a, b)
    }
    pub fn nested_range2<U: TuplePack, V: TuplePack>(&self, x: &U, y: &V) -> (Vec<u8>, Vec<u8>)
    where
        T: HasPrefix<U> + HasPrefix<V>,
    {
        let a = self.inner.pack(x);
        let mut b = self.inner.pack(y);
        advance_tuple_key(&mut b);
        (a, b)
    }
}

impl<'de, T: TupleUnpack<'de>> TypedSubspace<T> {
    pub fn unpack(&self, key: &'de [u8]) -> Result<T, Error> {
        self.inner.unpack(key).map_err(Into::into)
    }
}

// AgentDB must be initialized with a directory layer and a database connection
pub struct Global {
    pub(crate) db: Arc<Database>,
    pub(crate) dir: DirectoryLayer,
    pub(crate) roots: RwLock<HashMap<String, Arc<RootSpace>>>,
}

fn read_rwlock<T, R>(lock: &RwLock<T>, f: impl FnOnce(&T) -> R) -> R {
    f(&lock.read())
}

impl Global {
    pub fn new_with_dir(db: Arc<Database>, dir: DirectoryLayer) -> Arc<Self> {
        Arc::new(Self {
            db,
            dir,
            roots: Default::default(),
        })
    }
    pub fn new(db: Arc<Database>) -> Arc<Self> {
        Self::new_with_dir(db, Default::default())
    }
    pub fn connect(path: Option<&str>) -> Result<Arc<Self>, Error> {
        Ok(Self::new(Arc::new(Database::new(path)?)))
    }
    pub fn connect_with_dir(path: Option<&str>, dir: DirectoryLayer) -> Result<Arc<Self>, Error> {
        Ok(Self::new_with_dir(Arc::new(Database::new(path)?), dir))
    }
    pub fn db(&self) -> &Arc<Database> {
        &self.db
    }
    pub(crate) async fn root(&self, root: &str) -> Result<Arc<RootSpace>, Error> {
        Ok(
            if let Some(root_space) = read_rwlock(&self.roots, |roots| roots.get(root).cloned()) {
                root_space
            } else {
                let root_space = Arc::new(RootSpace::new(self, root).await?);
                let mut roots = self.roots.write();
                roots.entry(root.into()).or_insert(root_space).clone()
            },
        )
    }
}

pub(crate) struct RootSpace {
    pub(crate) root: String,
    pub(crate) user_dir: DirectoryOutput,
    pub(crate) clients: TypedSubspace<Uuid>,
    pub(crate) agents: TypedSubspace<Uuid>,
    pub(crate) blob_modified: TypedSubspace<Uuid>,
    pub(crate) blob_data: TypedSubspace<(Uuid, u32)>,
    pub(crate) partition_range_send: Vec<u8>,
    pub(crate) partition_range_recv: Vec<u8>,
    pub(crate) partition_dir: DirectoryOutput,
    pub(crate) partitions: RwLock<HashMap<u32, Arc<PartitionSpace>>>,
}

impl RootSpace {
    async fn new(parent: &Global, root: &str) -> Result<Self, Error> {
        parent
            .db
            .transact_boxed(
                (parent, root),
                |tx, &mut (parent, root)| {
                    async move {
                        let dir = parent
                            .dir
                            .create_or_open(tx, vec![root.into()], None, Some(AGENTDB_LAYER.into()))
                            .await
                            .map_err(Error::from_dir)?;
                        let user_dir = dir
                            .create_or_open(tx, vec!["user".into()], None, None)
                            .await
                            .map_err(Error::from_dir)?;
                        let clients = TypedSubspace::open_or_create(tx, &dir, "clients").await?;
                        let agents = TypedSubspace::open_or_create(tx, &dir, "agents").await?;
                        let blob_modified =
                            TypedSubspace::open_or_create(tx, &dir, "blob_modified").await?;
                        let blob_data =
                            TypedSubspace::open_or_create(tx, &dir, "blob_data").await?;
                        let partition_range_send = dir.pack(&"partition_range_send".as_bytes());
                        let partition_range_recv = dir.pack(&"partition_range_recv".as_bytes());
                        let partition_dir = dir
                            .create_or_open(tx, vec!["partition".into()], None, None)
                            .await
                            .map_err(Error::from_dir)?;
                        Ok(Self {
                            root: root.into(),
                            user_dir,
                            clients,
                            agents,
                            blob_modified,
                            blob_data,
                            partition_range_send,
                            partition_range_recv,
                            partition_dir,
                            partitions: Default::default(),
                        })
                    }
                    .boxed()
                },
                TransactOption::idempotent(),
            )
            .await
    }
    pub async fn partition(
        &self,
        global: &Global,
        partition: u32,
    ) -> Result<Arc<PartitionSpace>, Error> {
        Ok(
            if let Some(partition_space) = read_rwlock(&self.partitions, |partitions| {
                partitions.get(&partition).cloned()
            }) {
                partition_space
            } else {
                let partition_space = Arc::new(PartitionSpace::new(self, global, partition).await?);
                let mut partitions = self.partitions.write();
                partitions
                    .entry(partition)
                    .or_insert(partition_space)
                    .clone()
            },
        )
    }
}

pub(crate) struct PartitionSpace {
    pub(crate) partition: u32,
    pub(crate) modified: Vec<u8>,
    pub(crate) agent_count: Vec<u8>,
    pub(crate) message: TypedSubspace<(Timestamp, Versionstamp, u32)>,
    pub(crate) batch: TypedSubspace<(Uuid, Versionstamp)>,
    pub(crate) agent_retry: TypedSubspace<Uuid>,
}

impl PartitionSpace {
    async fn new(parent: &RootSpace, global: &Global, partition: u32) -> Result<Self, Error> {
        global
            .db
            .transact_boxed(
                (parent, partition),
                |tx, &mut (parent, partition)| {
                    async move {
                        let dir = parent
                            .partition_dir
                            .create_or_open(
                                tx,
                                vec![partition.to_string()],
                                None,
                                Some(AGENTDB_LAYER.into()),
                            )
                            .await
                            .map_err(Error::from_dir)?;
                        let modified = dir.pack(&"modified".as_bytes());
                        let agent_count = dir.pack(&"agent_count".as_bytes());
                        let message = TypedSubspace::open_or_create(tx, &dir, "message").await?;
                        let batch = TypedSubspace::open_or_create(tx, &dir, "batch").await?;
                        let agent_retry =
                            TypedSubspace::open_or_create(tx, &dir, "agent_retry").await?;
                        Ok(Self {
                            partition,
                            modified,
                            agent_count,
                            message,
                            batch,
                            agent_retry,
                        })
                    }
                    .boxed()
                },
                TransactOption::idempotent(),
            )
            .await
    }
}
