use crate::interfaces::{ControlA, HelloEvent};
use crate::messages::{ControlAMsgs, HelloEventMsgs};
use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;

pub fn connection<T>() -> (ClientMarshaller<T>, UnboundedReceiver<T>) {
    let (tx, rx) = mpsc::unbounded_channel::<T>();
    (ClientMarshaller::<T> { tx }, rx)
}

pub struct ClientMarshaller<T> {
    tx: UnboundedSender<T>,
}

/// This could be generated
#[async_trait]
impl ControlA for ClientMarshaller<ControlAMsgs> {
    async fn say_hello(&mut self) -> usize {
        let (response_tx, response_rx) = oneshot::channel::<usize>();

        // Both of these will fail if the server is gone, so we only handle the error once:
        let _ = self.tx.send(ControlAMsgs::SayHello { response_tx });
        response_rx.await.expect("Actor is gone")
    }

    fn say_world(&mut self) {
        let _ = self.tx.send(ControlAMsgs::SayWorld {});
    }
}

/// This could be generated
pub async fn control_a_server_marshalling<T: ControlA>(msg: ControlAMsgs, target: &mut T) {
    match msg {
        ControlAMsgs::SayHello { response_tx } => {
            response_tx.send(target.say_hello().await).unwrap();
        }
        ControlAMsgs::SayWorld => {
            target.say_world();
        }
    }
}

/// This could be generated
impl HelloEvent for ClientMarshaller<HelloEventMsgs> {
    fn hello_from(&mut self, sender: String) {
        let _ = self.tx.send(HelloEventMsgs::HelloFrom { sender });
    }
}

/// This could be generated
pub async fn hello_event_server_marshalling<T: HelloEvent>(msg: HelloEventMsgs, target: &mut T) {
    match msg {
        HelloEventMsgs::HelloFrom { sender } => {
            target.hello_from(sender);
        }
    }
}
