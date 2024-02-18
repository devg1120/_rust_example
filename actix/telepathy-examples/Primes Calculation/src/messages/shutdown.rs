use actix::{Message, Addr};
use actix_telepathy::{RemoteMessage, DefaultSerialization, NetworkInterface};
use serde::{Serialize, Deserialize};


#[derive(RemoteMessage, Serialize, Deserialize)]
pub struct ShutdownMessage;
