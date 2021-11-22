use std::{fmt::Debug, sync::Arc, time::Duration};

use foundationdb::{
    directory::{Directory, DirectoryOutput},
    Transaction,
};
use futures::future::BoxFuture;
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
const MAX_BATCH_SIZE: usize = 100;

#[derive(Serialize, Deserialize)]
struct MessageHeader {
    recipient_id: Uuid,
    blob_id: Uuid,
}

#[derive(Debug)]
pub struct StateFnInput<'a> {
    user_dir: &'a DirectoryOutput,
    pub root: &'a str,
    pub tx: &'a Transaction,
    pub id: Uuid,
    pub state: Option<Vec<u8>>,
    pub messages: Vec<Vec<u8>>,
}

impl<'a> StateFnInput<'a> {
    pub async fn user_dir(&self) -> Result<DirectoryOutput, Error> {
        self.user_dir
            .create_or_open(self.tx, vec![self.id.to_string()], None, None)
            .await
            .map_err(Error::from_dir)
    }
}

#[derive(Debug)]
pub struct MessageToSend {
    pub recipient_root: String,
    pub recipient_id: Uuid,
    pub when: Timestamp,
    pub content: Vec<u8>,
}

#[derive(Clone)]
pub struct HookContext {
    global: Arc<Global>,
}

impl HookContext {
    pub fn global(&self) -> &Arc<Global> {
        &self.global
    }
}

pub type CommitHook = Box<dyn FnOnce(HookContext) + Send + Sync + 'static>;

pub struct StateFnOutput {
    pub state: Option<Vec<u8>>,
    pub messages: Vec<MessageToSend>,
    pub commit_hook: CommitHook,
}

pub type StateFn =
    Arc<dyn for<'a> Fn(StateFnInput<'a>) -> BoxFuture<'a, Result<StateFnOutput, ()>> + Send + Sync>;

pub fn start(
    client_name: String,
    global: Arc<Global>,
    root: String,
    state_fn: StateFn,
) -> CancellableHandle<Result<(), Error>> {
    spawn_cancellable(|c| client_task(client_name, global, root, state_fn, c))
}

pub async fn run(
    client_name: String,
    global: Arc<Global>,
    root: String,
    state_fn: StateFn,
) -> Result<(), Error> {
    start(client_name, global, root, state_fn).await?
}

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
