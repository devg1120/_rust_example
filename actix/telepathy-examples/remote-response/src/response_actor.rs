use std::net::SocketAddr;

use actix_broker::BrokerSubscribe;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;
use actix::prelude::*;
use actix_telepathy::prelude::*;

use crate::noise_actor::NoiseActor;
use crate::noise_actor::NoiseMessage;

#[derive(RemoteMessage, Serialize, Deserialize)]

pub struct RequestMessage {
    pub uuid: Uuid,
}

#[derive(RemoteMessage, Serialize, Deserialize)]
pub struct ResponseMessage {
    pub uuid: Uuid,
}

#[derive(RemoteActor)]
#[remote_messages(RequestMessage, ResponseMessage)]
pub struct ResponseActor {
    pub own_addr: SocketAddr,
    pub remote_addr: Option<RemoteAddr>,
    pub waiting_for_uuid: Option<Uuid>,
    pub noise_queue: Vec<NoiseMessage>,
    pub channel_write: Sender<bool>
}

impl Actor for ResponseActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.register(ctx.address().recipient());
        self.subscribe_system_async::<ClusterLog>(ctx);
    }
}

impl Handler<RequestMessage> for ResponseActor {
    type Result = ();

    fn handle(&mut self, msg: RequestMessage, _ctx: &mut Self::Context) -> Self::Result {
        debug!("RequestMessage received");
        let response_message = ResponseMessage {
            uuid: msg.uuid,
        };
        self.remote_addr.as_ref().unwrap().do_send(response_message);
    }
}

impl Handler<ResponseMessage> for ResponseActor {
    type Result = ();

    fn handle(&mut self, msg: ResponseMessage, ctx: &mut Self::Context) -> Self::Result {
        if let Some(uuid) = self.waiting_for_uuid {
            if uuid == msg.uuid {
                debug!("Got response for uuid: {}", uuid);
                self.waiting_for_uuid = None;
                while let Some(noise) = self.noise_queue.pop() {
                    ctx.address().do_send(noise);
                }
                let channel_write = self.channel_write.clone();

                tokio::spawn(async move {
                    channel_write.send(true).await.unwrap();
                });
                return ();
            }
            warn!("Got response for uuid: {}, but we are waiting for a different uuid", msg.uuid);
        }
        warn!("Got response for uuid: {}, but we are not waiting for a response", msg.uuid);
    }
}

impl Handler<NoiseMessage> for ResponseActor {
    type Result = ();

    fn handle(&mut self, msg: NoiseMessage, _ctx: &mut Self::Context) -> Self::Result {
        if self.waiting_for_uuid.is_none() {
            debug!("NoiseMessage received");
        } else {
            debug!("NoiseMessage received, but we are waiting for a response");
            self.noise_queue.push(msg);
        }
    }
}

impl Handler<ClusterLog> for ResponseActor {
    type Result = ();

    fn handle(&mut self, msg: ClusterLog, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ClusterLog::NewMember(node) => {
                if self.own_addr != node.socket_addr {
                    let remote_addr = node.get_remote_addr(Self::ACTOR_ID.to_string());
                    self.remote_addr = Some(remote_addr.clone());
                    let uuid = Uuid::new_v4();
                    remote_addr.do_send(RequestMessage { uuid: uuid.clone() });
                    self.waiting_for_uuid = Some(uuid);
                    NoiseActor { response_actor_addr: ctx.address(), channel_write: self.channel_write.clone() }.start();
                }
            },
            ClusterLog::MemberLeft(addr) => {
                debug!("member left {:?}", addr);
            }
        }
    }
}

impl ClusterListener for ResponseActor {}