use agentdb_core::Error;
use async_trait::async_trait;
use futures::future::BoxFuture;

use crate::agent::Agent;
use crate::agent_ref::AgentRef;
use crate::context::Context;
use crate::utils::dynamic_registry;

#[async_trait]
pub trait Destruct: Agent + Sized {
    async fn destruct(self, ref_: AgentRef<Self>, context: &mut Context) -> Result<(), Error>;
}

pub struct Destructor<A: Agent>
where
    Self: inventory::Collect,
{
    destruct_fn: for<'a> fn(
        state: A,
        ref_: AgentRef<A>,
        context: &'a mut Context,
    ) -> BoxFuture<'a, Result<(), Error>>,
}

impl<A: Agent> Destructor<A>
where
    Self: inventory::Collect,
{
    pub fn new() -> Self
    where
        A: Destruct,
    {
        Self {
            destruct_fn: |state, ref_, context| A::destruct(state, ref_, context),
        }
    }
    pub async fn call(state: A, ref_: AgentRef<A>, context: &mut Context<'_>) -> Result<(), Error>
    where
        Self: inventory::Collect,
    {
        for destructor in inventory::iter::<Self> {
            return (destructor.destruct_fn)(state, ref_, context).await;
        }
        Ok(())
    }
}

impl<A: Agent> inventory::Collect for Destructor<A> {
    #[inline]
    fn registry() -> &'static inventory::Registry<Self> {
        dynamic_registry()
    }
}
