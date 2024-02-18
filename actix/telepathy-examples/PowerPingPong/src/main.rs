mod actors;
mod custom_serializer;

use structopt::StructOpt;
use std::{net::SocketAddr, time::Duration};
use log::*;
use actix::prelude::*;
use actix_telepathy::prelude::*;
use crate::actors::{Multiplier, ClusterMemberListener};
pub use custom_serializer::BinCodeSerializer;

#[derive(Debug, StructOpt)]
#[structopt(name = "Role")]
enum Role {
    #[structopt(name = "main")]
    Main {},

    #[structopt(name = "sub")]
    Sub {
        #[structopt(short, long)]
        mainhost: SocketAddr
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Power Ping Pong", about = "Power Ping Pong")]
struct Args {
    #[structopt(subcommand)]
    role: Role,

    #[structopt(short, long)]
    host: SocketAddr,

    #[structopt(short, long)]
    steps: usize
}


#[actix_rt::main]
async fn main() {
    env_logger::init();
    let args = Args::from_args();
    debug!("{:?}", args);

    let host = args.host;
    let own_addr = RemoteAddr::new_from_id(host, Multiplier::ACTOR_ID);
    let steps = args.steps;

    match args.role {
        Role::Main {} => {
            let _cluster = Cluster::new(host, vec![]);
            let multiplier = Multiplier { steps, own_addr }.start();
            let _cluster_listener = ClusterMemberListener::new(host, multiplier).start();
        },
        Role::Sub { mainhost } => {
            let _cluster = Cluster::new(host, vec![mainhost]);
            let _multiplier = Multiplier { steps, own_addr }.start();
        }
    }

    tokio::time::sleep(Duration::from_secs(10)).await;
    System::current().stop();
}
