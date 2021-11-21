use agentdb_agents::effects::EffectFailure;
use serde::{Deserialize, Serialize};

use agentdb_agents::effects::callback::{EffectCallback, EffectContext};
use agentdb_agents::effects::retry::DoRetry;
use agentdb_system::*;
use tokio::io::AsyncReadExt;

declare_root!("example_retry" => MY_ROOT);

#[derive(Serialize, Deserialize)]
struct MyEffect;

#[typetag::serde(name = "my_effect")]
impl EffectCallback for MyEffect {
    fn call(&self, context: EffectContext) {
        tokio::spawn(async move {
            println!("Press enter to ack!");
            tokio::io::stdin().read(&mut [0]).await.unwrap();
            context.send_response(MyMessage).await.unwrap();
        });
    }
}

#[agent(name = "my_agent")]
#[derive(Serialize, Deserialize)]
struct MyAgent;

#[message(name = "my_message")]
#[derive(Serialize, Deserialize)]
struct MyMessage;

#[message(name = "my_constructor")]
#[derive(Serialize, Deserialize)]
struct MyConstructor;

#[constructor]
impl Construct for MyConstructor {
    type Agent = MyAgent;
    async fn construct(
        self,
        ref_: AgentRef<MyAgent>,
        context: &mut Context<'_>,
    ) -> Result<Option<MyAgent>, Error> {
        context.construct(
            MY_ROOT,
            DoRetry::new(ref_, MyEffect).with_max_attempts(Some(3)),
        )?;
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

#[handler]
impl Handle<EffectFailure> for MyAgent {
    async fn handle(
        &mut self,
        _ref_: AgentRef<Self>,
        message: EffectFailure,
        _context: &mut Context,
    ) -> Result<bool, Error> {
        println!("Failed to get a response! ({:?})", message.reason());
        Ok(false)
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = dotenv::dotenv();
    pretty_env_logger::init();

    let _network = unsafe { foundationdb::boot() };

    let global = Global::connect(None)?;

    let mut ctx = ExternalContext::new();
    ctx.construct(MY_ROOT, MyConstructor)?;
    ctx.run(&global).await?;

    run(default_client_name(), global, MY_ROOT).await
}
