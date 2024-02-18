use actix::prelude::*;
use actix_telepathy::prelude::*;
use std::net::SocketAddr;
use actix_broker::BrokerSubscribe;
use log::*;
use crate::messages::shutdown::ShutdownMessage;
use crate::actors::worker::Worker;


pub struct ClusterMemberListener {
    own_addr: SocketAddr,
    main_address: SocketAddr,
    local_workers: Vec<Addr<Worker>>,
    num_local_workers: usize
}

impl ClusterMemberListener {
    pub fn new(own_addr: SocketAddr, main_address: SocketAddr, num_local_workers: usize) -> Self {
        Self {
            own_addr,
            main_address,
            local_workers: vec![],
            num_local_workers
        }
    }
}

impl Actor for ClusterMemberListener {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<ClusterLog>(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        System::current().stop();
    }
}

impl Handler<ClusterLog> for ClusterMemberListener {
    type Result = ();

    fn handle(&mut self, msg: ClusterLog, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ClusterLog::NewMember(node) => {
                if self.main_address == node.socket_addr {
                    let profiler_addr = node.get_remote_addr("Profiler".to_string());
                    let own_addr = self.own_addr;

                    for i in 0..self.num_local_workers {
                        let profiler = profiler_addr.clone();
                        let worker = SyncArbiter::start(1,move || {
                            Worker::new(i, AnyAddr::Remote(profiler.clone()), Some(own_addr.clone()))
                        });
                        self.local_workers.push(worker);
                    }
                }
            },
            ClusterLog::MemberLeft(addr) => {
                debug!("member left {:?}", addr);
                if self.main_address == addr {
                    for worker in self.local_workers.iter() {
                        worker.do_send(ShutdownMessage);
                    }
                    ctx.stop();
                }
            }
        }
    }
}

impl ClusterListener for ClusterMemberListener {}
