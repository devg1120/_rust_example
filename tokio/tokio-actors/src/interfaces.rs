use async_trait::async_trait;

#[async_trait]
pub trait ControlA {
    async fn say_hello(&mut self) -> usize;

    // Note that calls where we don't need to wait for the actor's task to process
    // do not have to be marked async:
    fn say_world(&mut self);
}

pub trait HelloEvent {
    fn hello_from(&mut self, sender: String);
}
