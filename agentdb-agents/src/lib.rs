#![deny(missing_docs)]
//! # agentdb-agents
//!
//! This crate defines several widely-useful agent types.

use agentdb_system::*;
use serde::{Deserialize, Serialize};

pub mod data_models;
pub mod effects;

/// Provides a generic way for one agent to notify another about some
/// event. It stores and agent handle and a message, and will send the
/// message to the agent when requested.
#[derive(Serialize, Deserialize, Default)]
pub struct Notify {
    inner: Option<(DynAgentRef, DynMessage)>,
}

impl Notify {
    /// Constructs a new instance that will do nothing when `notify` is called.
    pub fn none() -> Self {
        Self::default()
    }
    /// Construct a new instance with the provided agent and message.
    pub fn new<A, M>(handle: AgentRef<A>, message: M) -> Self
    where
        M: Message,
        A: Agent + Handle<M>,
    {
        Self::new_dyn(handle.into(), Box::new(message))
    }
    /// Construct a new instance with the provided agent and message of any type.
    pub fn new_dyn(handle: DynAgentRef, message: DynMessage) -> Self {
        Self {
            inner: Some((handle, message)),
        }
    }
    /// Trigger the message to be sent to the agent. Further calls to `notify` will
    /// have no effect.
    pub fn notify(&mut self, ctx: &mut Context) -> Result<(), Error> {
        if let Some((handle, message)) = self.inner.take() {
            ctx.dyn_send(handle, message)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
