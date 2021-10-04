use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::Arc;

use agentdb_core::{blob, Error};
use anyhow::anyhow;
use foundationdb::{Database, TransactOption, Transaction};
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
    pub async fn load(self, tx: &Transaction, snapshot: bool) -> Result<Option<DynAgent>, Error> {
        let maybe_blob = blob::load(tx, self.root.as_bytes(), self.id, snapshot).await?;
        Ok(if let Some(state) = maybe_blob {
            Some(DefaultSerializer.deserialize::<DynAgent>(&state)?)
        } else {
            None
        })
    }
    pub async fn load_with_db(self, db: &Database) -> Result<Option<DynAgent>, Error> {
        db.transact_boxed_local(
            (),
            |tx, ()| self.load(tx, true).boxed_local(),
            TransactOption::idempotent(),
        )
        .await
    }
    pub fn watch(self, tx: &Transaction) -> impl Future<Output = Result<(), Error>> + 'static {
        blob::watch(tx, self.root.as_bytes(), self.id)
    }
    pub fn watch_stream(
        self,
        db: Arc<Database>,
    ) -> impl Stream<Item = Result<Option<DynAgent>, Error>> + 'static {
        blob::watch_stream(db, self.root.as_bytes(), self.id).and_then(|maybe_blob| async move {
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

pub struct AgentRef<A: Agent> {
    inner: DynAgentRef,
    phantom: PhantomData<fn(A) -> A>,
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
    pub async fn load(self, tx: &Transaction, snapshot: bool) -> Result<Option<A>, Error> {
        Self::downcast_state(self.inner.load(tx, snapshot).await?)
    }
    pub async fn load_with_db(self, db: &Database) -> Result<Option<A>, Error> {
        Self::downcast_state(self.inner.load_with_db(db).await?)
    }
    pub fn watch(self, tx: &Transaction) -> impl Future<Output = Result<(), Error>> + 'static {
        self.inner.watch(tx)
    }
    pub fn watch_stream(
        self,
        db: Arc<Database>,
    ) -> impl Stream<Item = Result<Option<A>, Error>> + 'static {
        self.inner
            .watch_stream(db)
            .and_then(|maybe_agent| async move { Self::downcast_state(maybe_agent) })
    }
}
