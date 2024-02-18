use actix::prelude::{Actor, Handler, Addr, Message};
use actix_telepathy::prelude::*;
use serde::{Serialize, Deserialize};
use actix::{ActorContext, SyncContext};
use std::net::SocketAddr;
use log::*;
use crate::messages::shutdown::ShutdownMessage;
use crate::actors::profiler::{Profiler, Primes, RegisterMessage};


const MAX_PRIMES_PER_MESSAGE: usize = 1000;

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddressMessage {
    pub own: SocketAddr,
    pub remote: RemoteAddr
}

#[derive(RemoteMessage, Serialize, Deserialize, Clone)]
pub struct Work {
    pub id: usize,
    pub start: usize,
    pub end: usize
}

#[derive(RemoteActor)]
#[remote_messages(Work)]
pub struct Worker {
    id: usize,
    addr: Option<AnyAddr<Worker>>,
    profiler: AnyAddr<Profiler>,
    own_socket: Option<SocketAddr>
}

impl Worker {
    pub fn new(id: usize, profiler: AnyAddr<Profiler>, own_socket: Option<SocketAddr>) -> Self {
        Self {
            id,
            addr: None,
            profiler,
            own_socket
        }
    }
}

impl Actor for Worker {
    type Context = SyncContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.register(ctx.address().recipient());

        match &self.profiler {
            AnyAddr::Local(_) => { self.addr = Some(AnyAddr::Local(ctx.address())) },
            AnyAddr::Remote(_) => {
                let remote_addr = RemoteAddr::new_from_id(
                    self.own_socket.unwrap(),
                    "Worker"
                );
                self.addr = Some(AnyAddr::Remote(remote_addr.clone()));
                self.profiler.do_send(RegisterMessage { source: remote_addr });
            }
        }
    }
}

impl Handler<Work> for Worker {
    type Result = ();

    fn handle(&mut self, msg: Work, _ctx: &mut Self::Context) -> Self::Result {
        info!("Worker-{}: Started discovering primes in [{},{}] ...", self.id, msg.start, msg.end);

        let own_addr = self.addr.as_ref().expect("AnyAddr should be set");
        let request_id = msg.id;
        let mut prime_buffer = vec![];

        for i in msg.start..msg.end+1 {
            if is_prime(i) {
                if prime_buffer.len() >= MAX_PRIMES_PER_MESSAGE {
                    self.profiler.do_send(
                        Primes {
                            request_id,
                            primes: prime_buffer.clone(),
                            worker_addr: own_addr.clone(),
                            is_complete: false
                        }
                    );
                    prime_buffer.clear();
                }
                prime_buffer.push(i);
            }
        }

        let answer = Primes { request_id, primes: prime_buffer, worker_addr: own_addr.clone(), is_complete: true};
        self.profiler.do_send(answer);
    }
}

impl Handler<ShutdownMessage> for Worker {
    type Result = ();

    fn handle(&mut self, _msg: ShutdownMessage, ctx: &mut Self::Context) -> Self::Result {
        info!("shutdown");
        ctx.stop();
    }
}


fn is_prime(n: usize) -> bool {
    if n == 1 {
        return false;
    }

    // Check for the most basic primes
    if n == 2 || n == 3 {
        return true;
    }

    // Check if n is an even number
    if n % 2 == 0 {
        return false;
    }

    // Check the odds
    let mut i = 3;
    loop {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            return false;
        }
        i = i + 2;
    }

    return true;
}
