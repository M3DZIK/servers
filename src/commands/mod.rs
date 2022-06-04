mod help;

pub use help::*;

use crate::command_handler::Command;

pub fn register_commands() -> Vec<Box<dyn Command>> {
    let mut commands: Vec<Box<dyn Command>> = Vec::new();

    commands.push(Box::new(CommandHelp));

    commands
}
