use std::{any::Any, sync::Arc};

use async_trait::async_trait;

use crate::{Client, commands};

#[async_trait]
pub trait Command: Any + Send + Sync {
    /// Command name
    fn name(&self) -> &'static str;
    /// Command help message
    fn help(&self) -> &'static str;
    /// Command function
    async fn execute(
        &self,
        client: &mut Client,
        args: Vec<&str>,
        plugins: &CommandManagerType,
    );
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

pub type CommandManagerType = Vec<Arc<Box<dyn Command>>>;

pub fn register_commands() -> CommandManagerType {
    let mut command_manager = CommandManager::new();

    for command in commands::register_commands() {
        command_manager.commands.push(command)
    }

    // create Arc in Vector
    let mut commands: CommandManagerType = Vec::new();
    for command in command_manager.commands {
        commands.push(Arc::new(command))
    }

    commands
}
