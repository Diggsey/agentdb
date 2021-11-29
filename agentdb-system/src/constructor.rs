use agentdb_core::Error;
use async_trait::async_trait;
use futures::future::BoxFuture;

use crate::agent::{Agent, DynAgent};
use crate::agent_ref::{AgentRef, DynAgentRef};
use crate::context::Context;
use crate::message::Message;
use crate::utils::dynamic_registry;

/// Trait implemented by messages which can be used to construct agents,
/// where the type of the constructed agent is not known.
#[async_trait]
pub trait DynConstruct: Message {
    /// Attempt to construct an agent using this message. Returns `None`
    /// if no agent should be constructed.
    async fn dyn_construct(
        self,
        ref_: DynAgentRef,
        context: &mut Context<'_>,
    ) -> Result<Option<DynAgent>, Error>;
}

/// Trait implemented by messages which can be used to construct agents
/// of a known type.
#[async_trait]
pub trait Construct: DynConstruct {
    /// The type of agent constructed by this message.
    type Agent: Agent;
    /// Attempt to construct an agent using this message. Returns `None`
    /// if no agent should be constructed.
    async fn construct(
        self,
        ref_: AgentRef<Self::Agent>,
        context: &mut Context<'_>,
    ) -> Result<Option<Self::Agent>, Error>;
}

#[async_trait]
impl<M: Construct> DynConstruct for M {
    async fn dyn_construct(
        self,
        ref_: DynAgentRef,
        context: &mut Context<'_>,
    ) -> Result<Option<DynAgent>, Error> {
        if M::Agent::is_frangible() {
            context.require_clearance().await?;
        }
        let maybe_agent = self.construct(ref_.unchecked_downcast(), context).await?;
        Ok(maybe_agent.map(|agent| Box::new(agent) as _))
    }
}

#[doc(hidden)]
pub struct Constructor<C: Message>
where
    Self: inventory::Collect,
{
    construct_fn: for<'a> fn(
        message: C,
        ref_: DynAgentRef,
        context: &'a mut Context,
    ) -> BoxFuture<'a, Result<Option<DynAgent>, Error>>,
}

impl<C: Message + DynConstruct> Default for Constructor<C>
where
    Self: inventory::Collect,
{
    fn default() -> Self {
        Self {
            construct_fn: |message, ref_, context| C::dyn_construct(message, ref_, context),
        }
    }
}

impl<C: Message> Constructor<C>
where
    Self: inventory::Collect,
{
    pub(crate) async fn call(
        message: C,
        ref_: DynAgentRef,
        context: &mut Context<'_>,
    ) -> Result<Option<DynAgent>, Error>
    where
        Self: inventory::Collect,
    {
        if let Some(constructor) = inventory::iter::<Self>.into_iter().next() {
            return (constructor.construct_fn)(message, ref_, context).await;
        }
        Ok(None)
    }
}

impl<M: Message> inventory::Collect for Constructor<M> {
    #[inline]
    fn registry() -> &'static inventory::Registry<Self> {
        dynamic_registry()
    }
}
