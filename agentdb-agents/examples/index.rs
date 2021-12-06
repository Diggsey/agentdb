use agentdb_agents::data_models::agent_index::{self, AgentIndex};
use serde::{Deserialize, Serialize};

use agentdb_system::*;

declare_root!("example_index" => MY_ROOT);

// Declare an index at a fixed UUID
const MY_INDEX: AgentRef<AgentIndex> = MY_ROOT.const_ref(0x56fcf5be_7c04_47f3_a004_d25b799e0c42);

#[agent(name = "my_agent")]
#[derive(Serialize, Deserialize)]
struct MyAgent {
    value: String,
}

#[message(name = "my_message")]
#[derive(Serialize, Deserialize)]
struct MyMessage {
    value: String,
}

#[constructor]
impl Construct for MyMessage {
    type Agent = MyAgent;
    async fn construct(
        self,
        ref_: AgentRef<MyAgent>,
        context: &mut Context,
    ) -> Result<Option<MyAgent>, Error> {
        context.send(
            MY_INDEX,
            agent_index::Update::add(Prepacked::new(&self.value), ref_.into()),
        )?;
        Ok(Some(MyAgent { value: self.value }))
    }
}

#[handler]
impl Handle<MyMessage> for MyAgent {
    async fn handle(
        &mut self,
        ref_: AgentRef<Self>,
        msg: MyMessage,
        context: &mut Context,
    ) -> Result<bool, Error> {
        let old_value = std::mem::replace(&mut self.value, msg.value);
        context.send(
            MY_INDEX,
            agent_index::Update::change(
                Prepacked::new(&old_value),
                Prepacked::new(&self.value),
                ref_.into(),
            ),
        )?;
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
    for i in 0..100 {
        ctx.construct(
            MY_ROOT,
            MyMessage {
                value: i.to_string(),
            },
        )?;
    }
    ctx.run(&global).await?;

    run(default_client_name(), global, MY_ROOT).await
}
