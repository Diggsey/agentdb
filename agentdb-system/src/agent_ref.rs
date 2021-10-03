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

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct AgentRef<A: Agent> {
    inner: DynAgentRef,
    phantom: PhantomData<fn(A) -> A>,
}

impl<A: Agent> AgentRef<A> {
    pub fn root(self) -> Root {
        self.inner.root
    }
    pub fn id(self) -> Uuid {
        self.inner.id
    }
}
