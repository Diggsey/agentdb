use std::any::TypeId;

use agentdb_core::Error;
use anyhow::anyhow;
use async_trait::async_trait;
use futures::future::BoxFuture;

use crate::agent::Agent;
use crate::agent_ref::{AgentRef, DynAgentRef};
use crate::context::Context;
use crate::message::Message;
use crate::utils::dynamic_registry;

#[async_trait]
pub trait Handle<M>: Agent + Sized {
    async fn handle(
        &mut self,
        ref_: AgentRef<Self>,
        message: M,
        context: &mut Context,
    ) -> Result<bool, Error>;
}

pub struct Handler<M: Message>
where
    Self: inventory::Collect,
{
    type_id: TypeId,
    handle_fn: for<'a> fn(
        state: &'a mut dyn Agent,
        ref_: DynAgentRef,
        message: M,
        context: &'a mut Context,
    ) -> BoxFuture<'a, Result<bool, Error>>,
}

impl<M: Message> Handler<M>
where
    Self: inventory::Collect,
{
    pub fn new<A: Agent + Handle<M>>() -> Self {
        Self {
            type_id: TypeId::of::<A>(),
            handle_fn: |state: &mut dyn Agent, ref_, message, context| {
                let state: &mut A = state.downcast_mut().expect("Agent of the correct type");
                state.handle(ref_.unchecked_downcast(), message, context)
            },
        }
    }

    pub async fn call(
        state: &mut dyn Agent,
        ref_: DynAgentRef,
        message: M,
        context: &mut Context<'_>,
    ) -> Result<bool, Error>
    where
        Self: inventory::Collect,
    {
        let type_id = state.type_id();
        for handler in inventory::iter::<Self> {
            if handler.type_id == type_id {
                return (handler.handle_fn)(state, ref_, message, context).await;
            }
        }
        Err(Error(anyhow!("No handler for this message/agent pairing")))
    }
}

impl<M: Message> inventory::Collect for Handler<M> {
    #[inline]
    fn registry() -> &'static inventory::Registry<Self> {
        dynamic_registry()
    }
}