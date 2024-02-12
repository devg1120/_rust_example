use crate::interfaces::HelloEvent;
use crate::marshalling::{hello_event_server_marshalling, ClientMarshaller};
use crate::messages::HelloEventMsgs;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct CompB {}

impl CompB {
    pub fn new(
        mut rx: UnboundedReceiver<HelloEventMsgs>,
        hello_client: ClientMarshaller<HelloEventMsgs>,
    ) -> Self {
        tokio::spawn(async move {
            // That will public sync methods outside the channel-protocols for setup etc.
            let mut inner = CompBImpl { hello_client };
            while let Some(message) = rx.recv().await {
                hello_event_server_marshalling(message, &mut inner).await;
            }
        });
        Self {}
    }
}

pub struct CompBImpl {
    hello_client: ClientMarshaller<HelloEventMsgs>,
}

impl HelloEvent for CompBImpl {
    fn hello_from(&mut self, sender: String) {
        println!("B: Hello from {}", sender);
        self.hello_client.hello_from("B".to_owned());
    }
}
