use std::time::SystemTime;
use actix::prelude::*;
use actix_telepathy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::custom_serializer::BinCodeSerializer;
use log::*;


#[derive(RemoteMessage, Serialize, Deserialize)]
#[with_source(source)]
pub struct PingPongMessage {
    pub send_time: SystemTime,
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
    pub step: usize,
    pub source: RemoteAddr
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct StartExperimentMessage {
    pub power_ping_pong_opponent: RemoteAddr
}


#[derive(RemoteActor)]
#[remote_messages(PingPongMessage)]
pub struct Multiplier {
    pub steps: usize,
    pub own_addr: RemoteAddr
}


impl Multiplier {
    pub const ACTOR_ID: &'static str = "Multiplier";

    fn random_bytes(&mut self, length: usize) -> Vec<u8> {
        let random_bytes: Vec<u8> = (0..length).map(|_| { rand::random::<u8>() }).collect();
        random_bytes
    }
}


impl Actor for Multiplier {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.register(ctx.address().recipient());
        debug!("Multiplier started");
    }
}


impl Handler<PingPongMessage> for Multiplier {
    type Result = ();

    fn handle(&mut self, msg: PingPongMessage, _ctx: &mut Self::Context) -> Self::Result {
        let now= SystemTime::now();
        let duration = now.duration_since(msg.send_time).expect("Time went backwards");
        let step = msg.step;
        let next_step = step + 1;

        info!("Step {}: {} | {}", step, duration.as_millis(), msg.content.len());

        if next_step >= self.steps {
            info!("Done");
            return;
        }
        let content = self.random_bytes(msg.content.len() * 2);

        let now= SystemTime::now();
        let answer = PingPongMessage {send_time: now, content, step: next_step, source: self.own_addr.clone() };
        msg.source.do_send(answer);
    }
}


impl Handler<StartExperimentMessage> for Multiplier {
    type Result = ();

    fn handle(&mut self, msg: StartExperimentMessage, _ctx: &mut Self::Context) -> Self::Result {
        let content = self.random_bytes(4);
        let send_time= SystemTime::now();
        let serve = PingPongMessage {
            send_time,
            content,
            step: 0,
            source: self.own_addr.clone()
        };

        msg.power_ping_pong_opponent.do_send(serve);
    }
}
