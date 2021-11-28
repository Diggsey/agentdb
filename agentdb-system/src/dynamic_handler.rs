use agentdb_core::Error;
use anyhow::anyhow;
use async_trait::async_trait;
use futures::future::BoxFuture;

use crate::agent::Agent;
use crate::agent_ref::AgentRef;
use crate::context::Context;
use crate::message::DynMessage;
use crate::utils::dynamic_registry;

/// Trait implemented by agents to handle messages of
/// unknown type.
#[async_trait]
pub trait HandleDyn: Agent + Sized {
    /// Handle the provided message. Returns `true` to indicate
    /// that the agent should terminate.
    async fn handle_dyn(
        &mut self,
        ref_: AgentRef<Self>,
        message: DynMessage,
        context: &mut Context,
    ) -> Result<bool, Error>;
}

#[doc(hidden)]
pub struct HandlerDyn<A: Agent>
where
    Self: inventory::Collect,
{
    handle_dyn_fn: for<'a> fn(
        state: &'a mut A,
        ref_: AgentRef<A>,
        message: DynMessage,
        context: &'a mut Context,
    ) -> BoxFuture<'a, Result<bool, Error>>,
}

impl<A: Agent> HandlerDyn<A>
where
    Self: inventory::Collect,
{
    #[doc(hidden)]
    pub fn new() -> Self
    where
        A: HandleDyn,
    {
        Self {
            handle_dyn_fn: |state, ref_, message, context| {
                A::handle_dyn(state, ref_, message, context)
            },
        }
    }
    pub(crate) async fn call(
        state: &mut A,
        ref_: AgentRef<A>,
        message: DynMessage,
        context: &mut Context<'_>,
    ) -> Result<bool, Error>
    where
        Self: inventory::Collect,
    {
        for handler in inventory::iter::<Self> {
            return (handler.handle_dyn_fn)(state, ref_, message, context).await;
        }
        Err(Error(anyhow!("No handler for this message/agent pairing")))
    }
}

impl<A: Agent> inventory::Collect for HandlerDyn<A> {
    #[inline]
    fn registry() -> &'static inventory::Registry<Self> {
        dynamic_registry()
    }
}
