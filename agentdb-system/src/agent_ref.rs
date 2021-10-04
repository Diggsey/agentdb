use std::fmt::Debug;
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::agent::Agent;
use crate::root::Root;

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
}
