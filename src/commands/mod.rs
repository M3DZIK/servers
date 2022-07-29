//! Build-in commands

mod disconnect;
mod help;

use crate::plugins::Command;

/// Register build-in commands
pub fn register_commands() -> Vec<Box<dyn Command>> {
    // create array with build-in commands
    vec![
        Box::new(help::CommandHelp),
        Box::new(disconnect::CommandDisconnect),
    ]
}
