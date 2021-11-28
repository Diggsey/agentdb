use agentdb_core::Error;
use async_trait::async_trait;

use crate::agent::DynAgent;
use crate::agent_ref::DynAgentRef;
use crate::constructor::Constructor;
use crate::context::Context;
use crate::handler::Handler;

/// A message of any type.
pub type DynMessage = Box<dyn Message>;

#[doc(hidden)]
pub async fn deliver_message<M: Message>(
    message: M,
    agent_ref: DynAgentRef,
    maybe_agent_state: &mut Option<DynAgent>,
    context: &mut Context<'_>,
) -> Result<(), Error>
where
    Handler<M>: inventory::Collect,
    Constructor<M>: inventory::Collect,
{
    if let Some(mut agent_state) = maybe_agent_state.take() {
        if Handler::call(&mut *agent_state, agent_ref, message, context).await? {
            agent_state._internal_destruct(agent_ref, context).await?;
        } else {
            *maybe_agent_state = Some(agent_state);
        }
    } else {
        *maybe_agent_state = Constructor::call(message, agent_ref, context).await?;
    }
    Ok(())
}

/// Trait implemented by message types.
#[typetag::serde]
#[async_trait]
pub trait Message: Send + Sync + 'static {
    #[doc(hidden)]
    async fn _internal_deliver(
        self: Box<Self>,
        agent_ref: DynAgentRef,
        maybe_agent_state: &mut Option<DynAgent>,
        context: &mut Context,
    ) -> Result<(), Error>;
}
