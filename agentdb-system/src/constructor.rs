use agentdb_core::Error;
use async_trait::async_trait;
use futures::future::BoxFuture;

use crate::agent::{Agent, DynAgent};
use crate::agent_ref::{AgentRef, DynAgentRef};
use crate::context::Context;
use crate::message::Message;
use crate::utils::dynamic_registry;

#[async_trait]
pub trait DynConstruct: Message {
    async fn dyn_construct(
        self,
        ref_: DynAgentRef,
        context: &mut Context<'_>,
    ) -> Result<Option<DynAgent>, Error>;
}

#[async_trait]
pub trait Construct: DynConstruct {
    type Agent: Agent;
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
        let maybe_agent = self.construct(ref_.unchecked_downcast(), context).await?;
        Ok(maybe_agent.map(|agent| Box::new(agent) as _))
    }
}

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

impl<C: Message> Constructor<C>
where
    Self: inventory::Collect,
{
    pub fn new() -> Self
    where
        C: DynConstruct,
    {
        Self {
            construct_fn: |message, ref_, context| C::dyn_construct(message, ref_, context),
        }
    }
    pub async fn call(
        message: C,
        ref_: DynAgentRef,
        context: &mut Context<'_>,
    ) -> Result<Option<DynAgent>, Error>
    where
        Self: inventory::Collect,
    {
        for constructor in inventory::iter::<Self> {
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
