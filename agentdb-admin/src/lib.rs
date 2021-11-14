use std::{collections::BTreeMap, future::Future, ops::Range, sync::Arc};

use chrono::{DateTime, Utc};
use foundationdb::{api::NetworkAutoStop, Database, TransactOption};
use futures::{stream::TryStreamExt, FutureExt};
use lazy_static::lazy_static;
use rnet::{net, Delegate2, Net, ToNet};
use tokio::runtime::Runtime;

use agentdb_core::{
    admin::{self, describe_root, search_for_roots},
    blob, Error,
};
use uuid::Uuid;

rnet::root!();

struct Global {
    _network: NetworkAutoStop,
    pub runtime: Runtime,
}

impl Global {
    pub fn new() -> Self {
        let _network = unsafe { foundationdb::boot() };
        let runtime = Runtime::new().expect("Failed to start tokio runtime.");
        Self { _network, runtime }
    }
}

lazy_static! {
    static ref GLOBAL: Global = Global::new();
}

type Continuation<A> = Delegate2<(), Option<A>, Option<String>>;

fn wrap_async<A>(
    continuation: Continuation<A>,
    cb: impl Future<Output = Result<A, Error>> + Send + 'static,
) where
    A: ToNet,
{
    GLOBAL.runtime.spawn(async move {
        match cb.await {
            Ok(x) => continuation.call(Some(x), None),
            Err(e) => continuation.call(None, Some(e.to_string())),
        }
    });
}

pub struct Connection {
    db: Arc<Database>,
}

#[derive(Net)]
pub struct ClientDesc {
    last_active_ts: DateTime<Utc>,
    name: String,
    partitions: Range<u32>,
}

impl From<admin::ClientDesc> for ClientDesc {
    fn from(other: admin::ClientDesc) -> Self {
        Self {
            last_active_ts: other.last_active_ts().into(),
            name: other.name().into(),
            partitions: other.partitions(),
        }
    }
}

#[derive(Net)]
pub struct MessageDesc {
    message_id: Uuid,
    recipient_id: Uuid,
    scheduled_for: Option<DateTime<Utc>>,
}

impl From<admin::MessageDesc> for MessageDesc {
    fn from(other: admin::MessageDesc) -> Self {
        Self {
            message_id: other.message_id(),
            recipient_id: other.recipient_id(),
            scheduled_for: other.scheduled_for().map(Into::into),
        }
    }
}

#[derive(Net)]
pub struct PartitionDesc {
    agent_count: i64,
    pending_messages: Vec<MessageDesc>,
    pending_messages_overflow: bool,
    batched_messages: Vec<MessageDesc>,
    batched_messages_overflow: bool,
}

impl From<admin::PartitionDesc> for PartitionDesc {
    fn from(other: admin::PartitionDesc) -> Self {
        Self {
            agent_count: other.agent_count(),
            pending_messages: other
                .pending_messages()
                .iter()
                .cloned()
                .map(Into::into)
                .collect(),
            pending_messages_overflow: other.pending_messages_overflow(),
            batched_messages: other
                .batched_messages()
                .iter()
                .cloned()
                .map(Into::into)
                .collect(),
            batched_messages_overflow: other.batched_messages_overflow(),
        }
    }
}

#[derive(Net)]
pub struct RootDesc {
    partition_range_recv: Range<u32>,
    partition_range_send: Range<u32>,
    clients: Vec<ClientDesc>,
    partitions: BTreeMap<u32, PartitionDesc>,
}

impl From<admin::RootDesc> for RootDesc {
    fn from(other: admin::RootDesc) -> Self {
        Self {
            partition_range_recv: other.partition_range_recv(),
            partition_range_send: other.partition_range_send(),
            clients: other.clients().iter().cloned().map(Into::into).collect(),
            partitions: other
                .partitions()
                .iter()
                .map(|(k, v)| (k.clone(), v.clone().into()))
                .collect(),
        }
    }
}

#[net]
fn connect(path: Option<String>) -> Result<Arc<Connection>, Error> {
    lazy_static::initialize(&GLOBAL);
    Ok(Arc::new(Connection {
        db: Arc::new(Database::new(path.as_deref())?),
    }))
}

#[net]
fn list_roots(con: Arc<Connection>, continuation: Continuation<Vec<Vec<u8>>>) {
    wrap_async(continuation, async move {
        search_for_roots(con.db.clone()).try_collect().await
    });
}

#[net]
fn describe_root(con: Arc<Connection>, root: Vec<u8>, continuation: Continuation<RootDesc>) {
    wrap_async(continuation, async move {
        describe_root(con.db.clone(), &root).await.map(Into::into)
    });
}

#[net]
fn load_blob(
    con: Arc<Connection>,
    root: Vec<u8>,
    blob_id: Uuid,
    continuation: Continuation<Option<Vec<u8>>>,
) {
    wrap_async(continuation, async move {
        con.db
            .transact_boxed(
                root,
                move |tx, root| async move { blob::load(tx, &root, blob_id, true).await }.boxed(),
                TransactOption::idempotent(),
            )
            .await
    });
}
