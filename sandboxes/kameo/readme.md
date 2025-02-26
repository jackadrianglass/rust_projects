# Kameo

Learning through [this documentation](https://docs.page/tqwewe/kameo/)

# Core concepts

Actors: Encapsulates state and behaviour into independent concurrent units.
This is how concurrent state gets isolated from each other which allows each
individual actor to crash & recover without affecting the state of the rest of
the system

Messages: Communication between actors is done through messages. These messages
are delivered in an asynchronous manner. There are two patterns
    - "tell": Send a message without expecting a response
    - "ask": Send a message expecting a response
Requests are the spatial form of the messages. Replies are responses to requests

Supervisor: All actors are managed by a runtime that manages fault recovery. Actors
typically employ the "let it crash" philosophy so that parent actors can monitor and
respond to the failures

# Actor Trait

Key components of an actor:
- Lifecycle hooks: Intervention points where custom behaviour can be implemented (`on_start`,
  `on_stop`, `on_panic`, `on_link_died`)
- Mailbox: The incoming messages are queued or bounced here which provides back pressure if
  the system is receiving too many messages
- Messaging: Can send messages between actors through the `ActorRef` which is created when
  the actor is spawned
- Supervision: Actors can supervise other actors which allows for hierarchical error handling
  and recovery strategies

There are some proc_macros for removing a lot of the boiler plate code involved in creating
the actor

```rs
use kameo::Actor;

#[derive(Actor)]
#[actor(name = "MyAmazingActor", mailbox = bounded(64))]
struct MyActor { }
```

# Messages

Messages are processed one at a time, sequentially. Therefore, there is no need for synchronization
mechanisms to access data within the actor. Messages are also processed in the order that they are
received

## Ask Requests

The asker waits for a response from the receiver. Useful when getting information from the receiver or
when confirmation that the action has been completed when requested

- Reply is awaited
- Errors are handled by the asker
- Timeouts are available for both `mailbox_timeout` and for `reply_timeout`

## Tell Requests

The tell request is a "fire and forget" strategy

- No reply to the teller
- Error handling is handled through a panic (can customize through the `on_panic` hook)
- Only has the `mailbox_timeout` since there's no reply

See [this table](https://docs.page/tqwewe/kameo/core-concepts/requests#request-methods) for
the methods of requests in relation to bounded versus unbounded mailboxes

# Linking

Actors can be linked together through the supervision tree through the `link` and `unlink`
methods. Actors that are linked can respond to failure events from their linked actors
such as restarting the deceased actor
