use agentdb_system::*;
use foundationdb::{
    options::{MutationType, StreamingMode},
    RangeOption,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Notify;

pub static INDEX_SPACE: Subspace<(Uuid, Vec<u8>, Vec<u8>, Uuid)> = USER_SPACE.subspace(b"");

// Effect agent which will automatically retry a callback
#[agent(name = "adb.data_models.agent_index")]
#[derive(Default, Serialize, Deserialize)]
pub struct AgentIndex {
    count: u64,
}

/// Obtain high-level information about the index
#[message(name = "adb.data_models.agent_index.stat")]
#[derive(Serialize, Deserialize)]
pub struct Stat {
    pub query_id: Uuid,
    pub caller: DynAgentRef,
}

/// Obtain high-level information about the index
#[message(name = "adb.data_models.agent_index.stat_response")]
#[derive(Serialize, Deserialize)]
pub struct StatResponse {
    pub query_id: Uuid,
    pub count: u64,
}

#[derive(Serialize, Deserialize)]
pub enum UpdateOp {
    Add { key: Vec<u8>, value: DynAgentRef },
    Remove { key: Vec<u8>, value: DynAgentRef },
}

// Message to update the index
#[message(name = "adb.data_models.agent_index.update")]
#[derive(Serialize, Deserialize)]
pub struct Update {
    pub ops: Vec<UpdateOp>,
    pub notify: Notify,
}

impl Update {
    pub fn add(key: Vec<u8>, value: DynAgentRef) -> Self {
        Self {
            ops: vec![UpdateOp::Add { key, value }],
            notify: Notify::none(),
        }
    }
    pub fn remove(key: Vec<u8>, value: DynAgentRef) -> Self {
        Self {
            ops: vec![UpdateOp::Remove { key, value }],
            notify: Notify::none(),
        }
    }
    pub fn update(old_key: Vec<u8>, new_key: Vec<u8>, value: DynAgentRef) -> Self {
        Self {
            ops: vec![
                UpdateOp::Remove {
                    key: old_key,
                    value,
                },
                UpdateOp::Add {
                    key: new_key,
                    value,
                },
            ],
            notify: Notify::none(),
        }
    }
}

/// Query the index for exact keys. At most one agent will
/// be returned per key.
#[message(name = "adb.data_models.agent_index.query_exact")]
#[derive(Serialize, Deserialize)]
pub struct QueryExact {
    pub query_id: Uuid,
    pub caller: DynAgentRef,
    pub keys: Vec<Vec<u8>>,
}

// Response from an exact query. Values will be in the same order
// as keys in the query.
#[message(name = "adb.data_models.agent_index.query_exact_response")]
#[derive(Serialize, Deserialize)]
pub struct QueryExactResponse {
    pub query_id: Uuid,
    pub values: Vec<Option<DynAgentRef>>,
}

#[derive(Serialize, Deserialize)]
pub struct QueryRangeSelector {
    pub key: Vec<u8>,
    pub inclusive: bool,
}

impl QueryRangeSelector {
    fn index_key(
        selector: Option<Self>,
        ref_: AgentRef<AgentIndex>,
        root: Root,
        is_upper: bool,
    ) -> Vec<u8> {
        if let Some(selector) = selector {
            INDEX_SPACE.build(
                root.as_bytes(),
                (ref_.id(), selector.key),
                is_upper == selector.inclusive,
            )
        } else {
            INDEX_SPACE.build(root.as_bytes(), (ref_.id(),), is_upper)
        }
    }
}

// Query the index for a range of keys.
#[message(name = "adb.data_models.agent_index.query_range")]
#[derive(Serialize, Deserialize)]
pub struct QueryRange {
    pub query_id: Uuid,
    pub caller: DynAgentRef,
    pub lower: Option<QueryRangeSelector>,
    pub upper: Option<QueryRangeSelector>,
    pub reverse: bool,
    pub limit: u32,
}

// Response from a range query.
#[message(name = "adb.data_models.agent_index.query_range_response")]
#[derive(Serialize, Deserialize)]
pub struct QueryRangeResponse {
    pub query_id: Uuid,
    pub results: Vec<(Vec<u8>, DynAgentRef)>,
}

impl AgentIndex {
    fn index_key(
        &self,
        ref_: AgentRef<Self>,
        root: Root,
        key: Vec<u8>,
        value: DynAgentRef,
    ) -> Vec<u8> {
        INDEX_SPACE.key(
            root.as_bytes(),
            (ref_.id(), key, value.root().as_bytes().to_vec(), value.id()),
        )
    }
}

#[constructor]
impl Construct for Stat {
    type Agent = AgentIndex;

    async fn construct(
        self,
        ref_: AgentRef<AgentIndex>,
        context: &mut Context<'_>,
    ) -> Result<Option<AgentIndex>, Error> {
        let mut state = AgentIndex::default();
        Ok(if state.handle(ref_, self, context).await? {
            None
        } else {
            Some(state)
        })
    }
}

#[handler]
impl Handle<Stat> for AgentIndex {
    async fn handle(
        &mut self,
        _ref: AgentRef<Self>,
        msg: Stat,
        context: &mut Context,
    ) -> Result<bool, Error> {
        context.dyn_send(
            msg.caller,
            Box::new(StatResponse {
                query_id: msg.query_id,
                count: self.count,
            }),
        )?;
        Ok(self.count == 0)
    }
}

#[constructor]
impl Construct for Update {
    type Agent = AgentIndex;

    async fn construct(
        self,
        ref_: AgentRef<AgentIndex>,
        context: &mut Context<'_>,
    ) -> Result<Option<AgentIndex>, Error> {
        let mut state = AgentIndex::default();
        Ok(if state.handle(ref_, self, context).await? {
            None
        } else {
            Some(state)
        })
    }
}

#[handler]
impl Handle<Update> for AgentIndex {
    async fn handle(
        &mut self,
        ref_: AgentRef<Self>,
        mut msg: Update,
        context: &mut Context,
    ) -> Result<bool, Error> {
        let root = context.root();
        let tx = context.tx();
        for op in msg.ops {
            match op {
                UpdateOp::Add { key, value } => {
                    let index_key = self.index_key(ref_, root, key, value);
                    tx.atomic_op(&index_key, &1i64.to_le_bytes(), MutationType::Add);
                    self.count += 1;
                }
                UpdateOp::Remove { key, value } => {
                    let index_key = self.index_key(ref_, root, key, value);
                    tx.atomic_op(&index_key, &(-1i64).to_le_bytes(), MutationType::Add);
                    tx.atomic_op(
                        &index_key,
                        &0i64.to_le_bytes(),
                        MutationType::CompareAndClear,
                    );
                    self.count -= 1;
                }
            }
        }
        msg.notify.notify(context)?;
        Ok(self.count == 0)
    }
}

#[constructor]
impl Construct for QueryExact {
    type Agent = AgentIndex;

    async fn construct(
        self,
        ref_: AgentRef<AgentIndex>,
        context: &mut Context<'_>,
    ) -> Result<Option<AgentIndex>, Error> {
        let mut state = AgentIndex::default();
        Ok(if state.handle(ref_, self, context).await? {
            None
        } else {
            Some(state)
        })
    }
}

#[handler]
impl Handle<QueryExact> for AgentIndex {
    async fn handle(
        &mut self,
        ref_: AgentRef<Self>,
        msg: QueryExact,
        context: &mut Context,
    ) -> Result<bool, Error> {
        let root = context.root();
        let tx = context.tx();
        let mut res = Vec::with_capacity(msg.keys.len());
        for key in msg.keys {
            let mut index_range: RangeOption =
                INDEX_SPACE.range(root.as_bytes(), (ref_.id(), key)).into();
            index_range.limit = Some(1);
            index_range.mode = StreamingMode::WantAll;
            let values = tx.get_range(&index_range, 0, true).await?;
            res.push(values.get(0).and_then(|v| {
                let (_, _, agent_root, agent_id) = INDEX_SPACE.decode(root.as_bytes(), v.key())?;
                Some(DynAgentRef::from_parts(
                    Root::from_bytes(&agent_root)?,
                    agent_id,
                ))
            }));
        }
        context.dyn_send(
            msg.caller,
            Box::new(QueryExactResponse {
                query_id: msg.query_id,
                values: res,
            }),
        )?;
        Ok(self.count == 0)
    }
}

#[constructor]
impl Construct for QueryRange {
    type Agent = AgentIndex;

    async fn construct(
        self,
        ref_: AgentRef<AgentIndex>,
        context: &mut Context<'_>,
    ) -> Result<Option<AgentIndex>, Error> {
        let mut state = AgentIndex::default();
        Ok(if state.handle(ref_, self, context).await? {
            None
        } else {
            Some(state)
        })
    }
}

#[handler]
impl Handle<QueryRange> for AgentIndex {
    async fn handle(
        &mut self,
        ref_: AgentRef<Self>,
        msg: QueryRange,
        context: &mut Context,
    ) -> Result<bool, Error> {
        let root = context.root();
        let tx = context.tx();
        let mut index_range: RangeOption =
            (QueryRangeSelector::index_key(msg.lower, ref_, root, false)
                ..QueryRangeSelector::index_key(msg.upper, ref_, root, true))
                .into();

        index_range.limit = Some(msg.limit as usize);
        index_range.mode = StreamingMode::WantAll;
        let values = tx.get_range(&index_range, 0, true).await?;
        let res = values
            .into_iter()
            .flat_map(|v| {
                let (_, key, agent_root, agent_id) =
                    INDEX_SPACE.decode(root.as_bytes(), v.key())?;
                Some((
                    key,
                    DynAgentRef::from_parts(Root::from_bytes(&agent_root)?, agent_id),
                ))
            })
            .collect();

        context.dyn_send(
            msg.caller,
            Box::new(QueryRangeResponse {
                query_id: msg.query_id,
                results: res,
            }),
        )?;
        Ok(self.count == 0)
    }
}
