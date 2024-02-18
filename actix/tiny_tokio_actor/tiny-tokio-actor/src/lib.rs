//! A Simple Tiny Tokio Actor Crate
//!
//! This crate provides a minimally functioning actor system with a common
//! event bus. Tokio unbounded channels are used for the mailbox of the
//! actors, and actor behaviour (defined through the [`Handler`] trait) can
//! use the request+response pattern (using tokio oneshot channel for
//! responses). You an send messages to actors either through a `tell`
//! where the method does not provide a response, or an `ask`. The `ask`
//! method does provide a response
//!
//! Example
//! ```rust,no_run
//! use tiny_tokio_actor::*;
//!
//! // The event message you may want to publish to the system event bus.
//! #[derive(Clone, Debug)]
//! struct TestEvent(String);
//!
//! // Mark the struct as a system event message.
//! impl SystemEvent for TestEvent {}
//!
//! // The actor struct must be Send + Sync but need not be Clone
//! #[derive(Default)]
//! struct TestActor {
//!     counter: usize
//! }
//!
//! // Mark the struct as an actor. Note that you can optionally override
//! // some default methods here like `timeout()` and `supervision_stragegy()`.
//! // See the [`Actor`] trait for details.
//! impl Actor<TestEvent> for TestActor {}
//!
//! // The message the actor will expect. It must derive Clone.
//! // Debug is not required.
//! #[derive(Clone, Debug)]
//! struct TestMessage(String);
//!
//! // Mark the message struct as an actor message. Note that we
//! // also define the response we expect back from this message.
//! // If no response is desired, just use `()`.
//! impl Message for TestMessage {
//!     type Response = String;
//! }
//!
//! // Define the behaviour of the actor. Note that the `handle` method
//! // has a `String` return type because that is what we defined the
//! // Response to be of `TestMessage`. As the method is async, we have
//! // to annotate the implementation with the `async_trait` macro (a
//! // re-export of the `async_trait` crate).
//! #[async_trait]
//! impl Handler<TestEvent, TestMessage> for TestActor {
//!     async fn handle(&mut self, msg: TestMessage, ctx: &mut ActorContext<TestEvent>) -> String {
//!         self.counter += 1;
//!         ctx.system.publish(TestEvent(format!("message received by '{}'", ctx.path)));
//!         "Ping!".to_string()
//!     }
//! }
//!
//! #[tokio::main]
//! pub async fn main() -> Result<(), ActorError> {
//!
//!     // Create the actor
//!     let actor = TestActor { counter: 0 };
//!     // Create the message we will send
//!     let msg = TestMessage("hello world!".to_string());
//!
//!     // Create the system event bus
//!     let bus = EventBus::<TestEvent>::new(1000);
//!     // Create the actor system with the event bus
//!     let system = ActorSystem::new("test", bus);
//!     // Launch the actor on the actor system
//!     let actor_ref = system.create_actor("test-actor", actor).await?;
//!
//!     // Listen for events on the system event bus
//!     let mut events: EventReceiver<TestEvent> = system.events();
//!     tokio::spawn(async move {
//!         loop {
//!             match events.recv().await {
//!                 Ok(event) => println!("Received event! {:?}", event),
//!                 Err(err) => println!("Error receivng event!!! {:?}", err)
//!             }
//!         }
//!     });
//!
//!     // Wait a little for the actor to start up
//!     tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
//!
//!     // Send the actor the message through an `ask` from which we will
//!     // get a response
//!     let response = actor_ref.ask(msg).await?;
//!     println!("Response: {}", response);
//!     Ok(())
//! }
//! ```

mod actor;
mod bus;
mod system;

pub use actor::{
    supervision::{RetryStrategy, SupervisionStrategy},
    Actor, ActorContext, ActorError, ActorPath, ActorRef, Handler, Message,
};
pub mod supervision {
    //! Actor Supervision Strategies
    //!
    //! To supervise actor startup, you can choose to just have the actor Stop,
    //! or set a [`super::RetryStrategy`]. There are three built-in strategies you can
    //! use: [`NoIntervalStrategy`], [`FixedIntervalStrategy`],
    //! and [`ExponentialBackoffStrategy`].
    //!
    //! You can also opt to create your own strategy by implementing the provided
    //! [`super::RetryStrategy`] trait.
    pub use crate::actor::supervision::{
        ExponentialBackoffStrategy, FixedIntervalStrategy, NoIntervalStrategy,
    };
}
pub use bus::{EventBus, EventReceiver};
pub use system::{ActorSystem, SystemEvent};

pub use async_trait::async_trait;
pub use tokio::time::Duration;