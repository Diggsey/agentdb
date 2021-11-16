use agentdb_system::*;
use serde::{Deserialize, Serialize};

pub mod data_models;
pub mod effects;

#[derive(Serialize, Deserialize, Default)]
pub struct Notify {
    inner: Option<(DynAgentRef, DynMessage)>,
}

impl Notify {
    pub fn none() -> Self {
        Self::default()
    }
    pub fn new<A, M>(handle: AgentRef<A>, message: M) -> Self
    where
        M: Message,
        A: Agent + Handle<M>,
    {
        Self::new_dyn(handle.into(), Box::new(message))
    }
    pub fn new_dyn(handle: DynAgentRef, message: DynMessage) -> Self {
        Self {
            inner: Some((handle, message)),
        }
    }
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
