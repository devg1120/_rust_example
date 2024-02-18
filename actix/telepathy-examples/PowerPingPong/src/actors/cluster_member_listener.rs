use actix::prelude::*;
use actix_telepathy::prelude::*;
use std::net::SocketAddr;
use actix_broker::BrokerSubscribe;
use log::*;
use crate::actors::Multiplier;
use crate::actors::multiplier::StartExperimentMessage;


pub struct ClusterMemberListener {
    own_addr: SocketAddr,
    multiplier: Addr<Multiplier>
}

impl ClusterMemberListener {
    pub fn new(own_addr: SocketAddr, multiplier: Addr<Multiplier>) -> Self {
        Self {
            own_addr,
            multiplier
        }
    }
}

impl Actor for ClusterMemberListener {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<ClusterLog>(ctx);
        debug!("ClusterMemberListener started");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        System::current().stop();
    }
}

impl Handler<ClusterLog> for ClusterMemberListener {
    type Result = ();

    fn handle(&mut self, msg: ClusterLog, _ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ClusterLog::NewMember(node) => {
                if self.own_addr != node.socket_addr {
                    let multiplier_addr = node.get_remote_addr(Multiplier::ACTOR_ID.to_string());
                    self.multiplier.do_send(StartExperimentMessage { power_ping_pong_opponent: multiplier_addr });
                }
            },
            ClusterLog::MemberLeft(addr) => {
                debug!("member left {:?}", addr);
            }
        }
    }
}

impl ClusterListener for ClusterMemberListener {}
