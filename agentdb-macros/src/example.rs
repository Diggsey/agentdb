#[derive(Message)]
struct MyMessage {}

#[derive(Message)]
struct MyConstructor {}

#[agentdb]
impl Construct for MyConstructor {}

#[derive(Agent)]
struct MyAgent {}

#[agentdb]
impl Handle<MyMessage> for MyAgent {}

#[agentdb]
impl Destruct for MyAgent {}
