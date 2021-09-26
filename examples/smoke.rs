use std::sync::Arc;

use agentdb::{MessageToSend, StateFnInput, StateFnOutput};
use chrono::Utc;
use foundationdb::{Database, TransactOption};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

static ROOT: &[u8] = b"app";

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

fn state_fn(input: StateFnInput) -> StateFnOutput {
    let mut state = if let Some(state) = input.state {
        postcard::from_bytes(&state).unwrap()
    } else {
        Agent::Hello(HelloAgent { count: 0 })
    };

    match &mut state {
        Agent::Hello(hello_state) => {
            for msg in input.messages {
                let msg = postcard::from_bytes(&msg).unwrap();
                match msg {
                    Message::Hello(s) => {
                        hello_state.count += 1;
                        println!("Hello {}, you are number {}.", s, hello_state.count);
                    }
                }
            }
        }
    }

    StateFnOutput {
        state: Some(postcard::to_stdvec(&state).unwrap()),
        messages: Vec::new(),
        commit_hook: Box::new(|_ctx| {}),
    }
}

async fn say_hello(db: &Database, id: Uuid, from: &str) -> anyhow::Result<()> {
    Ok(db
        .transact_boxed_local(
            (),
            |tx, _| {
                let content = postcard::to_stdvec(&Message::Hello(from.to_string())).unwrap();
                Box::pin(async move {
                    agentdb::send_messages(
                        tx,
                        &[MessageToSend {
                            recipient_root: ROOT.into(),
                            recipient_id: id,
                            when: Utc::now(),
                            content,
                        }],
                        0,
                    )
                    .await
                })
            },
            TransactOption::default(),
        )
        .await?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();
    pretty_env_logger::init();

    let _network = unsafe { foundationdb::boot() };

    let db = Arc::new(Database::default()?);

    let agent_id1 = Uuid::new_v4();
    let agent_id2 = Uuid::new_v4();

    say_hello(&db, agent_id1, "John").await?;
    say_hello(&db, agent_id1, "Jim").await?;
    say_hello(&db, agent_id2, "Jack").await?;

    agentdb::run(db, ROOT.to_vec(), Arc::new(state_fn)).await;

    Ok(())
}
