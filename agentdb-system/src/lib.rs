mod agent;
mod agent_ref;
mod constructor;
mod context;
mod destructor;
mod handler;
mod macros;
mod message;
mod root;
mod serializer;
mod system;
mod utils;

pub use agent::{Agent, DynAgent};
pub use agent_ref::{AgentRef, DynAgentRef};
pub use agentdb_core::{Error, Timestamp};
pub use constructor::{Construct, DynConstruct};
pub use context::{CommitHook, Context, ExternalContext};
pub use destructor::Destruct;
pub use handler::Handle;
pub use message::{DynMessage, Message};
pub use root::Root;
pub use system::{run, start};

#[doc(hidden)]
pub mod hidden {
    pub use crate::agent::destruct_agent;
    pub use crate::constructor::Constructor;
    pub use crate::destructor::Destructor;
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
