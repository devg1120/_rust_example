use actix::prelude::{Actor, Context, Message, Addr};
use actix_telepathy::prelude::*;
use serde::{Serialize, Deserialize};
use actix::{Handler, AsyncContext, ActorContext, SyncArbiter, System};
use crate::actors::worker::Worker;
use std::collections::HashSet;
use crate::reactive_scheduling_strategy::ReactiveSchedulingStrategy;
use log::*;
use crate::messages::shutdown::ShutdownMessage;


#[derive(RemoteMessage, Serialize, Deserialize)]
pub struct Primes {
    pub request_id: usize,
    pub primes: Vec<usize>,
    pub worker_addr: AnyAddr<Worker>,
    pub is_complete: bool
}

#[derive(Message, Copy, Clone)]
#[rtype(result = "()")]
pub struct RangeMessage {
    pub start: usize,
    pub end: usize
}

#[derive(RemoteMessage, Serialize, Deserialize)]
#[with_source(source)]
pub struct RegisterMessage {
    pub(crate) source: RemoteAddr
}

#[derive(RemoteActor)]
#[remote_messages(Primes, RegisterMessage)]
pub struct Profiler {
    scheduling_strategy: Option<ReactiveSchedulingStrategy>,
    next_query_id: usize,
    num_local_workers: usize,
    num_nodes: usize,
    workers: HashSet<AnyAddr<Worker>>,
    primes: HashSet<usize>,
    range_messages: Vec<RangeMessage>
}

impl Profiler {
    pub fn new(num_local_workers: usize, num_nodes: usize) -> Self {
        Self {
            scheduling_strategy: None,
            next_query_id: 0,
            num_local_workers,
            num_nodes,
            workers: HashSet::new(),
            primes: HashSet::new(),
            range_messages: Vec::new()
        }
    }

    fn handle_range_messages(&mut self) {
        let range_messages = self.range_messages.clone();
        for range_message in range_messages.into_iter() {
            self.handle_range_message(range_message);
        }
        self.range_messages.clear();
    }

    fn handle_range_message(&mut self, msg: RangeMessage) {
        self.scheduling_strategy.as_mut().unwrap().schedule(self.next_query_id, msg.start, msg.end);
        self.next_query_id = self.next_query_id + 1;
    }

    fn has_finished(&mut self) -> bool {
        let scheduling_strategy = self.scheduling_strategy.as_ref().unwrap();
        (!scheduling_strategy.has_tasks_in_progress()) || scheduling_strategy.count_workers() < 1
    }

    fn stop_all(&mut self, ctx: &mut Context<Self>) {
        info!("Found {} primes, with {} as maximum",
              self.primes.len(),
              self.primes.iter().max().expect("There should be some primes!"));

        for worker in self.workers.iter() {
            worker.do_send(ShutdownMessage);
        }

        ctx.stop();
    }
}

impl Actor for Profiler {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.register(ctx.address().recipient());

        self.scheduling_strategy = Some(ReactiveSchedulingStrategy::new());
        for i in 0..self.num_local_workers {
            let profiler_addr = ctx.address();
            let worker = SyncArbiter::start(1, move || {
                Worker::new(i, AnyAddr::Local(profiler_addr.clone()), None)
            });
            let worker_addr = AnyAddr::Local(worker);
            self.scheduling_strategy.as_mut().unwrap().add_worker(worker_addr.clone());
            self.workers.insert(worker_addr);
        }
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        System::current().stop();
    }
}

impl Handler<RangeMessage> for Profiler {
    type Result = ();

    fn handle(&mut self, msg: RangeMessage, _ctx: &mut Self::Context) -> Self::Result {
        if self.workers.len() >= self.num_nodes * self.num_local_workers {
            self.handle_range_message(msg);
        } else {
            self.range_messages.push(msg);
        }
    }
}

impl Handler<Primes> for Profiler {
    type Result = ();

    fn handle(&mut self, msg: Primes, ctx: &mut Self::Context) -> Self::Result {
        self.primes.extend(msg.primes);

        if !msg.is_complete {
            return;
        }

        self.scheduling_strategy.as_mut().unwrap().finished(msg.request_id, msg.worker_addr);

        if self.has_finished() {
            self.stop_all(ctx);
        }
    }
}

impl Handler<RegisterMessage> for Profiler {
    type Result = ();

    fn handle(&mut self, msg: RegisterMessage, _ctx: &mut Self::Context) -> Self::Result {
        let worker = AnyAddr::Remote(msg.source);
        self.scheduling_strategy.as_mut().unwrap().add_worker(worker.clone());
        self.workers.insert(worker);
        info!("New worker, {} total workers", self.workers.len());

        if self.workers.len() >= self.num_nodes * self.num_local_workers {
            self.handle_range_messages();
        }
    }
}


impl Handler<ShutdownMessage> for Profiler {
    type Result = ();

    fn handle(&mut self, _msg: ShutdownMessage, ctx: &mut Self::Context) -> Self::Result {
        if self.has_finished() {
            self.stop_all(ctx);
        }
    }
}
