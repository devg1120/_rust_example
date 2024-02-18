mod response_actor;
mod noise_actor;

use std::net::SocketAddr;

use actix::prelude::*;
use actix_telepathy::Cluster;
use log::debug;
use response_actor::ResponseActor;
use tokio::sync::mpsc::channel;

use clap::Parser;


#[derive(Parser)]
struct Args {
    #[clap(long)]
    pub host: SocketAddr,

    #[clap(long, default_value = None)]
    pub seed: Option<SocketAddr>
}


#[actix_rt::main]
async fn main() {
    env_logger::init();
    let args = Args::parse();
    let (tx, mut rx) = channel::<bool>(2);
    let seed_nodes = if let Some(seed) = args.seed {
        vec![seed]
    } else {
        vec![]
    };
    let _ = Cluster::new(args.host, seed_nodes);
    let _ = ResponseActor {
        own_addr: args.host,
        remote_addr: None,
        waiting_for_uuid: None,
        noise_queue: vec![],
        channel_write: tx
    }.start();

    let mut i = 0;

    while let Some(_) = rx.recv().await {
        debug!("Received message from channel");
        i += 1;
        if i == 2 {
            break;
        }
    }
    System::current().stop();
}
