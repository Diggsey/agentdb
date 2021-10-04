use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use foundationdb::Database;
use serde::{Deserialize, Serialize};

use agentdb_system::*;

declare_root!("my_root" => MY_ROOT);

#[derive(Serialize, Deserialize)]
struct MyAgent;

declare_agent!("my_agent" => MyAgent);

#[derive(Serialize, Deserialize)]
struct MyMessage;

declare_message!("my_message" => MyMessage);

declare_constructor!(MyMessage);

#[async_trait]
impl Construct for MyMessage {
    type Agent = MyAgent;
    async fn construct(
        self,
        _ref_: AgentRef<MyAgent>,
        _context: &mut Context<'_>,
    ) -> Result<Option<MyAgent>, Error> {
        Ok(Some(MyAgent))
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenv::dotenv();
    pretty_env_logger::init();

    let _network = unsafe { foundationdb::boot() };

    let db = Arc::new(Database::default()?);

    let mut ctx = ExternalContext::new();
    let agent_ref = ctx.construct(MY_ROOT, MyMessage, Timestamp::zero())?;
    ctx.send(agent_ref, MyMessage, Timestamp::zero())?;
    ctx.send(
        agent_ref,
        MyMessage,
        Timestamp::now() + Duration::from_secs(5),
    )?;
    ctx.run_with_db(&db).await?;

    run(db, MY_ROOT).await
}
