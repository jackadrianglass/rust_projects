use std::time::Duration;

use anyhow::Result;
use kameo::actor::ActorRef;
use kameo::Actor;
use kameo::message::{Context, Message};

#[derive(Actor)]
struct Parent {
    children: Vec<ActorRef<Child>>,
}

#[derive(Actor)]
struct Child {
    name: String
}

struct BirthChild(String);

impl Message<BirthChild> for Parent {
    type Reply = ();

    async fn handle(
        &mut self,
        BirthChild(name): BirthChild,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        println!("Birthing {name}");
        self.children.push(kameo::spawn(Child{ name }));
    }
}

struct Ping;

impl Message<Ping> for Parent {
    type Reply = ();

    async fn handle(
        &mut self,
        _: Ping,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        println!("Parent got pinged");
        for child in self.children.iter() {
            child.tell(Ping{}).await.unwrap();
        }
    }
}

impl Message<Ping> for Child {
    type Reply = ();

    async fn handle(
        &mut self,
        _: Ping,
        _: Context<'_, Self, Self::Reply>,
    ) -> Self::Reply {
        println!("{} got pinged", self.name);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("In main");

    let parent = kameo::spawn(Parent { children: Vec::new() });

    println!("Sending first ping");
    parent.tell(Ping{}).await.unwrap();

    println!("Birthing Children");
    parent.tell(BirthChild("Bobby".to_string())).await.unwrap();
    parent.tell(BirthChild("Robert".to_string())).await.unwrap();

    println!("Sending second ping");
    parent.tell(Ping{}).await.unwrap();

    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}
