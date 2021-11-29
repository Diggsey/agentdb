use agentdb_core::{
    send_messages, Error, Global, HookContext, OutboundMessage, StateFnInput, Timestamp,
};
use anyhow::anyhow;
use foundationdb::directory::DirectoryOutput;
use foundationdb::{TransactOption, Transaction};
use futures::FutureExt;
use uuid::Uuid;

use crate::agent_ref::{AgentRef, DynAgentRef};
use crate::constructor::{Construct, Constructor};
use crate::handler::{Handle, Handler};
use crate::message::{DynMessage, Message};
use crate::root::Root;
use crate::serializer::{DefaultSerializer, Serializer};

// Require the ability to burst 500 messages for safe clearance
const MIN_SAFE_CLEARANCE: i64 = 500;

/// A function to run after an agent's new state has been committed.
pub type CommitHook = Box<dyn FnOnce(HookContext) + Send + Sync + 'static>;

/// Context passed to an agent's state function
pub struct Context<'a> {
    pub(crate) input: &'a StateFnInput<'a>,
    pub(crate) operation_id: Uuid,
    pub(crate) root: Root,
    pub(crate) messages: Vec<OutboundMessage>,
    pub(crate) commit_hooks: Vec<CommitHook>,
}

impl<'a> ContextLike for Context<'a> {
    fn dyn_send_at(
        &mut self,
        handle: DynAgentRef,
        message: DynMessage,
        when: Timestamp,
    ) -> Result<(), Error> {
        self.messages.push(OutboundMessage {
            recipient_root: handle.root().to_string(),
            recipient_id: handle.id(),
            operation_id: self.operation_id,
            when,
            content: DefaultSerializer.serialize(&message)?,
        });
        Ok(())
    }

    fn operation_id(&self) -> Uuid {
        self.operation_id
    }
}

impl<'a> Context<'a> {
    pub(crate) fn new(input: &'a StateFnInput<'a>, root: Root) -> Self {
        Self {
            input,
            root,
            operation_id: Uuid::nil(),
            messages: Vec::new(),
            commit_hooks: Vec::new(),
        }
    }

    /// Schedule `f` to be called after the agent's new state is committed.
    pub fn dyn_run_on_commit(&mut self, f: CommitHook) {
        self.commit_hooks.push(f);
    }
    /// Schedule `f` to be called after the agent's new state is committed.
    pub fn run_on_commit(&mut self, f: impl FnOnce(HookContext) + Send + Sync + 'static) {
        self.dyn_run_on_commit(Box::new(f))
    }
    /// Obtain the current FoundationDB transaction
    pub fn tx(&self) -> &'a Transaction {
        self.input.tx
    }
    /// Ontain the current AgentDB root.
    pub fn root(&self) -> Root {
        self.root
    }
    /// Obtain a FoundationDB directory which is unique to this agent, for storing custom
    /// data which is too large to be stored as part of the agent's normal state.
    pub async fn user_dir(&self) -> Result<DirectoryOutput, Error> {
        self.input.user_dir().await
    }
    /// Check for a certain amount of clearance in the current operation. If there is
    /// not enough clearance, an error is returned. This can be used to direct failures
    /// towards "safe points" in the system, where a stalled agent will not cause widespread
    /// unavailability.
    pub async fn require_clearance(&self) -> Result<(), Error> {
        if self.input.clearance(self.operation_id).await? < MIN_SAFE_CLEARANCE {
            Err(Error(anyhow!("Required clearance not met; failing safely")))
        } else {
            Ok(())
        }
    }
}

/// Context which can be used from outside the AgentDB system.
#[derive(Debug)]
pub struct ExternalContext {
    operation_id: Uuid,
    messages: Vec<OutboundMessage>,
}

impl ContextLike for ExternalContext {
    fn dyn_send_at(
        &mut self,
        handle: DynAgentRef,
        message: DynMessage,
        when: Timestamp,
    ) -> Result<(), Error> {
        self.messages.push(OutboundMessage {
            recipient_root: handle.root().to_string(),
            recipient_id: handle.id(),
            operation_id: self.operation_id,
            when,
            content: DefaultSerializer.serialize(&message)?,
        });
        Ok(())
    }

    fn operation_id(&self) -> Uuid {
        self.operation_id
    }
}

impl ExternalContext {
    /// Construct a new external context. The context will be associated
    /// with a new operation.
    pub fn new() -> Self {
        Self {
            operation_id: Uuid::new_v4(),
            messages: Vec::new(),
        }
    }

    async fn run_internal(
        &self,
        global: &Global,
        tx: &Transaction,
        user_version: u16,
    ) -> Result<(), Error> {
        send_messages(tx, global, &self.messages, user_version).await
    }

    /// Run all the side-effects accumulated within this context inside the
    /// provided transaction.
    pub async fn run_tx(
        self,
        global: &Global,
        tx: &Transaction,
        user_version: u16,
    ) -> Result<(), Error> {
        self.run_internal(global, tx, user_version).await
    }

    /// Run all the side-effects accumulated within this context.
    pub async fn run(self, global: &Global) -> Result<(), Error> {
        global
            .db()
            .transact_boxed(
                (global, self),
                |tx, &mut (global, ref this)| this.run_internal(global, tx, 0).boxed(),
                TransactOption::default(),
            )
            .await
    }
}

impl Default for ExternalContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait implemented by contexts which can send messages into
/// the AgentDB system.
pub trait ContextLike {
    // Required
    /// The most general way to send a message: can send any message type to
    /// any agent type at a scheduled time.
    fn dyn_send_at(
        &mut self,
        handle: DynAgentRef,
        message: DynMessage,
        when: Timestamp,
    ) -> Result<(), Error>;

    /// Obtain the ID of the operation to which messages sent using this context belong.
    fn operation_id(&self) -> Uuid;

    // Provided
    /// Immediately send a message of any type to an agent of any type.
    fn dyn_send(&mut self, handle: DynAgentRef, message: DynMessage) -> Result<(), Error> {
        self.dyn_send_at(handle, message, Timestamp::zero())
    }
    /// Immediately construct a new agent of unknown type using a message of any type.
    fn dyn_construct(&mut self, root: Root, message: DynMessage) -> Result<DynAgentRef, Error> {
        self.dyn_construct_at(root, message, Timestamp::zero())
    }
    /// Schedule a new agent of unknown type to be constructed using a message of any type.
    fn dyn_construct_at(
        &mut self,
        root: Root,
        message: DynMessage,
        when: Timestamp,
    ) -> Result<DynAgentRef, Error> {
        self.dyn_construct_at_with(root, |_| message, when)
    }
    /// Immediately construct a new agent of unknown type using a message of any type, using
    /// a callback to allow the message to know the new agent's ID.
    fn dyn_construct_with(
        &mut self,
        root: Root,
        message_fn: impl FnOnce(DynAgentRef) -> DynMessage,
    ) -> Result<DynAgentRef, Error> {
        self.dyn_construct_at_with(root, message_fn, Timestamp::zero())
    }
    /// Schedule a new agent of unknown type to be constructed using a message of any type, using
    /// a callback to allow the message to know the new agent's ID.
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

    /// Immediately send a message to an agent.
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

    /// Schedule a message to be sent to an agent.
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

    /// Immediately construct an agent.
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

    /// Schedule an agent to be constructed.
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

    /// Immediate construct an agent, using a callback to allow the message to
    /// know the new agent's ID.
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
    /// Schedule an agent to be constructed, using a callback to allow the message to
    /// know the new agent's ID.
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
