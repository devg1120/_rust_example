use crate::interfaces::ControlA;
use crate::marshalling::{hello_event_server_marshalling, ClientMarshaller};
use crate::{control_a_server_marshalling, ControlAMsgs, HelloEvent, HelloEventMsgs};
use async_trait::async_trait;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct CompA {}

/// This is the "outer" implementation of the component, which handles
/// messaging (boilerplate code) only.
/// The actual implementation is in CompAImpl.
/// This cleanly separates infrastructure/boilerplate from the actual implementation.
/// An actual application should put this outer struct in a separate file to separate
/// from the implementation.
impl CompA {
    pub fn new(
        mut ctrl_rx: UnboundedReceiver<ControlAMsgs>,
        mut hello_rx: UnboundedReceiver<HelloEventMsgs>,
        hello_client: ClientMarshaller<HelloEventMsgs>,
    ) -> Self {
        tokio::spawn(async move {
            // That will public sync methods outside the channel-protocols for setup etc.
            let mut inner = CompAImpl {
                hello_client,
                count: 0,
            };
            loop {
                tokio::select! {
                    Some(message) = ctrl_rx.recv() => {
                        control_a_server_marshalling(message, &mut inner).await;
                    }
                    Some(message) = hello_rx.recv() => {
                        hello_event_server_marshalling(message, &mut inner).await
                    }
                    else => break,
                }
            }
        });
        Self {}
    }
}

/// This and below is the actual implementation of CompA.
/// This reads cleanly as normal code with no boilerplate
pub struct CompAImpl {
    hello_client: ClientMarshaller<HelloEventMsgs>,
    count: usize,
}

#[async_trait]
impl ControlA for CompAImpl {
    async fn say_hello(&mut self) -> usize {
        println!("Hello");
        self.hello_client.hello_from("A".to_owned());
        self.count += 1;
        self.count
    }

    fn say_world(&mut self) {
        println!("World!");
    }
}

impl HelloEvent for CompAImpl {
    fn hello_from(&mut self, sender: String) {
        println!("A: Hello from {}", sender);
    }
}
