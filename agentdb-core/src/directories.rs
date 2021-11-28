use std::{collections::HashMap, sync::Arc};

use foundationdb::{
    directory::{directory_layer::DirectoryLayer, Directory, DirectoryOutput},
    tuple::Versionstamp,
    Database, TransactOption,
};
use futures::FutureExt;
use parking_lot::RwLock;
use uuid::Uuid;

pub const AGENTDB_LAYER: &[u8] = b"agentdb";

use crate::{Error, Timestamp, TypedSubspace};

/// AgentDB must be initialized with a connection to FoundationDB, and
/// a FoundationDB directory layer for storing AgentDB roots.
pub struct Global {
    pub(crate) db: Arc<Database>,
    pub(crate) dir: DirectoryLayer,
    pub(crate) roots: RwLock<HashMap<String, Arc<RootSpace>>>,
}

fn read_rwlock<T, R>(lock: &RwLock<T>, f: impl FnOnce(&T) -> R) -> R {
    f(&lock.read())
}

impl Global {
    /// Construct a global instance with a database connection and custom directory layer.
    pub fn new_with_dir(db: Arc<Database>, dir: DirectoryLayer) -> Arc<Self> {
        Arc::new(Self {
            db,
            dir,
            roots: Default::default(),
        })
    }
    /// Construct a global instance with a database connection and the default directory layer.
    pub fn new(db: Arc<Database>) -> Arc<Self> {
        Self::new_with_dir(db, Default::default())
    }
    /// Construct a global instance by connecting to FoundationDB and using the default directory layer.
    pub fn connect(path: Option<&str>) -> Result<Arc<Self>, Error> {
        Ok(Self::new(Arc::new(Database::new(path)?)))
    }
    /// Construct a global instance by connecting to FoundationDB and using a custom directory layer.
    pub fn connect_with_dir(path: Option<&str>, dir: DirectoryLayer) -> Result<Arc<Self>, Error> {
        Ok(Self::new_with_dir(Arc::new(Database::new(path)?), dir))
    }
    /// Get the database connection used by this instance.
    pub fn db(&self) -> &Arc<Database> {
        &self.db
    }
    /// Get the directory layer used by this instance.
    pub fn dir(&self) -> &DirectoryLayer {
        &self.dir
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
    pub(crate) agent_counts: TypedSubspace<u32>,
    pub(crate) blob_modified: TypedSubspace<Uuid>,
    pub(crate) blob_data: TypedSubspace<(Uuid, u32)>,
    pub(crate) partition_range_send: Vec<u8>,
    pub(crate) partition_range_recv: Vec<u8>,
    pub(crate) partition_dir: DirectoryOutput,
    pub(crate) partitions: RwLock<HashMap<u32, Arc<PartitionSpace>>>,
    pub(crate) operation_ts: TypedSubspace<Uuid>,
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
                        let agent_counts =
                            TypedSubspace::open_or_create(tx, &dir, "agent_counts").await?;
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
                        let operation_ts =
                            TypedSubspace::open_or_create(tx, &dir, "operation_ts").await?;
                        Ok(Self {
                            root: root.into(),
                            user_dir,
                            clients,
                            agents,
                            agent_counts,
                            blob_modified,
                            blob_data,
                            partition_range_send,
                            partition_range_recv,
                            partition_dir,
                            partitions: Default::default(),
                            operation_ts,
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
                            .create_or_open(tx, vec![partition.to_string()], None, None)
                            .await
                            .map_err(Error::from_dir)?;
                        let modified = dir.pack(&"modified".as_bytes());
                        let message = TypedSubspace::open_or_create(tx, &dir, "message").await?;
                        let batch = TypedSubspace::open_or_create(tx, &dir, "batch").await?;
                        let agent_retry =
                            TypedSubspace::open_or_create(tx, &dir, "agent_retry").await?;
                        Ok(Self {
                            partition,
                            modified,
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
