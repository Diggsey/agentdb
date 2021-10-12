use agentdb_core::Error;
use async_trait::async_trait;
use mopa::mopafy;

use crate::agent_ref::DynAgentRef;
use crate::context::Context;
use crate::destructor::Destructor;
use crate::dynamic_handler::HandlerDyn;
use crate::message::DynMessage;

pub type DynAgent = Box<dyn Agent>;

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

pub async fn handle_dyn<A: Agent>(
    state: &mut A,
    ref_: DynAgentRef,
    message: DynMessage,
    context: &mut Context<'_>,
) -> Result<bool, Error>
where
    HandlerDyn<A>: inventory::Collect,
{
    HandlerDyn::call(state, ref_.unchecked_downcast(), message, context).await
}

#[typetag::serde]
#[async_trait]
pub trait Agent: mopa::Any + Send + Sync {
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
