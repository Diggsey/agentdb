use agentdb_core::Error;
use async_trait::async_trait;
use mopa::mopafy;

use crate::agent_ref::DynAgentRef;
use crate::context::Context;
use crate::destructor::Destructor;

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

#[typetag::serde]
#[async_trait]
pub trait Agent: mopa::Any + Send + Sync {
    #[doc(hidden)]
    async fn _internal_destruct(
        self: Box<Self>,
        ref_: DynAgentRef,
        context: &mut Context,
    ) -> Result<(), Error>;
}

mopafy!(Agent);
