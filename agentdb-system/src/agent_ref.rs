use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use agentdb_core::{blob, Error, Global};
use anyhow::anyhow;
use foundationdb::{TransactOption, Transaction};
use futures::{Future, FutureExt, Stream, TryStreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::agent::{Agent, DynAgent};
use crate::root::Root;
use crate::serializer::{DefaultSerializer, Serializer};

/// A handle to an agent whose type is unknown.
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DynAgentRef {
    pub(crate) root: Root,
    pub(crate) id: Uuid,
}

impl DynAgentRef {
    /// Directly construct the handle from a root and agent ID.
    pub const fn from_parts(root: Root, id: Uuid) -> Self {
        Self { root, id }
    }

    /// Downcast this handle to a handle of a specific agent type.
    /// If the agent is not actually of this type, the conversion will
    /// silently succeed regardless.
    pub fn unchecked_downcast<A: Agent>(self) -> AgentRef<A> {
        AgentRef {
            inner: self,
            phantom: PhantomData,
        }
    }
    /// Obtain the AgentDB root containing this agent.
    pub fn root(self) -> Root {
        self.root
    }
    /// Obtain the ID of the agent.
    pub fn id(self) -> Uuid {
        self.id
    }
    /// Directly load this agent's state from the database as part of a transaction.
    pub async fn load_tx(
        self,
        global: &Global,
        tx: &Transaction,
        snapshot: bool,
    ) -> Result<Option<DynAgent>, Error> {
        let maybe_blob = blob::load(tx, global, self.root.name(), self.id, snapshot).await?;
        Ok(if let Some(state) = maybe_blob {
            Some(DefaultSerializer.deserialize::<DynAgent>(&state)?)
        } else {
            None
        })
    }
    /// Directly load this agent's state from the database.
    pub async fn load(self, global: &Global) -> Result<Option<DynAgent>, Error> {
        global
            .db()
            .transact_boxed(
                global,
                |tx, global| self.load_tx(global, tx, true).boxed(),
                TransactOption::idempotent(),
            )
            .await
    }
    /// Watch for the first change to this agent's state.
    pub async fn watch(
        self,
        global: &Global,
        tx: &Transaction,
    ) -> Result<impl Future<Output = Result<(), Error>> + 'static, Error> {
        blob::watch(tx, global, self.root.name(), self.id).await
    }
    /// Watch for all changes to this agent's state.
    pub fn watch_stream(
        self,
        global: Arc<Global>,
    ) -> impl Stream<Item = Result<Option<DynAgent>, Error>> + 'static {
        blob::watch_stream(global, self.root.name(), self.id).and_then(|maybe_blob| async move {
            Ok(if let Some(state) = maybe_blob {
                Some(DefaultSerializer.deserialize::<DynAgent>(&state)?)
            } else {
                None
            })
        })
    }
}

impl<A: Agent> From<AgentRef<A>> for DynAgentRef {
    fn from(other: AgentRef<A>) -> Self {
        other.inner
    }
}

/// A handle to an agent of type `A`.
pub struct AgentRef<A> {
    inner: DynAgentRef,
    phantom: PhantomData<Arc<Mutex<A>>>,
}

impl<A: Agent> Copy for AgentRef<A> {}
impl<A: Agent> Clone for AgentRef<A> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<A: Agent> Debug for AgentRef<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AgentRef")
            .field("root", &self.inner.root)
            .field("id", &self.inner.id)
            .finish()
    }
}
impl<A: Agent> Serialize for AgentRef<A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}
impl<'de, A: Agent> Deserialize<'de> for AgentRef<A> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        DynAgentRef::deserialize(deserializer).map(|inner| AgentRef {
            inner,
            phantom: PhantomData,
        })
    }
}

impl<A> AgentRef<A> {
    /// Directly construct this handle from a root and agent ID. The construction will
    /// silently succeed even if no such agent exists, or if the agent is of the wrong
    /// type.
    pub const fn from_parts_unchecked(root: Root, id: Uuid) -> Self {
        Self {
            inner: DynAgentRef::from_parts(root, id),
            phantom: PhantomData,
        }
    }
}

impl<A: Agent> AgentRef<A> {
    /// Obtain the AgentDB root containing this agent.
    pub fn root(self) -> Root {
        self.inner.root
    }
    /// Obtain the ID of this agent.
    pub fn id(self) -> Uuid {
        self.inner.id
    }
    fn downcast_state(maybe_agent: Option<DynAgent>) -> Result<Option<A>, Error> {
        Ok(if let Some(agent) = maybe_agent {
            Some(
                *agent
                    .downcast::<A>()
                    .map_err(|_| Error(anyhow!("Agent had incorrect type")))?,
            )
        } else {
            None
        })
    }
    /// Directly load this agent's state from the database as part of a transaction.
    pub async fn load_tx(
        self,
        global: &Global,
        tx: &Transaction,
        snapshot: bool,
    ) -> Result<Option<A>, Error> {
        Self::downcast_state(self.inner.load_tx(global, tx, snapshot).await?)
    }
    /// Directly load this agent's state from the database.
    pub async fn load(self, global: &Global) -> Result<Option<A>, Error> {
        Self::downcast_state(self.inner.load(global).await?)
    }
    /// Watch for the first change to this agent's state.
    pub async fn watch(
        self,
        global: &Global,
        tx: &Transaction,
    ) -> Result<impl Future<Output = Result<(), Error>> + 'static, Error> {
        self.inner.watch(global, tx).await
    }
    /// Watch for all changes to this agent's state.
    pub fn watch_stream(
        self,
        global: Arc<Global>,
    ) -> impl Stream<Item = Result<Option<A>, Error>> + 'static {
        self.inner
            .watch_stream(global)
            .and_then(|maybe_agent| async move { Self::downcast_state(maybe_agent) })
    }
}
