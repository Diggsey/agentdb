use agentdb_core::Error;
use async_trait::async_trait;
use downcast_rs::{impl_downcast, DowncastSync};
use serde::{Deserialize, Serialize};

use crate::agent_ref::DynAgentRef;
use crate::context::Context;
use crate::destructor::Destructor;
use crate::dynamic_handler::HandlerDyn;
use crate::message::DynMessage;
use crate::serializer::{DefaultSerializer, Serializer};

/// An agent of any type.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynAgent(pub(crate) Vec<u8>);

impl DynAgent {
    /// Attempt to downcast this message to a concrete type
    pub fn downcast<A: Agent>(self) -> Result<Box<A>, Self> {
        if let Ok(a) = DefaultSerializer.deserialize::<Box<dyn Agent>>(&self.0) {
            if let Ok(a) = a.downcast() {
                return Ok(a);
            }
        }
        Err(self)
    }
    pub(crate) fn deserialize(&self) -> Result<Box<dyn Agent>, Error> {
        DefaultSerializer.deserialize::<Box<dyn Agent>>(&self.0)
    }
}

impl<A: Agent> From<A> for DynAgent {
    fn from(a: A) -> Self {
        Self(
            DefaultSerializer
                .serialize(&a as &dyn Agent)
                .expect("Infallible serialization"),
        )
    }
}

impl From<&dyn Agent> for DynAgent {
    fn from(a: &dyn Agent) -> Self {
        Self(
            DefaultSerializer
                .serialize(a)
                .expect("Infallible serialization"),
        )
    }
}

impl From<Box<dyn Agent>> for DynAgent {
    fn from(a: Box<dyn Agent>) -> Self {
        (*a).into()
    }
}

#[doc(hidden)]
pub async fn destruct_agent<A: Agent>(
    state: A,
    ref_: DynAgentRef,
    context: &mut Context<'_>,
) -> Result<(), Error>
where
    Destructor<A>: inventory::Collect,
{
    Destructor::call(state, ref_.unchecked_downcast(), context).await
}

#[doc(hidden)]
pub async fn handle_dyn<A: Agent>(
    state: &mut A,
    ref_: DynAgentRef,
    message: DynMessage,
    context: &mut Context<'_>,
) -> Result<bool, Error>
where
    HandlerDyn<A>: inventory::Collect,
{
    if A::is_frangible() {
        context.require_clearance().await?;
    }
    HandlerDyn::call(state, ref_.unchecked_downcast(), message, context).await
}

/// The trait implemented by agent types using the `#[agent(...)]` attribute macro.
#[typetag::serde]
#[async_trait]
pub trait Agent: DowncastSync {
    /// Returns `true` if this agent can "stall" without impacting the overall availability
    /// of the system. In the event that an operation's work limit is exceeded, the operation
    /// will prefer to stall at a frangible agent if possible.
    ///
    /// You should mark as many agents as possible as "frangible" using the `#[agent(..., frangible)]`
    /// attribute, so that failures do not cascade throughout the entire system.
    fn is_frangible() -> bool
    where
        Self: Sized,
    {
        false
    }
    #[doc(hidden)]
    async fn _internal_destruct(
        self: Box<Self>,
        ref_: DynAgentRef,
        context: &mut Context,
    ) -> Result<(), Error>;
    #[doc(hidden)]
    async fn _internal_handle_dyn(
        &mut self,
        ref_: DynAgentRef,
        message: DynMessage,
        context: &mut Context,
    ) -> Result<bool, Error>;
}

impl_downcast!(sync Agent);
