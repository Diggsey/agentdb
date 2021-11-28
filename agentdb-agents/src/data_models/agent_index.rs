//! This module defines an agent which manages an "index" of other agents.

use agentdb_system::*;
use foundationdb::{
    options::{MutationType, StreamingMode},
    RangeOption,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Notify;

type IndexSpace = TypedSubspace<(Prepacked, String, Uuid)>;

/// An agent which manages an "index" of other agents. This can later be queried efficiently.
///
/// The index is not unique, meaning that multiple agents may be present with the same key, and
/// the same agent may appear multiple times, even with the same key.
#[agent(name = "adb.data_models.agent_index")]
#[derive(Serialize, Deserialize)]
pub struct AgentIndex {
    count: u64,
    index_space: IndexSpace,
}

impl AgentIndex {
    async fn new(context: &mut Context<'_>) -> Result<Self, Error> {
        Ok(Self {
            count: 0,
            index_space: TypedSubspace::open_or_create(
                context.tx(),
                &context.user_dir().await?,
                "idx",
            )
            .await?,
        })
    }
}

/// Obtain high-level information about the index
#[message(name = "adb.data_models.agent_index.stat")]
#[derive(Serialize, Deserialize)]
pub struct Stat {
    /// The ID of this query - used by the caller to correlate the response.
    pub query_id: Uuid,
    /// The agent which should receive the response.
    pub caller: DynAgentRef,
}

/// Obtain high-level information about the index
#[message(name = "adb.data_models.agent_index.stat_response")]
#[derive(Serialize, Deserialize)]
pub struct StatResponse {
    /// The ID sent in the initial `Stat` query.
    pub query_id: Uuid,
    /// The total number of agents stored in the index.
    pub count: u64,
}

/// An individual "update" operation to the index.
#[derive(Serialize, Deserialize)]
pub enum UpdateOp {
    /// Add a new entry to the index.
    Add {
        /// The key of the new entry.
        key: Prepacked,
        /// The value (agent handle) of the new entry.
        value: DynAgentRef,
    },
    /// Remove an existing entry from the index.
    Remove {
        /// The key of the entry to remove.
        key: Prepacked,
        /// The value (agent handle) of the entry to remove.
        value: DynAgentRef,
    },
}

/// Message to apply one or more updates to the index
#[message(name = "adb.data_models.agent_index.update")]
#[derive(Serialize, Deserialize)]
pub struct Update {
    /// An ordered list of updates to apply.
    pub ops: Vec<UpdateOp>,
    /// Who to notify once the index has been updated.
    pub notify: Notify,
}

impl Update {
    /// Convenience method to construct an update with a single "Add" operation.
    pub fn add(key: Prepacked, value: DynAgentRef) -> Self {
        Self {
            ops: vec![UpdateOp::Add { key, value }],
            notify: Notify::none(),
        }
    }
    /// Convenience method to construct an update with a single "Remove" operation.
    pub fn remove(key: Prepacked, value: DynAgentRef) -> Self {
        Self {
            ops: vec![UpdateOp::Remove { key, value }],
            notify: Notify::none(),
        }
    }
    /// Convenience method to construct an update with a pair of symmetric "Remove"/"Add" operations.
    pub fn update(old_key: Prepacked, new_key: Prepacked, value: DynAgentRef) -> Self {
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

/// Query the index for exact keys. At most one agent will be returned per key.
/// If multiple agents are present with the same key, it is unspecified which
/// one will be returend.
#[message(name = "adb.data_models.agent_index.query_exact")]
#[derive(Serialize, Deserialize)]
pub struct QueryExact {
    /// The ID of this query - used by the caller to correlate the response.
    pub query_id: Uuid,
    /// Where to send the response.
    pub caller: DynAgentRef,
    /// The list of keys to query. Results will be returned in the same order.
    pub keys: Vec<Prepacked>,
}

/// Response from an exact query. Values will be in the same order
/// as keys in the query.
#[message(name = "adb.data_models.agent_index.query_exact_response")]
#[derive(Serialize, Deserialize)]
pub struct QueryExactResponse {
    /// The ID sent in the initial `QueryExact` query.
    pub query_id: Uuid,
    /// The ordered list of query results.
    pub values: Vec<Option<DynAgentRef>>,
}

/// One half of a range query.
#[derive(Serialize, Deserialize)]
pub struct QueryRangeSelector {
    /// The key indicating one end of the range.
    pub key: Prepacked,
    /// Whether agents with this exact key should be included in the response.
    pub inclusive: bool,
}

impl QueryRangeSelector {
    fn index_key(selector: Option<Self>, space: &IndexSpace, is_upper_bound: bool) -> Vec<u8> {
        let ((from, to), use_to) = if let Some(selector) = selector {
            (
                space.nested_range(&(selector.key,)),
                selector.inclusive == is_upper_bound,
            )
        } else {
            (space.range(), is_upper_bound)
        };
        if use_to {
            to
        } else {
            from
        }
    }
}

/// Query the index for a range of keys.
#[message(name = "adb.data_models.agent_index.query_range")]
#[derive(Serialize, Deserialize)]
pub struct QueryRange {
    /// The ID of this query - used by the caller to correlate the response.
    pub query_id: Uuid,
    /// Where to send the response.
    pub caller: DynAgentRef,
    /// The lower bound of the query range, or `None` to indicate no lower bound.
    pub lower: Option<QueryRangeSelector>,
    /// The upper bound of the query range, or `None` to indicate no upper bound.
    pub upper: Option<QueryRangeSelector>,
    /// If `true`, results are returned starting from the upper bound instead of
    /// the lower bound.
    pub reverse: bool,
    /// The maximum number of results to return.
    pub limit: u32,
}

/// Response from a range query.
#[message(name = "adb.data_models.agent_index.query_range_response")]
#[derive(Serialize, Deserialize)]
pub struct QueryRangeResponse {
    /// The ID sent in the initial `QueryRange` query.
    pub query_id: Uuid,
    /// The list of results, ordered according to the initial query.
    pub results: Vec<(Prepacked, DynAgentRef)>,
}

impl AgentIndex {
    fn index_key(&self, key: Prepacked, value: DynAgentRef) -> Vec<u8> {
        self.index_space
            .pack(&(key, value.root().to_string(), value.id()))
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
        let mut state = AgentIndex::new(context).await?;
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
        let mut state = AgentIndex::new(context).await?;
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
        _ref_: AgentRef<Self>,
        mut msg: Update,
        context: &mut Context,
    ) -> Result<bool, Error> {
        let tx = context.tx();
        for op in msg.ops {
            match op {
                UpdateOp::Add { key, value } => {
                    let index_key = self.index_key(key, value);
                    tx.atomic_op(&index_key, &1i64.to_le_bytes(), MutationType::Add);
                    self.count += 1;
                }
                UpdateOp::Remove { key, value } => {
                    let index_key = self.index_key(key, value);
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
        let mut state = AgentIndex::new(context).await?;
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
        _ref_: AgentRef<Self>,
        msg: QueryExact,
        context: &mut Context,
    ) -> Result<bool, Error> {
        let tx = context.tx();
        let mut res = Vec::with_capacity(msg.keys.len());
        for key in msg.keys {
            let mut index_range: RangeOption = self.index_space.nested_range(&(key,)).into();
            index_range.limit = Some(1);
            index_range.mode = StreamingMode::WantAll;
            let values = tx.get_range(&index_range, 0, true).await?;
            res.push(values.get(0).and_then(|v| {
                let (_, agent_root, agent_id) = self.index_space.unpack(v.key()).ok()?;
                Some(DynAgentRef::from_parts(
                    Root::from_name(&agent_root)?,
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
        let mut state = AgentIndex::new(context).await?;
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
        _ref_: AgentRef<Self>,
        msg: QueryRange,
        context: &mut Context,
    ) -> Result<bool, Error> {
        let tx = context.tx();
        let mut index_range: RangeOption =
            (QueryRangeSelector::index_key(msg.lower, &self.index_space, false)
                ..QueryRangeSelector::index_key(msg.upper, &self.index_space, true))
                .into();

        index_range.limit = Some(msg.limit as usize);
        index_range.mode = StreamingMode::WantAll;
        let values = tx.get_range(&index_range, 0, true).await?;
        let res = values
            .into_iter()
            .flat_map(|v| {
                let (key, agent_root, agent_id) = self.index_space.unpack(v.key()).ok()?;
                Some((
                    key,
                    DynAgentRef::from_parts(Root::from_name(&agent_root)?, agent_id),
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
