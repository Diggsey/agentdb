//! Module for a snapshot proxy agent

use std::collections::VecDeque;

use agentdb_system::*;
use serde::{Deserialize, Serialize};

use crate::Notify;

#[derive(Serialize, Deserialize)]
enum MessageOrNotify {
    Message(DynMessage),
    Notify(Notify),
}

/// Proxy agent which will present consistent snapshot of another agent
#[agent(name = "adb.proxies.snapshot")]
#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    inner_ref: DynAgentRef,
    current_snapshot: Option<DynAgent>,
    queue: VecDeque<MessageOrNotify>,
    is_ready: bool,
}

impl Snapshot {
    /// Access the current snapshot
    pub fn dyn_current(self) -> Option<DynAgent> {
        self.current_snapshot
    }

    /// Access the current snapshot, expecting a particular agent type
    pub fn current<T: Agent + Sized>(self) -> Result<Option<T>, DynAgent> {
        Ok(self
            .dyn_current()
            .map(<dyn Agent>::downcast)
            .transpose()?
            .map(|agent| *agent))
    }
}

/// Message to construct a snapshot agent
#[message(name = "adb.proxies.snapshot.create")]
#[derive(Serialize, Deserialize)]
pub struct CreateSnapshot {
    /// Message to use to construct the inner agent
    pub constructor: DynMessage,
}

/// Message sent from the snapshot agent to the inner agent after creation
#[message(name = "adb.proxies.snapshot.register")]
#[derive(Serialize, Deserialize)]
pub struct RegisterSnapshot {
    /// Handle to the snapshot agent
    pub snapshot: AgentRef<Snapshot>,
}

/// Message sent to the snapshot agent to request notification of applied changes
#[message(name = "adb.proxies.snapshot.request_notify")]
#[derive(Serialize, Deserialize)]
pub struct RequestNotifySnapshot {
    /// Notify object
    pub notify: Notify,
}

/// Message sent from the inner agent to indicate that it's in a consistent state
#[message(name = "adb.proxies.snapshot.ready")]
#[derive(Serialize, Deserialize)]
pub struct SnapshotReady;

#[constructor]
impl Construct for CreateSnapshot {
    type Agent = Snapshot;

    async fn construct(
        self,
        ref_: AgentRef<Snapshot>,
        context: &mut Context,
    ) -> Result<Option<Self::Agent>, Error> {
        let inner_ref = context.dyn_construct(ref_.root(), self.constructor)?;
        context.dyn_send(inner_ref, Box::new(RegisterSnapshot { snapshot: ref_ }))?;
        Ok(Some(Snapshot {
            inner_ref,
            current_snapshot: None,
            is_ready: false,
            queue: VecDeque::new(),
        }))
    }
}

#[handler]
impl Handle<SnapshotReady> for Snapshot {
    async fn handle(
        &mut self,
        _ref_: AgentRef<Self>,
        _message: SnapshotReady,
        context: &mut Context,
    ) -> Result<bool, Error> {
        self.current_snapshot = self.inner_ref.load(context.global()).await?;

        loop {
            match self.queue.pop_front() {
                Some(MessageOrNotify::Message(msg)) => {
                    context.dyn_send(self.inner_ref, msg)?;
                    break;
                }
                Some(MessageOrNotify::Notify(mut notify)) => {
                    notify.notify(context)?;
                }
                None => {
                    self.is_ready = true;
                    break;
                }
            }
        }

        Ok(self.current_snapshot.is_none())
    }
}

#[handler]
impl Handle<RequestNotifySnapshot> for Snapshot {
    async fn handle(
        &mut self,
        _ref_: AgentRef<Self>,
        mut message: RequestNotifySnapshot,
        context: &mut Context,
    ) -> Result<bool, Error> {
        self.current_snapshot = self.inner_ref.load(context.global()).await?;

        if self.is_ready {
            message.notify.notify(context)?;
        } else {
            self.queue
                .push_back(MessageOrNotify::Notify(message.notify));
        }

        Ok(false)
    }
}

#[constructor]
impl Construct for RequestNotifySnapshot {
    type Agent = Snapshot;

    async fn construct(
        mut self,
        _ref_: AgentRef<Snapshot>,
        context: &mut Context,
    ) -> Result<Option<Self::Agent>, Error> {
        self.notify.notify(context)?;
        Ok(None)
    }
}

#[handler]
impl HandleDyn for Snapshot {
    async fn handle_dyn(
        &mut self,
        _ref_: AgentRef<Self>,
        msg: DynMessage,
        context: &mut Context,
    ) -> Result<bool, Error> {
        if self.is_ready {
            context.dyn_send(self.inner_ref, msg)?;
            self.is_ready = false;
        } else {
            self.queue.push_back(MessageOrNotify::Message(msg));
        }

        Ok(false)
    }
}
