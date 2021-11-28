#![deny(missing_docs)]
//! # agentdb-core
//!
//! A reactive, persistent database using FoundationDB.
//!
//! The database has the following concepts:
//!
//! ## Agents
//!
//! Agents are the atomic building blocks of your system: each agent has state,
//! and that state can evolve over time according to your business logic. Agents
//! are identified by a UUID.
//!
//! A snapshot of an agent's state can be read externally, and an external observer
//! can also watch for changes to an agent's state.
//!
//! ## Messages
//!
//! Messages are used externally to effect changes within the system, and are
//! also used internally for communication between agents. In the latter case,
//! AgentDB guarantees exactly-once delivery of messages.
//!
//! ## State function
//!
//! The state function governs the behaviour of the entire system: it is provided
//! with an agent's initial state and a list of messages it has received. It returns
//! the new state of the agent, a list of messages to send, and optionally a
//! post-commit hook to run after the new state has been saved to the database.
//!
//! It is a pure function, making it simple to test every edge-case of the state
//! function without complex mocking or system setup.
//!
//! ## Operations
//!
//! Each message belongs to an operation. When an agent responds to an inbound message
//! by sending out new messages, it is expected that the causally linked messages belond
//! to the same operation as the original message.
//!
//! An operation is identified by a UUID and serves multiple purposes:
//!
//! - Tracing.
//!   You can use the operation ID to follow all the messages which were triggered by
//!   some initial external event.
//!
//! - Feedback-loop protection.
//!   It is possible that a logic error in your code triggers a cascade of messages,
//!   which themselves trigger more messages, ad infinitum. AgentDB prevents this
//!   by limiting the amount of work that can be done as part of a single operation
//!   per unit of time.
//!
//!   The [StateFnInput::clearance] function can be used to trigger a failure before
//!   this limit is hit, so that failures can be limited to "safe" places in the system.
//!
//! ## Post-commit hooks
//!
//! A post-commit hook can be returned by the state function, and is simply a piece
//! of code that will be executed once the new agent state has been committed to the
//! database.
//!
//! This should be used for all interactions external to AgentDB - eg. sending an HTTP
//! request, or reading a file. These will not be pure functions, and so will be harder
//! to test. As a result, it makes sense to keep them as simple as possible.
//!
//! ## Roots
//!
//! Multiple instances of AgentDB can run within the same FoundationDB cluster. Each
//! instance is known as a "root". Whilst messages can be freely sent between roots
//! without risking the "exactly-once" guarantee, each application should only act as
//! a client for a single root.
//!
//! Roots are a good way to split up your business logic into independently scalable
//! pieces.
//!
//! ## Clients
//!
//! AgentDB runs within your application. When you have multiple instances of your
//! application connected to the same FoundationDB cluster, each instance is an AgentDB
//! "client".
//!
//! Clients send a heartbeat via FoundationDB which allows them to keep track of any
//! other clients which are active. If a client is terminated for any reason, its
//! heartbeat will stop updating, and the other clients will know that it no longer
//! exists.
//!
//! Each client is identified by a UUID, and uses its knowledge of the other active
//! clients, along with its own position within that sequence of client IDs, to determine
//! which partitions it should own.
//!
//! ## Partitions
//!
//! Agents in AgentDB are implicitly divided into a certain number of partitions. Partitions
//! are numbered sequentially, but do not necessarily start at zero. At any one time, a
//! partition is owned by a single client, which allows clients to cooperate efficiently
//! rather than conflicting with each other.
//!
//! Since the allocation of agents to partitions is an implicit function of the agent's ID
//! and the total number of partitions, re-partitioning is possible without actually moving
//! the agents around within FoundationDB.
//!
//! However, messages in AgentDB are explicitly allocated to a partition (based on the
//! receiving agent's partition) and so when re-partitioning occurs, in-flight messages
//! must be moved to the corresponding new partitions before new messages can be processed.
//! Apart from this pause, re-partitioning can be done completely online without causing
//! extended donwtime
//!

use std::{fmt::Debug, sync::Arc, time::Duration};

use byteorder::{ByteOrder, LittleEndian};
use foundationdb::{
    directory::{Directory, DirectoryOutput},
    Transaction,
};
use futures::future::BoxFuture;
use message::{INITIAL_TS_OFFSET, MS_PER_MSG_PER_OP};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod admin;
pub mod blob;
pub mod cancellation;
mod client;
mod directories;
mod error;
mod message;
mod partition;
mod prepacked;
mod typed_subspace;
mod utils;

use cancellation::{spawn_cancellable, CancellableHandle};
use client::{client_task, PartitionRange};
pub use directories::Global;
pub use error::Error;
pub use message::send_messages;
pub use prepacked::Prepacked;
pub use typed_subspace::TypedSubspace;
pub use utils::Timestamp;

const DEFAULT_PARTITION_RANGE: PartitionRange = PartitionRange {
    offset: 0,
    count: 100,
};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const GC_INTERVAL: Duration = Duration::from_secs(10);
const MAX_BATCH_SIZE: usize = 100;

#[derive(Serialize, Deserialize)]
struct MessageHeader {
    recipient_id: Uuid,
    blob_id: Uuid,
    operation_id: Uuid,
}

/// A single message received by the agent.
#[derive(Debug)]
pub struct InboundMessage {
    /// The ID of the operation this message belongs to.
    pub operation_id: Uuid,
    /// The contents of the message.
    pub data: Vec<u8>,
}

/// The input to the state function.
#[derive(Debug)]
pub struct StateFnInput<'a> {
    operation_ts: &'a TypedSubspace<Uuid>,
    user_dir: &'a DirectoryOutput,
    /// The current root.
    pub root: &'a str,
    /// The current FoundationDB transaction.
    pub tx: &'a Transaction,
    /// The ID of the agent being processed.
    pub id: Uuid,
    /// The current state of the agent, or `None` if the agent does not exist.
    pub state: Option<Vec<u8>>,
    /// The ordered list of messages to be handled by this agent.
    pub messages: Vec<InboundMessage>,
}

impl<'a> StateFnInput<'a> {
    /// Obtain a FoundationDB directory for sole use by this agent.
    /// This can be used when the agent wants to store more state than can be
    /// efficiently loaded/saved in one go.
    pub async fn user_dir(&self) -> Result<DirectoryOutput, Error> {
        self.user_dir
            .create_or_open(self.tx, vec![self.id.to_string()], None, None)
            .await
            .map_err(Error::from_dir)
    }

    /// Returns the clearance level of the given operation: how many messages
    /// can be sent as part of this operation before the system will return
    /// an error. If no messages are sent, the clearance level will gradually
    /// increase over time.
    pub async fn clearance(&self, operation_id: Uuid) -> Result<i64, Error> {
        let current_ts = Timestamp::now().millis();
        let initial_operation_ts = current_ts - INITIAL_TS_OFFSET;
        let key = self.operation_ts.pack(&operation_id);
        let operation_ts = self
            .tx
            .get(&key, true)
            .await?
            .map(|slice| LittleEndian::read_i64(&slice))
            .unwrap_or(initial_operation_ts);
        Ok((current_ts - operation_ts) / MS_PER_MSG_PER_OP)
    }
}

/// A message to be sent when the new agent state is saved.
#[derive(Debug)]
pub struct OutboundMessage {
    /// The root of the receiving agent.
    pub recipient_root: String,
    /// The ID of the receiving agent.
    pub recipient_id: Uuid,
    /// The ID of the operation which caused this message to be sent.
    pub operation_id: Uuid,
    /// When the message should be sent. Use `Timestamp::zero()` to
    /// send messages immediately.
    pub when: Timestamp,
    /// The contents of the message.
    pub content: Vec<u8>,
}

/// A context accessible to post-commit hooks.
#[derive(Clone)]
pub struct HookContext {
    global: Arc<Global>,
}

impl HookContext {
    /// Access global information such as the FoundationDB connection.
    pub fn global(&self) -> &Arc<Global> {
        &self.global
    }
}

/// The type of a post-commit hook.
pub type CommitHook = Box<dyn FnOnce(HookContext) + Send + Sync + 'static>;

/// The return type of a state function.
pub struct StateFnOutput {
    /// The new state of the agent.
    pub state: Option<Vec<u8>>,
    /// The messages to send.
    pub messages: Vec<OutboundMessage>,
    /// A post-commit hook to run.
    pub commit_hook: CommitHook,
}

/// The type of a state function.
pub type StateFn =
    Arc<dyn for<'a> Fn(StateFnInput<'a>) -> BoxFuture<'a, Result<StateFnOutput, ()>> + Send + Sync>;

/// Start an AgentDB client, and obtain a cancellation handle.
pub fn start(
    client_name: String,
    global: Arc<Global>,
    root: String,
    state_fn: StateFn,
) -> CancellableHandle<Result<(), Error>> {
    spawn_cancellable(|c| client_task(client_name, global, root, state_fn, c))
}

/// Run the AgentDB client forever, or until it returns an error.
pub async fn run(
    client_name: String,
    global: Arc<Global>,
    root: String,
    state_fn: StateFn,
) -> Result<(), Error> {
    start(client_name, global, root, state_fn).await?
}

/// Construct the default client name. This is a combination of the hostname and process ID.
pub fn default_client_name() -> String {
    let hn = hostname::get().unwrap_or_else(|_| "unknown".into());
    let pid = std::process::id();
    format!("{}:{}", hn.to_string_lossy(), pid)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
