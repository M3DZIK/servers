use std::any::Any;

use async_trait::async_trait;

use crate::{plugins::manager::PluginsManager, server::Client};

// A main plugin trait.
#[async_trait]
pub trait Plugin: Any + Send + Sync {
    /// Name of the plugin.
    fn name(&self) -> &'static str;
    /// A function that will be executed when the plugin is loaded.
    async fn on_load(&self);
}

/// Add a command to the plugin.
#[async_trait]
pub trait Command: Any + Send + Sync {
    /// Name of the command.
    fn name(&self) -> &'static str;
    /// Aliases for the command.
    fn aliases(&self) -> Vec<&'static str>;
    /// Help message of the command.
    fn help(&self) -> &'static str;
    /// Usage message of the command.
    fn usage(&self) -> &'static str;
    /// Command function.
    async fn execute(&self, client: &Client, args: Vec<&str>) -> anyhow::Result<()>;
}

/// All possible to run events.
#[derive(Debug, PartialEq, Eq)]
pub enum EventType {
    /// On client connected.
    OnConnect,
    /// On client sent message.
    OnSend,
}

/// Add a event to the plugin.
#[async_trait]
pub trait Event: Any + Send + Sync {
    /// Type of the event.
    fn event(&self) -> EventType;
    /// Event function.
    async fn execute(&self, client: &Client) -> anyhow::Result<()>;
}

pub trait Registrar {
    /// Function to register plugins.
    fn register_plugins(&mut self, plugin: Box<dyn Plugin>);
    /// Function to register commands.
    fn register_commands(&mut self, command: Box<dyn Command>);
    /// Function to register events.
    fn register_events(&mut self, event: Box<dyn Event>);
}

impl Registrar for PluginsManager {
    fn register_plugins(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin)
    }

    fn register_commands(&mut self, command: Box<dyn Command>) {
        self.commands.push(command)
    }

    fn register_events(&mut self, event: Box<dyn Event>) {
        self.events.push(event)
    }
}
