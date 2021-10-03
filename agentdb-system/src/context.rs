use agentdb_core::{Error, HookContext, MessageToSend};
use chrono::{DateTime, Utc};
use foundationdb::Transaction;

use crate::agent_ref::{AgentRef, DynAgentRef};
use crate::handler::{Handle, Handler};
use crate::message::{DynMessage, Message};
use crate::serializer::{DefaultSerializer, Serializer};

pub type CommitHook = Box<dyn FnOnce(&HookContext) + Send + Sync + 'static>;

pub struct Context<'a> {
    pub(crate) tx: &'a Transaction,
    pub(crate) messages: Vec<MessageToSend>,
    pub(crate) commit_hooks: Vec<CommitHook>,
}

impl<'a> Context<'a> {
    pub(crate) fn new(tx: &'a Transaction) -> Self {
        Self {
            tx,
            messages: Vec::new(),
            commit_hooks: Vec::new(),
        }
    }
    pub fn dyn_send(
        &mut self,
        handle: DynAgentRef,
        message: DynMessage,
        when: DateTime<Utc>,
    ) -> Result<(), Error> {
        self.messages.push(MessageToSend {
            recipient_root: handle.root().to_bytes(),
            recipient_id: handle.id(),
            when,
            content: DefaultSerializer.serialize(&message)?,
        });
        Ok(())
    }
    pub fn dyn_run_on_commit(&mut self, f: CommitHook) {
        self.commit_hooks.push(f);
    }
    pub fn send<M: Message, A: Handle<M>>(
        &mut self,
        handle: AgentRef<A>,
        message: M,
        when: DateTime<Utc>,
    ) -> Result<(), Error>
    where
        Handler<M>: inventory::Collect,
    {
        self.dyn_send(handle.into(), Box::new(message), when)
    }
    pub fn run_on_commit(&mut self, f: impl FnOnce(&HookContext) + Send + Sync + 'static) {
        self.dyn_run_on_commit(Box::new(f))
    }
    pub fn tx(&self) -> &'a Transaction {
        self.tx
    }
}
