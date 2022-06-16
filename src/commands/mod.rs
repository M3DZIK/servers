mod help;

use crate::plugins::Command;

/// Register default commands
pub fn register_commands() -> Vec<Box<dyn Command>> {
    // create Vector with Commands
    vec![Box::new(help::CommandHelp)]
}
