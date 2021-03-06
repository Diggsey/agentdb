use agentdb_core::Error;
use async_trait::async_trait;
use downcast_rs::{impl_downcast, DowncastSync};
use serde::{Deserialize, Serialize};

use crate::agent::DynAgent;
use crate::agent_ref::DynAgentRef;
use crate::constructor::Constructor;
use crate::context::Context;
use crate::handler::Handler;
use crate::serializer::{DefaultSerializer, Serializer};

/// A message of any type.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DynMessage(pub(crate) Vec<u8>);

impl DynMessage {
    /// Attempt to downcast this message to a concrete type
    pub fn downcast<M: Message>(self) -> Result<Box<M>, Self> {
        if let Ok(m) = DefaultSerializer.deserialize::<Box<dyn Message>>(&self.0) {
            if let Ok(m) = m.downcast() {
                return Ok(m);
            }
        }
        Err(self)
    }
}

impl<M: Message> From<M> for DynMessage {
    fn from(m: M) -> Self {
        Self(
            DefaultSerializer
                .serialize(&m as &dyn Message)
                .expect("Infallible serialization"),
        )
    }
}

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
    if let Some(agent_state) = maybe_agent_state.take() {
        let mut agent_state = agent_state.deserialize()?;
        if Handler::call(&mut *agent_state, agent_ref, message, context).await? {
            agent_state._internal_destruct(agent_ref, context).await?;
        } else {
            *maybe_agent_state = Some(agent_state.into());
        }
    } else {
        *maybe_agent_state = Constructor::call(message, agent_ref, context).await?;
    }
    Ok(())
}

pub(crate) async fn deliver_unknown_message(
    message: DynMessage,
    agent_ref: DynAgentRef,
    maybe_agent_state: &mut Option<DynAgent>,
    context: &mut Context<'_>,
) -> Result<(), Error> {
    if let Some(agent_state) = maybe_agent_state.take() {
        let mut agent_state = agent_state.deserialize()?;
        if agent_state
            ._internal_handle_dyn(agent_ref, message, context)
            .await?
        {
            agent_state._internal_destruct(agent_ref, context).await?;
        } else {
            *maybe_agent_state = Some(agent_state.into());
        }
    }
    Ok(())
}

/// Trait implemented by message types.
#[typetag::serde]
#[async_trait]
pub trait Message: DowncastSync {
    #[doc(hidden)]
    async fn _internal_deliver(
        self: Box<Self>,
        agent_ref: DynAgentRef,
        maybe_agent_state: &mut Option<DynAgent>,
        context: &mut Context,
    ) -> Result<(), Error>;
}

impl_downcast!(sync Message);
