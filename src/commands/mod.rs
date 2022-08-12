mod disconnect;
mod help;

use self::{disconnect::Disconnect, help::Help};
use crate::plugins::prelude::*;

pub fn register_commands() -> Vec<Box<dyn Command>> {
    vec![Box::new(Help), Box::new(Disconnect)]
}
