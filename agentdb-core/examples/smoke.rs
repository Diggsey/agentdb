use std::sync::Arc;

use agentdb_core::{id, Global, OutboundMessage, StateFnInput, StateFnOutput, Timestamp};
use foundationdb::TransactOption;
use futures::FutureExt;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

static ROOT: &str = "app";

#[derive(Serialize, Deserialize, Clone, Debug)]
struct HelloAgent {
    count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Agent {
    Hello(HelloAgent),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum Message {
    Hello(String),
}

async fn state_fn(input: StateFnInput<'_>) -> Result<StateFnOutput, ()> {
    let mut state = if let Some(state) = input.state {
        postcard::from_bytes(&state).unwrap()
    } else {
        Agent::Hello(HelloAgent { count: 0 })
    };

    match &mut state {
        Agent::Hello(hello_state) => {
            for msg in input.messages {
                let msg = postcard::from_bytes(&msg.data).unwrap();
                match msg {
                    Message::Hello(s) => {
                        hello_state.count += 1;
                        println!("Hello {}, you are number {}.", s, hello_state.count);
                    }
                }
            }
        }
    }

    Ok(StateFnOutput {
        state: Some(postcard::to_stdvec(&state).unwrap()),
        messages: Vec::new(),
        commit_hook: Box::new(|_ctx| {}),
    })
}

async fn say_hello(global: &Global, id: Uuid, from: &str) -> anyhow::Result<()> {
    Ok(global
        .db()
        .transact_boxed(
            global,
            |tx, &mut global| {
                let content = postcard::to_stdvec(&Message::Hello(from.to_string())).unwrap();
                async move {
                    agentdb_core::send_messages(
                        tx,
                        global,
                        &[OutboundMessage {
                            recipient_root: ROOT.into(),
                            recipient_id: id,
                            operation_id: id::new(),
                            when: Timestamp::now(),
                            content,
                        }],
                        0,
                    )
                    .await
                }
                .boxed()
            },
            TransactOption::idempotent(),
        )
        .await?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();
    pretty_env_logger::init();

    let _network = unsafe { foundationdb::boot() };

    let global = Global::connect(None)?;

    let agent_id1 = id::new();
    let agent_id2 = id::new();

    say_hello(&global, agent_id1, "John").await?;
    say_hello(&global, agent_id1, "Jim").await?;
    say_hello(&global, agent_id2, "Jack").await?;

    agentdb_core::run(
        agentdb_core::default_client_name(),
        global,
        ROOT.into(),
        Arc::new(|input| Box::pin(state_fn(input))),
    )
    .await?;

    Ok(())
}
