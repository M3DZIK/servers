mod help;

pub use help::*;

use crate::plugins::Command;

pub fn register_commands() -> Vec<Box<dyn Command>> {
    vec![Box::new(CommandHelp)]
}
