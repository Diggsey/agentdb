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
    pub fn dyn_construct(
        &mut self,
        root: Root,
        message: DynMessage,
        when: Timestamp,
    ) -> Result<DynAgentRef, Error> {
        let handle = DynAgentRef {
            root,
            id: Uuid::new_v4(),
        };
        self.messages.push(MessageToSend {
            recipient_root: handle.root().to_bytes(),
            recipient_id: handle.id(),
            when,
            content: DefaultSerializer.serialize(&message)?,
        });
        Ok(handle)
    }
    pub fn dyn_construct_with(
        &mut self,
        root: Root,
        message_fn: impl FnOnce(DynAgentRef) -> DynMessage,
        when: Timestamp,
    ) -> Result<DynAgentRef, Error> {
        let handle = DynAgentRef {
            root,
            id: Uuid::new_v4(),
        };
        self.messages.push(MessageToSend {
            recipient_root: handle.root().to_bytes(),
            recipient_id: handle.id(),
            when,
            content: DefaultSerializer.serialize(&message_fn(handle))?,
        });
        Ok(handle)
    }
    pub fn dyn_run_on_commit(&mut self, f: CommitHook) {
        self.commit_hooks.push(f);
    }
    pub fn send<M: Message, A: Handle<M>>(
        &mut self,
        handle: AgentRef<A>,
        message: M,
        when: Timestamp,
    ) -> Result<(), Error>
    where
        Handler<M>: inventory::Collect,
    {
        self.dyn_send(handle.into(), Box::new(message), when)
    }
    pub fn construct<M: Construct>(
        &mut self,
        root: Root,
        message: M,
        when: Timestamp,
    ) -> Result<AgentRef<M::Agent>, Error>
    where
        Constructor<M>: inventory::Collect,
    {
        Ok(self
            .dyn_construct(root, Box::new(message), when)?
            .unchecked_downcast())
    }
    pub fn construct_with<M: Construct>(
        &mut self,
        root: Root,
        message_fn: impl FnOnce(AgentRef<M::Agent>) -> M,
        when: Timestamp,
    ) -> Result<AgentRef<M::Agent>, Error>
    where
        Constructor<M>: inventory::Collect,
    {
        Ok(self
            .dyn_construct_with(
                root,
                |ref_| Box::new(message_fn(ref_.unchecked_downcast())),
                when,
            )?
            .unchecked_downcast())
    }
    pub fn run_on_commit(&mut self, f: impl FnOnce(HookContext) + Send + Sync + 'static) {
        self.dyn_run_on_commit(Box::new(f))
    }
    pub fn tx(&self) -> &'a Transaction {
        self.tx
    }
}

#[derive(Debug, Default)]
pub struct ExternalContext {
    messages: Vec<MessageToSend>,
}

impl ExternalContext {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn dyn_send(
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
    pub fn dyn_construct(
        &mut self,
        root: Root,
        message: DynMessage,
        when: Timestamp,
    ) -> Result<DynAgentRef, Error> {
        let handle = DynAgentRef {
            root,
            id: Uuid::new_v4(),
        };
        self.messages.push(MessageToSend {
            recipient_root: handle.root().to_bytes(),
            recipient_id: handle.id(),
            when,
            content: DefaultSerializer.serialize(&message)?,
        });
        Ok(handle)
    }
    pub fn dyn_construct_with(
        &mut self,
        root: Root,
        message_fn: impl FnOnce(DynAgentRef) -> DynMessage,
        when: Timestamp,
    ) -> Result<DynAgentRef, Error> {
        let handle = DynAgentRef {
            root,
            id: Uuid::new_v4(),
        };
        self.messages.push(MessageToSend {
            recipient_root: handle.root().to_bytes(),
            recipient_id: handle.id(),
            when,
            content: DefaultSerializer.serialize(&message_fn(handle))?,
        });
        Ok(handle)
    }

    pub fn send<M: Message, A: Handle<M>>(
        &mut self,
        handle: AgentRef<A>,
        message: M,
        when: Timestamp,
    ) -> Result<(), Error>
    where
        Handler<M>: inventory::Collect,
    {
        self.dyn_send(handle.into(), Box::new(message), when)
    }
    pub fn construct<M: Construct>(
        &mut self,
        root: Root,
        message: M,
        when: Timestamp,
    ) -> Result<AgentRef<M::Agent>, Error>
    where
        Constructor<M>: inventory::Collect,
    {
        Ok(self
            .dyn_construct(root, Box::new(message), when)?
            .unchecked_downcast())
    }
    pub fn construct_with<M: Construct>(
        &mut self,
        root: Root,
        message_fn: impl FnOnce(AgentRef<M::Agent>) -> M,
        when: Timestamp,
    ) -> Result<AgentRef<M::Agent>, Error>
    where
        Constructor<M>: inventory::Collect,
    {
        Ok(self
            .dyn_construct_with(
                root,
                |ref_| Box::new(message_fn(ref_.unchecked_downcast())),
                when,
            )?
            .unchecked_downcast())
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
