//! This module defines a callback type capable of causing
//! external side-effects.

use std::sync::Arc;

use agentdb_system::*;

/// Context which can be accessed be an effectful operation,
/// such as how many previous attempts have been made, and
/// where to send the result.
pub struct EffectContext {
    pub(crate) ref_: DynAgentRef,
    pub(crate) inner: HookContext,
    pub(crate) attempt: u64,
}

impl EffectContext {
    /// A reference to the effect agent
    pub fn ref_(&self) -> DynAgentRef {
        self.ref_
    }
    /// The number of this attempt
    pub fn attempt(&self) -> u64 {
        self.attempt
    }
    /// The database connection
    pub fn global(&self) -> &Arc<Global> {
        self.inner.global()
    }
    /// Send a dynamic response to the operation
    pub async fn dyn_send_response(&self, message: DynMessage) -> Result<(), Error> {
        let mut ctx = ExternalContext::new();
        ctx.dyn_send(self.ref_, message)?;
        ctx.run(self.global()).await?;
        Ok(())
    }
    /// Send a response to the operation
    pub async fn send_response(&self, message: impl Message) -> Result<(), Error> {
        self.dyn_send_response(Box::new(message)).await
    }
}

/// Represents a effectful operation that can be serialized
#[typetag::serde]
pub trait EffectCallback: Send + Sync + 'static {
    /// Run the callback.
    fn call(&self, context: EffectContext);
}
