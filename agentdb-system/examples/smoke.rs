use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use agentdb_system::*;

#[derive(Serialize, Deserialize)]
struct MyAgent;

declare_agent!("my_agent" => MyAgent);

#[derive(Serialize, Deserialize)]
struct MyMessage;

declare_message!("my_message" => MyMessage);

declare_constructor!(MyMessage);

#[async_trait]
impl Construct for MyMessage {
    async fn construct(
        self,
        _ref_: DynAgentRef,
        _context: &mut Context<'_>,
    ) -> Result<Option<DynAgent>, Error> {
        Ok(Some(Box::new(MyAgent)))
    }
}

declare_handler!(MyAgent[MyMessage]);

#[async_trait]
impl Handle<MyMessage> for MyAgent {
    async fn handle(
        &mut self,
        _ref_: AgentRef<Self>,
        _message: MyMessage,
        _context: &mut Context,
    ) -> Result<bool, Error> {
        println!("Hello, world!");
        Ok(false)
    }
}

fn main() {}
