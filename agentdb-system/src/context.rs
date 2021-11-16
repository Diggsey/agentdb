use agentdb_core::{send_messages, Error, HookContext, MessageToSend, Timestamp};
use foundationdb::{Database, TransactOption, Transaction};
use futures::FutureExt;
use uuid::Uuid;

use crate::agent_ref::{AgentRef, DynAgentRef};
use crate::constructor::{Construct, Constructor};
use crate::handler::{Handle, Handler};
use crate::message::{DynMessage, Message};
use crate::root::Root;
use crate::serializer::{DefaultSerializer, Serializer};

pub type CommitHook = Box<dyn FnOnce(HookContext) + Send + Sync + 'static>;

pub struct Context<'a> {
    pub(crate) tx: &'a Transaction,
    pub(crate) root: Root,
    pub(crate) messages: Vec<MessageToSend>,
    pub(crate) commit_hooks: Vec<CommitHook>,
}

impl<'a> ContextLike for Context<'a> {
    fn dyn_send_at(
        &mut self,
        handle: DynAgentRef,
        message: DynMessage,
        when: Timestamp,
    ) -> Result<(), Error> {
        self.messages.push(MessageToSend {
            recipient_root: handle.root().to_bytes(),
            recipient_id: handle.id(),
            when,
            content: DefaultSerializer.serialize(&message)?,
        });
        Ok(())
    }
}

impl<'a> Context<'a> {
    pub(crate) fn new(tx: &'a Transaction, root: Root) -> Self {
        Self {
            tx,
            root,
            messages: Vec::new(),
            commit_hooks: Vec::new(),
        }
    }

    pub fn dyn_run_on_commit(&mut self, f: CommitHook) {
        self.commit_hooks.push(f);
    }
    pub fn run_on_commit(&mut self, f: impl FnOnce(HookContext) + Send + Sync + 'static) {
        self.dyn_run_on_commit(Box::new(f))
    }
    pub fn tx(&self) -> &'a Transaction {
        self.tx
    }
    pub fn root(&self) -> Root {
        self.root
    }
}

#[derive(Debug, Default)]
pub struct ExternalContext {
    messages: Vec<MessageToSend>,
}

impl ContextLike for ExternalContext {
    fn dyn_send_at(
        &mut self,
        handle: DynAgentRef,
        message: DynMessage,
        when: Timestamp,
    ) -> Result<(), Error> {
        self.messages.push(MessageToSend {
            recipient_root: handle.root().to_bytes(),
            recipient_id: handle.id(),
            when,
            content: DefaultSerializer.serialize(&message)?,
        });
        Ok(())
    }
}

impl ExternalContext {
    pub fn new() -> Self {
        Self::default()
    }

    async fn run_internal(&self, tx: &Transaction, user_version: u16) -> Result<(), Error> {
        send_messages(tx, &self.messages, user_version).await
    }
    pub async fn run_with_tx(self, tx: &Transaction, user_version: u16) -> Result<(), Error> {
        self.run_internal(tx, user_version).await
    }
    pub async fn run_with_db(self, db: &Database) -> Result<(), Error> {
        db.transact_boxed(
            self,
            |tx, this| this.run_internal(tx, 0).boxed(),
            TransactOption::default(),
        )
        .await
    }
}

pub trait ContextLike {
    // Required
    fn dyn_send_at(
        &mut self,
        handle: DynAgentRef,
        message: DynMessage,
        when: Timestamp,
    ) -> Result<(), Error>;

    // Provided
    fn dyn_send(&mut self, handle: DynAgentRef, message: DynMessage) -> Result<(), Error> {
        self.dyn_send_at(handle, message, Timestamp::zero())
    }
    fn dyn_construct(&mut self, root: Root, message: DynMessage) -> Result<DynAgentRef, Error> {
        self.dyn_construct_at(root, message, Timestamp::zero())
    }
    fn dyn_construct_at(
        &mut self,
        root: Root,
        message: DynMessage,
        when: Timestamp,
    ) -> Result<DynAgentRef, Error> {
        self.dyn_construct_at_with(root, |_| message, when)
    }
    fn dyn_construct_with(
        &mut self,
        root: Root,
        message_fn: impl FnOnce(DynAgentRef) -> DynMessage,
    ) -> Result<DynAgentRef, Error> {
        self.dyn_construct_at_with(root, message_fn, Timestamp::zero())
    }
    fn dyn_construct_at_with(
        &mut self,
        root: Root,
        message_fn: impl FnOnce(DynAgentRef) -> DynMessage,
        when: Timestamp,
    ) -> Result<DynAgentRef, Error> {
        let handle = DynAgentRef {
            root,
            id: Uuid::new_v4(),
        };
        self.dyn_send_at(handle, message_fn(handle), when)?;
        Ok(handle)
    }

    fn send<M: Message, A: Handle<M>>(
        &mut self,
        handle: AgentRef<A>,
        message: M,
    ) -> Result<(), Error>
    where
        Handler<M>: inventory::Collect,
    {
        self.send_at(handle, message, Timestamp::zero())
    }
    fn send_at<M: Message, A: Handle<M>>(
        &mut self,
        handle: AgentRef<A>,
        message: M,
        when: Timestamp,
    ) -> Result<(), Error>
    where
        Handler<M>: inventory::Collect,
    {
        self.dyn_send_at(handle.into(), Box::new(message), when)
    }
    fn construct<M: Construct>(
        &mut self,
        root: Root,
        message: M,
    ) -> Result<AgentRef<M::Agent>, Error>
    where
        Constructor<M>: inventory::Collect,
    {
        self.construct_at(root, message, Timestamp::zero())
    }
    fn construct_at<M: Construct>(
        &mut self,
        root: Root,
        message: M,
        when: Timestamp,
    ) -> Result<AgentRef<M::Agent>, Error>
    where
        Constructor<M>: inventory::Collect,
    {
        self.construct_at_with(root, |_| message, when)
    }
    fn construct_with<M: Construct>(
        &mut self,
        root: Root,
        message_fn: impl FnOnce(AgentRef<M::Agent>) -> M,
    ) -> Result<AgentRef<M::Agent>, Error>
    where
        Constructor<M>: inventory::Collect,
    {
        self.construct_at_with(root, message_fn, Timestamp::zero())
    }
    fn construct_at_with<M: Construct>(
        &mut self,
        root: Root,
        message_fn: impl FnOnce(AgentRef<M::Agent>) -> M,
        when: Timestamp,
    ) -> Result<AgentRef<M::Agent>, Error>
    where
        Constructor<M>: inventory::Collect,
    {
        Ok(self
            .dyn_construct_at_with(
                root,
                |ref_| Box::new(message_fn(ref_.unchecked_downcast())),
                when,
            )?
            .unchecked_downcast())
    }
}
