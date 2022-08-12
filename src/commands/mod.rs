mod disconnect;
mod help;
mod id;

use self::{disconnect::Disconnect, help::Help, id::ID};
use crate::plugins::prelude::*;

/// Register default commands
pub fn register_commands() -> Vec<Box<dyn Command>> {
    vec![Box::new(Help), Box::new(Disconnect), Box::new(ID)]
}
