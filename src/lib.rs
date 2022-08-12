use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

use crate::server::Client;

pub mod commands;
pub mod plugins;
pub mod server;

lazy_static! {
    /// List with all connected clients
    pub static ref CLIENTS: Mutex<HashMap<usize, Client>> = Mutex::new(HashMap::new());
    /// Next ID of the client to be add to [CLIENTS]
    pub static ref CLIENT_NEXT: Mutex<usize> = Mutex::new(0);
}
