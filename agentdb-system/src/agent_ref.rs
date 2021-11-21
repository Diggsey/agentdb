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

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DynAgentRef {
    pub(crate) root: Root,
    pub(crate) id: Uuid,
}

impl DynAgentRef {
    pub const fn from_parts(root: Root, id: Uuid) -> Self {
        Self { root, id }
    }
    pub fn unchecked_downcast<A: Agent>(self) -> AgentRef<A> {
        AgentRef {
            inner: self,
            phantom: PhantomData,
        }
    }
    pub fn root(self) -> Root {
        self.root
    }
    pub fn id(self) -> Uuid {
        self.id
    }
    pub async fn load_tx(
        self,
        global: &Global,
        tx: &Transaction,
        snapshot: bool,
    ) -> Result<Option<DynAgent>, Error> {
        let maybe_blob = blob::load(tx, global, self.root.as_str(), self.id, snapshot).await?;
        Ok(if let Some(state) = maybe_blob {
            Some(DefaultSerializer.deserialize::<DynAgent>(&state)?)
        } else {
            None
        })
    }
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
    pub async fn watch(
        self,
        global: &Global,
        tx: &Transaction,
    ) -> Result<impl Future<Output = Result<(), Error>> + 'static, Error> {
        blob::watch(tx, global, self.root.as_str(), self.id).await
    }
    pub fn watch_stream(
        self,
        global: Arc<Global>,
    ) -> impl Stream<Item = Result<Option<DynAgent>, Error>> + 'static {
        blob::watch_stream(global, self.root.as_str(), self.id).and_then(|maybe_blob| async move {
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

pub struct AgentRef<A> {
    inner: DynAgentRef,
    phantom: PhantomData<Arc<Mutex<A>>>,
}

impl<A: Agent> Copy for AgentRef<A> {}
impl<A: Agent> Clone for AgentRef<A> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            phantom: self.phantom.clone(),
        }
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
    pub const fn from_parts_unchecked(root: Root, id: Uuid) -> Self {
        Self {
            inner: DynAgentRef::from_parts(root, id),
            phantom: PhantomData,
        }
    }
}

impl<A: Agent> AgentRef<A> {
    pub fn root(self) -> Root {
        self.inner.root
    }
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
    pub async fn load_tx(
        self,
        global: &Global,
        tx: &Transaction,
        snapshot: bool,
    ) -> Result<Option<A>, Error> {
        Self::downcast_state(self.inner.load_tx(global, tx, snapshot).await?)
    }
    pub async fn load(self, global: &Global) -> Result<Option<A>, Error> {
        Self::downcast_state(self.inner.load(global).await?)
    }
    pub async fn watch(
        self,
        global: &Global,
        tx: &Transaction,
    ) -> Result<impl Future<Output = Result<(), Error>> + 'static, Error> {
        self.inner.watch(global, tx).await
    }
    pub fn watch_stream(
        self,
        global: Arc<Global>,
    ) -> impl Stream<Item = Result<Option<A>, Error>> + 'static {
        self.inner
            .watch_stream(global)
            .and_then(|maybe_agent| async move { Self::downcast_state(maybe_agent) })
    }
}
