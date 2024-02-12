mod comp_a;
mod comp_b;
mod interfaces;
mod marshalling;
mod messages;

use crate::comp_b::CompB;
use crate::interfaces::{ControlA, HelloEvent};
use crate::marshalling::connection;
use crate::messages::{ControlAMsgs, HelloEventMsgs};
use comp_a::CompA;
use marshalling::control_a_server_marshalling;
use std::time;

#[tokio::main]
async fn main() {
    let (hello_client_for_b, b_hello_server_rx) = connection::<HelloEventMsgs>();
    let (hello_client_for_a, a_hello_server_rx) = connection::<HelloEventMsgs>();
    let (mut ctrl_client_for_a, a_ctrl_server_rx) = connection::<ControlAMsgs>();
    let _a = CompA::new(a_ctrl_server_rx, a_hello_server_rx, hello_client_for_b);
    let _b = CompB::new(b_hello_server_rx, hello_client_for_a);

    ctrl_client_for_a.say_hello().await;
    let count = ctrl_client_for_a.say_hello().await;
    ctrl_client_for_a.say_world();

    tokio::time::sleep(time::Duration::from_millis(500)).await;
    println!("Count is {}", count);
}
