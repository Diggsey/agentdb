use std::sync::Arc;

use agentdb_core::cancellation::CancellableHandle;
use agentdb_core::{Error, StateFnInput, StateFnOutput};
use anyhow::anyhow;
use foundationdb::Database;

use crate::agent::DynAgent;
use crate::context::Context;
use crate::message::DynMessage;
use crate::root::Root;
use crate::serializer::{DefaultSerializer, Serializer};
use crate::DynAgentRef;

async fn system_fn_fallible(input: StateFnInput<'_>) -> Result<StateFnOutput, Error> {
    let mut maybe_agent_state = if let Some(state) = input.state {
        Some(DefaultSerializer.deserialize::<DynAgent>(&state)?)
    } else {
        None
    };

    let root = Root::from_bytes(input.root).ok_or_else(|| Error(anyhow!("Unknown root")))?;
    let mut context = Context::new(input.tx, root);

    let agent_ref = DynAgentRef { id: input.id, root };

    for msg_bytes in input.messages {
        let msg = DefaultSerializer.deserialize::<DynMessage>(&msg_bytes)?;
        msg._internal_deliver(agent_ref, &mut maybe_agent_state, &mut context)
            .await?;
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

pub fn start(
    client_name: String,
    db: Arc<Database>,
    root: Root,
) -> CancellableHandle<Result<(), Error>> {
    agentdb_core::start(
        client_name,
        db,
        root.to_bytes(),
        Arc::new(|input| Box::pin(async move { system_fn(input).await })),
    )
}

pub async fn run(client_name: String, db: Arc<Database>, root: Root) -> Result<(), Error> {
    start(client_name, db, root).await?
}
