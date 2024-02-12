//! The message types here could be generated from the traits.
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum ControlAMsgs {
    SayHello { response_tx: Sender<usize> },
    SayWorld,
}

pub enum HelloEventMsgs {
    HelloFrom { sender: String },
}
