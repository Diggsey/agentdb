use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    sync::Arc,
};

use agentdb_system::*;
use anyhow::anyhow;
use futures::{Future, FutureExt, StreamExt, TryFutureExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Header used by messages intended to elicit a response.
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcRequest {
    /// The agent to whom the response should be sent.
    pub caller: Option<DynAgentRef>,
    /// The ID of this RPC request.
    pub id: Uuid,
}

/// Header used by messages intended to respond to an RPC.
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcResponse {
    /// The ID of the corresponding request.
    pub id: Uuid,
}

impl RpcRequest {
    /// Dynamically respond to this RPC.
    pub fn respond_dyn(
        self,
        response_fn: impl FnOnce(RpcResponse) -> DynMessage,
        context: &mut Context,
    ) -> Result<(), Error> {
        if let Some(caller) = self.caller {
            context.dyn_send(caller, response_fn(RpcResponse { id: self.id }))?;
        }
        Ok(())
    }

    /// Respond to this RPC.
    pub fn respond<M: Message>(
        self,
        response_fn: impl FnOnce(RpcResponse) -> M,
        context: &mut Context,
    ) -> Result<(), Error> {
        self.respond_dyn(|resp| response_fn(resp).into(), context)
    }

    /// Used when the caller doesn't need a response from the RPC.
    pub fn no_response() -> Self {
        Self {
            caller: None,
            id: id::new(),
        }
    }

    /// Construct a new RPC request.
    pub fn new(caller: impl Into<DynAgentRef>) -> Self {
        Self {
            caller: Some(caller.into()),
            id: id::new(),
        }
    }

    /// Construct a new instance which will resolve a future when a response is received.
    pub async fn future_dyn(
        global: Arc<Global>,
        root: Root,
        timeout: Timestamp,
    ) -> Result<
        (
            Self,
            impl Future<Output = Result<DynMessage, Error>> + Send + 'static,
        ),
        Error,
    > {
        let mut context = ExternalContext::new();
        let agent_ref = context.construct(root, RpcHelperCreate { timeout })?;
        context.run(&global).await?;

        let fut = Box::pin(agent_ref.watch_stream(global).skip(1).try_filter_map(
            |maybe_state| async move {
                if let Some(state) = maybe_state {
                    Ok(state.response)
                } else {
                    Err(Error(anyhow!("Timeout")))
                }
            },
        ))
        .into_future()
        .map(|(maybe_m, _)| maybe_m.unwrap_or_else(|| Err(Error(anyhow!("Cancelled")))));

        Ok((Self::new(agent_ref), fut))
    }
    /// Construct a new instance which will resolve a future when a response of a specific type is received.
    pub async fn future<M: Message>(
        global: Arc<Global>,
        root: Root,
        timeout: Timestamp,
    ) -> Result<
        (
            Self,
            impl Future<Output = Result<M, Error>> + Send + 'static,
        ),
        Error,
    > {
        let (rpc, fut) = Self::future_dyn(global, root, timeout).await?;
        Ok((
            rpc,
            fut.and_then(|m| async move {
                Ok(*m
                    .downcast()
                    .map_err(|_| Error(anyhow!("Downcast failed: unexpected message type")))?)
            }),
        ))
    }
}

impl RpcResponse {
    /// Attempt to complete the provided `RpcCompletable` using this RPC response.
    pub fn complete<T: RpcCompletable>(&self, value: &mut T) -> T::Output {
        value.complete_rpc(self.id)
    }
}

/// Implemented for various types that might be used to track an in-flight RPC
/// request.
pub trait RpcCompletable {
    /// The result of trying to complete the RPC with a specific response ID.
    type Output;
    /// Attempt to complete this RPC with a specific response ID.
    fn complete_rpc(&mut self, id: Uuid) -> Self::Output;
}

impl RpcCompletable for Option<Uuid> {
    type Output = bool;
    fn complete_rpc(&mut self, id: Uuid) -> bool {
        if *self == Some(id) {
            *self = None;
            true
        } else {
            false
        }
    }
}

impl RpcCompletable for BTreeSet<Uuid> {
    type Output = bool;
    fn complete_rpc(&mut self, id: Uuid) -> bool {
        self.remove(&id)
    }
}

impl RpcCompletable for HashSet<Uuid> {
    type Output = bool;
    fn complete_rpc(&mut self, id: Uuid) -> bool {
        self.remove(&id)
    }
}

impl<V> RpcCompletable for BTreeMap<Uuid, V> {
    type Output = Option<V>;
    fn complete_rpc(&mut self, id: Uuid) -> Option<V> {
        self.remove(&id)
    }
}

impl<V> RpcCompletable for HashMap<Uuid, V> {
    type Output = Option<V>;
    fn complete_rpc(&mut self, id: Uuid) -> Option<V> {
        self.remove(&id)
    }
}

// Helper agent used to create a `Notify` that will resolve a future
#[agent(name = "adb.rpc_helper")]
#[derive(Serialize, Deserialize, Debug)]
struct RpcHelper {
    response: Option<DynMessage>,
}

// Message to create a `NotifyHelper`
#[message(name = "adb.rpc_helper.create")]
#[derive(Serialize, Deserialize, Debug)]
struct RpcHelperCreate {
    timeout: Timestamp,
}

// Message to destroy a `NotifyHelper`
#[message(name = "adb.rpc_helper.destroy")]
#[derive(Serialize, Deserialize, Debug)]
struct RpcHelperDestroy;

#[constructor]
impl Construct for RpcHelperCreate {
    type Agent = RpcHelper;
    async fn construct(
        self,
        ref_: AgentRef<RpcHelper>,
        context: &mut Context,
    ) -> Result<Option<RpcHelper>, Error> {
        // Destroy ourselves when the timeout expires.
        context.send_at(ref_, RpcHelperDestroy, self.timeout)?;
        Ok(Some(RpcHelper { response: None }))
    }
}

#[handler]
impl Handle<RpcHelperDestroy> for RpcHelper {
    async fn handle(
        &mut self,
        _ref_: AgentRef<Self>,
        _message: RpcHelperDestroy,
        _context: &mut Context,
    ) -> Result<bool, Error> {
        Ok(true)
    }
}

#[handler]
impl HandleDyn for RpcHelper {
    async fn handle_dyn(
        &mut self,
        _ref_: AgentRef<Self>,
        message: DynMessage,
        _context: &mut Context,
    ) -> Result<bool, Error> {
        self.response = Some(message);
        Ok(false)
    }
}
