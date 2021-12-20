#![deny(missing_docs)]
//! # agentdb-agents
//!
//! This crate defines several widely-useful agent types.

use std::future::Future;

use agentdb_system::*;
use foundationdb::TransactOption;
use futures::FutureExt;
use serde::{Deserialize, Serialize};

pub mod data_models;
pub mod effects;
pub mod proxies;
mod rpc;
pub use rpc::*;

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
        Self::new_dyn(handle.into(), message.into())
    }
    /// Construct a new instance with the provided agent and message of any type.
    pub fn new_dyn(handle: DynAgentRef, message: DynMessage) -> Self {
        Self {
            inner: Some((handle, message)),
        }
    }
    /// Construct a new instance which will resolve a future when notified.
    pub async fn future(
        global: &Global,
        root: Root,
    ) -> Result<(Self, impl Future<Output = ()> + Send + Sync + 'static), Error> {
        let agent_ref = AgentRef::<NotifyHelper>::from_parts_unchecked(root, id::new());

        let fut = global
            .db()
            .transact_boxed(
                global,
                |tx, &mut global| agent_ref.watch(global, tx).boxed(),
                TransactOption::idempotent(),
            )
            .await?;

        Ok((Self::new(agent_ref, NotifyHelperUpdate), fut.map(|_| ())))
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

// Helper agent used to create a `Notify` that will resolve a future
#[agent(name = "adb.notify_helper")]
#[derive(Serialize, Deserialize)]
struct NotifyHelper;

// Message to create/destroy a `NotifyHelper`
#[message(name = "adb.notify_helper.update")]
#[derive(Serialize, Deserialize)]
struct NotifyHelperUpdate;

#[constructor]
impl Construct for NotifyHelperUpdate {
    type Agent = NotifyHelper;
    async fn construct(
        self,
        ref_: AgentRef<NotifyHelper>,
        context: &mut Context,
    ) -> Result<Option<NotifyHelper>, Error> {
        // Destroy ourselves on the next update. Don't do it synchronously
        // or else the "watch" won't trigger.
        context.send(ref_, NotifyHelperUpdate)?;
        Ok(Some(NotifyHelper))
    }
}

#[handler]
impl Handle<NotifyHelperUpdate> for NotifyHelper {
    async fn handle(
        &mut self,
        _ref_: AgentRef<Self>,
        _message: NotifyHelperUpdate,
        _context: &mut Context,
    ) -> Result<bool, Error> {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
