use std::{fmt::Debug, marker::PhantomData, sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use foundationdb::{Database, Transaction};
use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod blob;
pub mod cancellation;
mod client;
mod error;
mod message;
mod partition;
mod subspace;
mod utils;

use cancellation::spawn_cancellable;
use client::client_task;
pub use error::Error;
pub use message::send_messages;

const DEFAULT_PARTITION_COUNT: u32 = 100;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const MAX_BATCH_SIZE: usize = 100;

#[derive(Serialize, Deserialize)]
struct MessageHeader {
    recipient_id: Uuid,
    blob_id: Uuid,
}

#[derive(Debug)]
pub struct StateFnInput<'a> {
    pub tx: &'a Transaction,
    pub id: Uuid,
    pub state: Option<Vec<u8>>,
    pub messages: Vec<Vec<u8>>,
}

pub struct MessageToSend {
    pub recipient_root: Vec<u8>,
    pub recipient_id: Uuid,
    pub when: DateTime<Utc>,
    pub content: Vec<u8>,
}

#[derive(Clone)]
pub struct HookContext<'a> {
    db: Arc<Database>,
    phantom: PhantomData<&'a ()>,
}

pub type CommitHook = Box<dyn FnOnce(&HookContext) + Send + Sync + 'static>;

pub struct StateFnOutput {
    pub state: Option<Vec<u8>>,
    pub messages: Vec<MessageToSend>,
    pub commit_hook: CommitHook,
}

pub type StateFn = Arc<dyn for<'a> Fn(StateFnInput<'a>) -> BoxFuture<'a, StateFnOutput> + Send + Sync>;

pub async fn run(db: Arc<Database>, root: Vec<u8>, state_fn: StateFn) {
    let handle = spawn_cancellable(|c| client_task(db, root, state_fn, c));
    handle.await.unwrap().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
