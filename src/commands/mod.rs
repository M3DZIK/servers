mod broadcast;
mod disconnect;
mod help;
mod id;

use self::{broadcast::Broadcast, disconnect::Disconnect, help::Help, id::Id};
use crate::plugins::prelude::*;

/// Register default commands
pub fn register_commands() -> Vec<Box<dyn Command>> {
    vec![
        Box::new(Broadcast),
        Box::new(Disconnect),
        Box::new(Help),
        Box::new(Id),
    ]
}
