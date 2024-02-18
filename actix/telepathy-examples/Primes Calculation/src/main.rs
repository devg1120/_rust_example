pub(crate) mod actors;
mod reactive_scheduling_strategy;
pub(crate) mod messages;

use actix::prelude::*;
use actix_telepathy::prelude::*;
use structopt::StructOpt;
use std::net::SocketAddr;
use std::time::Duration;

use log::*;
use crate::actors::listeners::cluster_listener::ClusterMemberListener;
use crate::actors::profiler::{Profiler, RangeMessage};


#[derive(Debug, StructOpt)]
#[structopt(name = "Role")]
enum Role {
    #[structopt(name = "main")]
    Main {
        #[structopt(short, long)]
        start: usize,

        #[structopt(short, long)]
        end: usize,

        #[structopt(short, long)]
        nodes: usize
    },

    #[structopt(name = "sub")]
    Sub {
        #[structopt(short, long)]
        mainhost: SocketAddr
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Prime Calculation", about = "Prime number calculation")]
struct Args {
    #[structopt(subcommand)]
    role: Role,

    #[structopt(short, long)]
    host: SocketAddr,

    #[structopt(short, long)]
    workers: usize,
}

#[actix_rt::main]
async fn main() {
    env_logger::init();
    let args = Args::from_args();
    debug!("{:?}", args);


    let host = args.host;
    let workers = args.workers;

    match args.role {
        Role::Main{start, end, nodes} => {
            let _ = Cluster::new(host, vec![]);
            let profiler = Profiler::new(workers, nodes).start();
            profiler.do_send(RangeMessage { start, end });
        },
        Role::Sub{mainhost} => {
            let _ = Cluster::new(host, vec![mainhost.clone()]);
            let _ = ClusterMemberListener::new(host,mainhost, workers).start();
        }
    }

    tokio::time::sleep(Duration::from_secs(5)).await;
    System::current().stop();
}
