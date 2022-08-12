use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

use crate::tcp::Client;

pub mod commands;
pub mod plugins;
pub mod tcp;

lazy_static! {
    pub static ref CLIENTS: Mutex<HashMap<usize, Client>> = Mutex::new(HashMap::new());
    pub static ref CLIENT_NEXT: Mutex<usize> = Mutex::new(0);
}
