use std::{sync::Arc, time::Duration};

use foundationdb::Database;
use serde::{Deserialize, Serialize};

use agentdb_system::*;

declare_root!("my_root" => MY_ROOT);

#[agent(name = "my_agent")]
#[derive(Serialize, Deserialize)]
struct MyAgent;

#[message(name = "my_message")]
#[derive(Serialize, Deserialize)]
struct MyMessage;

#[constructor]
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

#[handler]
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
    let agent_ref = ctx.construct(MY_ROOT, MyMessage)?;
    ctx.send(agent_ref, MyMessage)?;
    ctx.send_at(
        agent_ref,
        MyMessage,
        Timestamp::now() + Duration::from_secs(5),
    )?;
    ctx.run_with_db(&db).await?;

    run(default_client_name(), db, MY_ROOT).await
}
