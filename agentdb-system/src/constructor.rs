use agentdb_core::Error;
use async_trait::async_trait;
use futures::future::BoxFuture;

use crate::agent::DynAgent;
use crate::agent_ref::DynAgentRef;
use crate::context::Context;
use crate::message::Message;
use crate::utils::dynamic_registry;

#[async_trait]
pub trait Construct: Message {
    async fn construct(
        self,
        ref_: DynAgentRef,
        context: &mut Context<'_>,
    ) -> Result<Option<DynAgent>, Error>;
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
        C: Construct,
    {
        Self {
            construct_fn: |message, ref_, context| C::construct(message, ref_, context),
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
