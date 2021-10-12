use std::sync::Arc;

use agentdb_system::*;
use foundationdb::Database;

pub struct EffectContext {
    pub(crate) ref_: DynAgentRef,
    pub(crate) inner: HookContext,
    pub(crate) attempt: u64,
}

impl EffectContext {
    pub fn ref_(&self) -> DynAgentRef {
        self.ref_
    }
    pub fn attempt(&self) -> u64 {
        self.attempt
    }
    pub fn db(&self) -> Arc<Database> {
        self.inner.db()
    }
    pub async fn dyn_send_response(&self, message: DynMessage) -> Result<(), Error> {
        let mut ctx = ExternalContext::new();
        ctx.dyn_send(self.ref_, message, Timestamp::zero())?;
        ctx.run_with_db(&self.db()).await?;
        Ok(())
    }
    pub async fn send_response(&self, message: impl Message) -> Result<(), Error> {
        self.dyn_send_response(Box::new(message)).await
    }
}

#[typetag::serde]
pub trait EffectCallback: Send + Sync + 'static {
    fn call(&self, context: EffectContext);
}
