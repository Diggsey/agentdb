//! Module for a snapshot proxy agent

use std::collections::VecDeque;

use agentdb_system::*;
use serde::{Deserialize, Serialize};

use crate::{RpcRequest, RpcResponse};

#[derive(Serialize, Deserialize)]
enum QueueItem {
    Message(DynMessage),
    Request(RpcRequest),
}

/// Proxy agent which will present consistent snapshot of another agent
#[agent(name = "adb.proxies.snapshot")]
#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    inner_ref: DynAgentRef,
    current_snapshot: Option<DynAgent>,
    queue: VecDeque<QueueItem>,
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
            .map(DynAgent::downcast)
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

/// RPC sent to the snapshot agent to both deliver a message, and get
/// back the updated state.
#[message(name = "adb.proxies.snapshot.request")]
#[derive(Serialize, Deserialize)]
pub struct SnapshotRequest {
    /// RPC request header
    pub rpc: RpcRequest,
}

/// RPC response with the new agent state.
#[message(name = "adb.proxies.snapshot.response")]
#[derive(Serialize, Deserialize)]
pub struct SnapshotResponse {
    /// RPC response header
    pub rpc: RpcResponse,
    /// Agent state
    pub state: Option<DynAgent>,
}

impl SnapshotResponse {
    /// Downcast the snapshot state assuming a particular agent type.
    pub fn downcast_state<A: Agent>(self) -> Result<Option<A>, DynAgent> {
        self.state.map(|a| Ok(*a.downcast()?)).transpose()
    }
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
        context.dyn_send(inner_ref, RegisterSnapshot { snapshot: ref_ }.into())?;
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
                Some(QueueItem::Message(msg)) => {
                    context.dyn_send(self.inner_ref, msg)?;
                    break;
                }
                Some(QueueItem::Request(rpc)) => {
                    rpc.respond(
                        |rpc| SnapshotResponse {
                            rpc,
                            state: self.current_snapshot.clone(),
                        },
                        context,
                    )?;
                    break;
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
impl Handle<SnapshotRequest> for Snapshot {
    async fn handle(
        &mut self,
        _ref_: AgentRef<Self>,
        message: SnapshotRequest,
        context: &mut Context,
    ) -> Result<bool, Error> {
        self.current_snapshot = self.inner_ref.load(context.global()).await?;

        if self.is_ready {
            message.rpc.respond(
                |rpc| SnapshotResponse {
                    rpc,
                    state: self.current_snapshot.clone(),
                },
                context,
            )?;
        } else {
            self.queue.push_back(QueueItem::Request(message.rpc));
        }

        Ok(false)
    }
}

#[constructor]
impl Construct for SnapshotRequest {
    type Agent = Snapshot;

    async fn construct(
        mut self,
        _ref_: AgentRef<Snapshot>,
        context: &mut Context,
    ) -> Result<Option<Self::Agent>, Error> {
        self.rpc
            .respond(|rpc| SnapshotResponse { rpc, state: None }, context)?;
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
            self.queue.push_back(QueueItem::Message(msg));
        }

        Ok(false)
    }
}
