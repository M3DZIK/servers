use std::{any::Any, sync::Arc};

use async_trait::async_trait;

use crate::Client;

#[async_trait]
pub trait Command: Any + Send + Sync {
    /// Command name
    fn name(&self) -> &'static str;
    /// Help message of this command
    fn help(&self) -> &'static str;
    /// Command function
    async fn execute(&self, client: &mut Client, args: Vec<&str>, commands: &CommandManagerType);
}

pub struct CommandManager {
    /// Vector with plugins
    pub commands: Vec<Box<dyn Command>>,
}

impl CommandManager {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
}

impl Default for CommandManager {
    fn default() -> Self {
        Self::new()
    }
}

pub type CommandManagerType = Arc<Vec<Box<dyn Command>>>;
