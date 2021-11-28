use agentdb_core::Error;
use async_trait::async_trait;
use mopa::mopafy;

use crate::agent_ref::DynAgentRef;
use crate::context::Context;
use crate::destructor::Destructor;
use crate::dynamic_handler::HandlerDyn;
use crate::message::DynMessage;

/// An agent of any type.
pub type DynAgent = Box<dyn Agent>;

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
pub trait Agent: mopa::Any + Send + Sync {
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

mopafy!(Agent);
