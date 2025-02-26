use std::time::Duration;

use anyhow::Result;
use futures::{StreamExt, stream};
use kameo::Actor;
use kameo::message::{Context, Message};
use tokio::task::JoinHandle;
use tokio::time;

// this actor doesn't maintain any state
#[derive(Actor)]
struct ActorA {
    name: String,
    timer: Option<JoinHandle<()>>,
}

// messages must be Send since you're sending them over asynchronous contexts
struct Greet(String);

// this is the message handling trait
impl Message<Greet> for ActorA {
    // can reply to messages
    type Reply = ();

    async fn handle(
        &mut self,
        Greet(greeting): Greet,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        let name = self.name.clone();
        println!("{name} got a message from {greeting}");
    }
}

struct MessageThisGuy(kameo::actor::ActorRef<ActorA>, tokio::time::Interval);

impl Message<MessageThisGuy> for ActorA {
    // can reply to messages
    type Reply = ();

    async fn handle(
        &mut self,
        MessageThisGuy(buddy, interval): MessageThisGuy,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        if let Some(ref timer) = self.timer {
            timer.abort();
        }

        let name = self.name.clone();

        self.timer = Some(tokio::spawn(async move {
            let forever = stream::unfold(interval, |mut interval| async {
                interval.tick().await;
                Some(((), interval))
            });

            forever
                .for_each(|_| async {
                    buddy
                        .tell(Greet(name.clone()))
                        .await
                        .expect("nothing but success");
                })
                .await;
        }));
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("In main");

    let actor_1 = kameo::spawn(ActorA { name: "Gertrude".to_string(), timer: None });
    let actor_2 = kameo::spawn(ActorA { name: "Bernard".to_string(), timer: None });

    actor_1
        .tell(MessageThisGuy(
            actor_2.clone(),
            time::interval(Duration::from_secs(4)),
        ))
        .await
        .unwrap();
    actor_2
        .tell(MessageThisGuy(
            actor_1.clone(),
            time::interval(Duration::from_secs(5)),
        ))
        .await
        .unwrap();

    tokio::time::sleep(Duration::from_secs(60)).await;

    Ok(())
}
