use actix::prelude::*;
use tokio::sync::mpsc::Sender;

use crate::response_actor::ResponseActor;


#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct NoiseMessage {}


pub struct NoiseActor {
    pub response_actor_addr: Addr<ResponseActor>,
    pub channel_write: Sender<bool>
}

impl Actor for NoiseActor {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let noise_message = NoiseMessage {};
        for _ in 0..10 {
            self.response_actor_addr.do_send(noise_message.clone());
        }

        let channel_write = self.channel_write.clone();

        tokio::spawn(async move {
            channel_write.send(true).await.unwrap();
        });
        
    }
}
