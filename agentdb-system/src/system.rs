use std::sync::Arc;

use agentdb_core::cancellation::CancellableHandle;
use agentdb_core::{Error, Global, StateFnInput, StateFnOutput};

use crate::agent::DynAgent;
use crate::context::Context;
use crate::message::deliver_unknown_message;
use crate::root::Root;
use crate::serializer::{DefaultSerializer, Serializer};
use crate::{DynAgentRef, DynMessage, Message};

pub(crate) async fn system_fn_fallible(
    mut input: StateFnInput<'_>,
) -> Result<StateFnOutput, Error> {
    let mut maybe_agent_state = if let Some(state) = &input.state {
        Some(DefaultSerializer.deserialize::<DynAgent>(state)?)
    } else {
        None
    };

    let messages = std::mem::take(&mut input.messages);
    let mut context = Context::new(&input);

    let agent_ref = DynAgentRef {
        id: input.id,
        root: context.root(),
    };

    for inbound_msg in messages {
        context.operation_id = inbound_msg.operation_id;
        match DefaultSerializer.deserialize::<Box<dyn Message>>(&inbound_msg.data) {
            Ok(msg) => {
                msg._internal_deliver(agent_ref, &mut maybe_agent_state, &mut context)
                    .await?;
            }
            Err(e) if e.to_string().starts_with("unknown variant") => {
                // Allow delivery of message types that might not be known to
                // our crate.
                deliver_unknown_message(
                    DynMessage(inbound_msg.data),
                    agent_ref,
                    &mut maybe_agent_state,
                    &mut context,
                )
                .await?;
            }
            Err(e) => return Err(e),
        }
    }

    let commit_hooks = context.commit_hooks;

    Ok(StateFnOutput {
        state: if let Some(agent_state) = maybe_agent_state {
            Some(DefaultSerializer.serialize(&agent_state)?)
        } else {
            None
        },
        messages: context.messages,
        commit_hook: Box::new(|hook_ctx| {
            for commit_hook in commit_hooks {
                commit_hook(hook_ctx.clone());
            }
        }),
    })
}

async fn system_fn(input: StateFnInput<'_>) -> Result<StateFnOutput, ()> {
    system_fn_fallible(input).await.map_err(|e| {
        log::error!("{:?}", e);
    })
}

/// Start the AgentDB client, and return a cancellable handle.
pub fn start(
    client_name: String,
    global: Arc<Global>,
    root: Root,
) -> CancellableHandle<Result<(), Error>> {
    agentdb_core::start(
        client_name,
        global,
        root.to_string(),
        Arc::new(|input| Box::pin(async move { system_fn(input).await })),
    )
}

/// Run the AgentDB client forever, or until an error is returned.
pub async fn run(client_name: String, global: Arc<Global>, root: Root) -> Result<(), Error> {
    start(client_name, global, root).await?
}
