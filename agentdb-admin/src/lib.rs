use std::{collections::BTreeMap, fmt, future::Future, ops::Range, sync::Arc};

use chrono::{DateTime, Utc};
use foundationdb::{
    api::NetworkAutoStop,
    directory::Directory,
    options::StreamingMode,
    tuple::{Element, Subspace},
    RangeOption, TransactOption,
};
use futures::{stream::TryStreamExt, FutureExt};
use lazy_static::lazy_static;
use rnet::{net, Delegate2, Net, ToNet};
use tokio::runtime::Runtime;

use agentdb_core::{
    admin::{self, describe_root, search_for_roots},
    blob, Error, Global,
};
use uuid::Uuid;

rnet::root!();

struct GlobalRuntime {
    _network: NetworkAutoStop,
    pub runtime: Runtime,
}

impl GlobalRuntime {
    pub fn new() -> Self {
        let _network = unsafe { foundationdb::boot() };
        let runtime = Runtime::new().expect("Failed to start tokio runtime.");
        Self { _network, runtime }
    }
}

lazy_static! {
    static ref GLOBAL: GlobalRuntime = GlobalRuntime::new();
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
    global: Arc<Global>,
}

#[derive(Net)]
pub struct ClientDesc {
    last_active_ts: DateTime<Utc>,
    name: String,
    partitions: Range<u32>,
}

#[derive(Net)]
pub struct NoResult;

impl From<()> for NoResult {
    fn from(_: ()) -> Self {
        Self
    }
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
    pending_messages: Vec<MessageDesc>,
    pending_messages_overflow: bool,
    batched_messages: Vec<MessageDesc>,
    batched_messages_overflow: bool,
}

impl From<admin::PartitionDesc> for PartitionDesc {
    fn from(other: admin::PartitionDesc) -> Self {
        Self {
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
    agent_count: i64,
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
            agent_count: other.agent_count(),
        }
    }
}

#[net]
fn connect(path: Option<String>) -> Result<Arc<Connection>, Error> {
    lazy_static::initialize(&GLOBAL);
    Ok(Arc::new(Connection {
        global: Global::connect(path.as_deref())?,
    }))
}

#[net]
fn list_roots(con: Arc<Connection>, continuation: Continuation<Vec<String>>) {
    wrap_async(continuation, async move {
        search_for_roots(&con.global).try_collect().await
    });
}

#[net]
fn describe_root(con: Arc<Connection>, root: String, continuation: Continuation<RootDesc>) {
    wrap_async(continuation, async move {
        describe_root(&con.global, &root).await.map(Into::into)
    });
}

#[net]
fn load_blob(
    con: Arc<Connection>,
    root: String,
    blob_id: Uuid,
    continuation: Continuation<Option<Vec<u8>>>,
) {
    wrap_async(continuation, async move {
        con.global
            .db()
            .transact_boxed(
                (root, con.global.clone()),
                move |tx, (root, global)| {
                    async move { blob::load(tx, global, &root, blob_id, true).await }.boxed()
                },
                TransactOption::idempotent(),
            )
            .await
    });
}

#[net]
fn change_partitions(
    con: Arc<Connection>,
    root: String,
    partition_range: Range<u32>,
    continuation: Continuation<NoResult>,
) {
    wrap_async(continuation, async move {
        admin::change_partitions(&con.global, &root, partition_range)
            .await
            .map(Into::into)
    });
}

#[net]
fn list_agents(
    con: Arc<Connection>,
    root: String,
    from: Uuid,
    limit: u32,
    reverse: bool,
    continuation: Continuation<Vec<Uuid>>,
) {
    wrap_async(continuation, async move {
        admin::list_agents(&con.global, &root, from, limit as usize, reverse).await
    });
}

#[derive(Net)]
pub struct KeyValueDesc {
    key_bytes: Vec<u8>,
    key_decoded: Vec<String>,
    value_bytes: Vec<u8>,
}

struct HexSlice<'a>(&'a [u8]);

impl<'a> fmt::Display for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &byte in self.0 {
            write!(f, "{:0>2x}", byte)?;
        }
        Ok(())
    }
}

fn element_to_string(element: &Element) -> String {
    match element {
        Element::Nil => "NULL".into(),
        Element::Bytes(b) => HexSlice(b).to_string(),
        Element::String(s) => format!("{:?}", s),
        Element::Tuple(elems) => {
            let parts: Vec<_> = elems.iter().map(element_to_string).collect();
            format!("({})", parts.join(", "))
        }
        Element::Int(v) => v.to_string(),
        Element::Float(v) => v.to_string(),
        Element::Double(v) => v.to_string(),
        Element::Bool(v) => v.to_string(),
        Element::Uuid(v) => v.to_string(),
        Element::Versionstamp(v) => format!("Versionstamp({})", HexSlice(v.as_bytes())),
        Element::BigInt(v) => v.to_string(),
    }
}

impl KeyValueDesc {
    pub fn new(subspace: &Subspace, key: &[u8], value: &[u8]) -> Result<Self, Error> {
        let elems: Vec<Element> = subspace.unpack(key)?;
        Ok(Self {
            key_bytes: key.into(),
            key_decoded: elems.iter().map(element_to_string).collect(),
            value_bytes: value.into(),
        })
    }
}

#[net]
fn list_subspace(
    con: Arc<Connection>,
    prefix: Vec<u8>,
    from: Vec<u8>,
    limit: u32,
    reverse: bool,
    continuation: Continuation<Vec<KeyValueDesc>>,
) {
    wrap_async(continuation, async move {
        con.global
            .db()
            .transact_boxed(
                (from, prefix),
                |tx, (from, prefix)| {
                    async move {
                        let subspace = Subspace::from_bytes(&prefix);
                        let (mut start, mut end) = subspace.range();
                        if reverse {
                            if *from < end {
                                end = from.clone();
                            }
                        } else {
                            if *from > start {
                                start = from.clone();
                                start.push(0);
                            }
                        }
                        let mut range: RangeOption = (start, end).into();
                        range.limit = Some(limit as usize);
                        range.mode = StreamingMode::WantAll;

                        let values = tx.get_range(&range, 0, true).await?;

                        values
                            .into_iter()
                            .map(|v| KeyValueDesc::new(&subspace, v.key(), v.value()))
                            .collect()
                    }
                    .boxed()
                },
                TransactOption::idempotent(),
            )
            .await
    });
}

#[net]
fn list_directory(
    con: Arc<Connection>,
    path: Vec<String>,
    continuation: Continuation<Vec<String>>,
) {
    wrap_async(continuation, async move {
        con.global
            .db()
            .transact_boxed(
                (con.global.clone(), path),
                move |tx, (global, path)| {
                    async move {
                        global
                            .dir()
                            .list(tx, path.clone())
                            .await
                            .map_err(Error::from_dir)
                    }
                    .boxed()
                },
                TransactOption::idempotent(),
            )
            .await
    });
}

#[derive(Net)]
pub struct DirectoryDesc {
    path: Vec<String>,
    prefix: Vec<u8>,
    layer: Vec<u8>,
}

#[net]
fn open_directory(
    con: Arc<Connection>,
    path: Vec<String>,
    continuation: Continuation<DirectoryDesc>,
) {
    wrap_async(continuation, async move {
        con.global
            .db()
            .transact_boxed(
                (con.global.clone(), path),
                move |tx, (global, path)| {
                    async move {
                        let dir = global
                            .dir()
                            .open(tx, path.clone(), None)
                            .await
                            .map_err(Error::from_dir)?;
                        Ok(DirectoryDesc {
                            path: path.clone(),
                            prefix: dir.bytes().into(),
                            layer: dir.get_layer(),
                        })
                    }
                    .boxed()
                },
                TransactOption::idempotent(),
            )
            .await
    });
}
