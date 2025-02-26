use anyhow::Result;
use kameo::Actor;
use kameo::message::{Context, Message};

// this actor doesn't maintain any state
#[derive(Actor)]
struct HelloWorldActor;

// messages must be Send since you're sending them over asynchronous contexts
struct Greet(String);

// this is the message handling trait
impl Message<Greet> for HelloWorldActor {
    // can reply to messages
    type Reply = ();

    async fn handle(
        &mut self,
        Greet(greeting): Greet,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        println!("in actor");
        println!("{greeting}");
    }
}


#[derive(Actor)]
struct Actor2;

#[tokio::main]
async fn main() -> Result<()> {
    println!("In main");

    let actor_ref = kameo::spawn(HelloWorldActor);
    let _actor_2 = kameo::spawn(Actor2);

    actor_ref.tell(Greet("Hello World!".to_string())).await?;
    // won't compile since Actor2 doesn't implement a message handler for Greet
    // actor_2.tell(Greet("Hello World!".to_string())).await?;

    Ok(())
}
