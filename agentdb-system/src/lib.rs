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
pub use agentdb_core::{default_client_name, Error, HookContext, Timestamp};
pub use constructor::{Construct, DynConstruct};
pub use context::{CommitHook, Context, ExternalContext};
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
