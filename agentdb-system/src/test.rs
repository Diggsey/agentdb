//! Utilities for testing agents
use std::fmt::Debug;

use agentdb_core::{id, Error, OutboundMessage, StateFnInput, StateFnOutput, Timestamp};
use uuid::Uuid;

use crate::{
    serializer::{DefaultSerializer, Serializer},
    system::system_fn_fallible,
    Agent, DynAgent, DynAgentRef, DynMessage, Message, Root,
};

/// Builder struct for running unit tests on agents
#[derive(Debug, Clone)]
pub struct TestBuilder {
    input: StateFnInput<'static>,
    operation_id: Uuid,
}

impl TestBuilder {
    /// Construct a new builder for the specified root
    pub fn new(root: Root) -> Self {
        Self {
            input: StateFnInput::test(root.name(), id::new(), None, Vec::new()),
            operation_id: id::new(),
        }
    }
    /// Use a specific ID for the agent under test, instead of an auto-generated ID
    pub fn with_id(mut self, id: Uuid) -> Self {
        self.input.id = id;
        self
    }
    /// Set the initial agent state
    pub fn with_initial_state(mut self, state: &dyn Agent) -> Self {
        self.input.state = Some(
            DefaultSerializer
                .serialize(state)
                .expect("Infallible serialization"),
        );
        self
    }
    /// Append a message to the agent's inbox
    pub fn with_message(mut self, message: &dyn Message) -> Self {
        self.input.messages.push(agentdb_core::InboundMessage {
            operation_id: self.operation_id,
            data: DefaultSerializer
                .serialize(message)
                .expect("Infallible serialization"),
        });
        self
    }
    /// Override the operation ID used when adding new messages
    pub fn with_operation_id(mut self, operation_id: Uuid) -> Self {
        self.operation_id = operation_id;
        self
    }
    /// Run the test
    pub async fn run(self) -> Result<TestOutput, Error> {
        let output = system_fn_fallible(self.input).await?;
        Ok(TestOutput { output })
    }
}

/// Test struct providing various assertions
pub struct TestOutput {
    output: StateFnOutput,
}

impl TestOutput {
    /// Returns true if the state function indicated that the agent
    /// should terminate.
    pub fn terminated(&self) -> bool {
        self.output.state.is_none()
    }

    /// Returns the final state of the agent. Panics if the agent
    /// terminated.
    pub fn final_dyn_state(&self) -> DynAgent {
        DefaultSerializer
            .deserialize(self.output.state.as_deref().expect("Agent terminated"))
            .expect("Failed to deserialize agent state")
    }

    /// Returns the final state of the agent. Panics if the agent
    /// terminated or is of a different type than expected.
    pub fn final_state<A: Agent>(&self) -> A {
        *self
            .final_dyn_state()
            .downcast()
            .map_err(|_| ())
            .expect("Agent had the wrong type")
    }

    /// Checks that the final agent state is equal to the passed value.
    /// Panics if this is not the case.
    pub fn assert_final_state_eq<A: Agent + PartialEq + Debug>(&self, state: &A) {
        assert_eq!(&self.final_state::<A>(), state);
    }

    /// Returns an iterator over the messages sent during evaluation of the
    /// agent's state function.
    pub fn sent_messages(&self) -> impl Iterator<Item = TestOutboundMessage> {
        self.output
            .messages
            .iter()
            .map(|message| TestOutboundMessage { message })
    }

    /// Returns the number of messages sent during evaluation of the
    /// agent's state function.
    pub fn sent_message_count(&self) -> usize {
        self.output.messages.len()
    }
}

/// A single message sent during a test
pub struct TestOutboundMessage<'a> {
    message: &'a OutboundMessage,
}

impl TestOutboundMessage<'_> {
    /// Returns the root of the recipient of this message
    pub fn recipient_root(&self) -> Root {
        Root::from_name(&self.message.recipient_root)
    }
    /// Returns the ID of the recipient of this message
    pub fn recipient_id(&self) -> Uuid {
        self.message.recipient_id
    }
    /// Returns the agent ref of the recipient of this message
    pub fn recipient_ref(&self) -> DynAgentRef {
        DynAgentRef::from_parts(self.recipient_root(), self.recipient_id())
    }
    /// Returns the ID of the operation to which this message belongs
    pub fn operation_id(&self) -> Uuid {
        self.message.operation_id
    }
    /// Returns the timestamp when this message is scheduled to be delivered
    pub fn scheduled_for(&self) -> Timestamp {
        self.message.when
    }
    /// Returns true if this message is scheduled to be delivered immediately
    pub fn immediate(&self) -> bool {
        self.message.when == Timestamp::zero()
    }
    /// Returns the content of this message.
    pub fn dyn_content(&self) -> DynMessage {
        DynMessage(self.message.content.clone())
    }

    /// Returns the content of this message. Panics if the message
    /// is of a different type than expected.
    pub fn content<M: Message>(&self) -> M {
        *self
            .dyn_content()
            .downcast()
            .map_err(|_| ())
            .expect("Message had the wrong type")
    }

    /// Checks that the message content is equal to the passed value.
    /// Panics if this is not the case.
    pub fn assert_content_eq<M: Message + PartialEq + Debug>(&self, content: &M) {
        assert_eq!(&self.content::<M>(), content);
    }
}
