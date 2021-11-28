#![deny(missing_docs)]
//! # agentdb-system
//!
//! This crate provides a higher level interface to `agentdb-core`. The same concepts
//! of agents, messages, etc. exist, but the following new concepts are introduced:
//!
//! ## Agent types
//!
//! Rather than the evolution of the system being dictated by a single global state
//! function, this crate distinguishes different types of agent, and allows each
//! type to operate according to its own state function.
//!
//! New agent types can be created by adding the `#[agent(name = "...")]` attribute to a type.
//!
//! ## Message types
//!
//! Similarly to agents, messages are now distinguished by type. Agents can respond
//! to different types of messages in different ways.
//!
//! New message types can be created by adding the `#[message(name = "...")]` attribute to a type.
//!
//! ## Registry
//!
//! The different agent and message types are registered at startup in a global
//! registry. This is handled automatically by the corresponding attribute macros.
//!
//! The names of registered agents and messages should be unique and stable.
//!
//! ## Handlers
//!
//! Handlers are implementations of the `Handle<M>` or `HandleDyn` traits for an agent type.
//! They are registered with the `#[handler]` attribute. The handlers control how an
//! existing agent responds to messages.
//!
//! ## Constructors
//!
//! Constructors are implementations of the `Construct` trait for a message type.
//! They are registered with the `#[constructor]` attribute. The constructors
//! control the behaviour when a message is sent to an agent that doesn't exist.
//!
//! ## Destructors
//!
//! Destructors are implementations of the `Destruct` trait for an agent type.
//! They are registered with the `#[destructor]` attribute. The destructor
//! controls the behaviour of an agent when it ceases to exist.

mod agent;
mod agent_ref;
mod constructor;
mod context;
mod destructor;
mod dynamic_handler;
mod handler;
mod macros;
mod message;
mod root;
mod serializer;
mod system;
mod utils;

pub use agentdb_macros::*;

pub use agent::{Agent, DynAgent};
pub use agent_ref::{AgentRef, DynAgentRef};
pub use agentdb_core::{
    default_client_name, Error, Global, HookContext, Prepacked, Timestamp, TypedSubspace,
};
pub use constructor::{Construct, DynConstruct};
pub use context::{CommitHook, Context, ContextLike, ExternalContext};
pub use destructor::Destruct;
pub use dynamic_handler::HandleDyn;
pub use handler::Handle;
pub use message::{DynMessage, Message};
pub use root::Root;
pub use system::{run, start};

#[doc(hidden)]
pub mod hidden {
    pub use crate::agent::{destruct_agent, handle_dyn};
    pub use crate::constructor::Constructor;
    pub use crate::destructor::Destructor;
    pub use crate::dynamic_handler::HandlerDyn;
    pub use crate::handler::Handler;
    pub use crate::message::deliver_message;

    pub use async_trait::async_trait;
    pub use inventory;
    pub use typetag;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
