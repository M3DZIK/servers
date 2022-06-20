use std::{any::Any, sync::Arc};

use async_trait::async_trait;

use crate::tcp::Client;

/// A plugin wich allows you to add extra functionality.
#[async_trait]
pub trait Plugin: Any + Send + Sync {
    /// Name of the plugin.
    fn name(&self) -> &'static str;
    /// A function will be executed when plugin loading.
    /// Usally used for initialization.
    async fn on_plugin_load(&self);
}

/// Add a new command
#[async_trait]
pub trait Command: Any + Send + Sync {
    /// Name of the command.
    fn name(&self) -> &'static str;
    /// Help message of the command.
    fn help(&self) -> &'static str;
    /// Command function
    async fn execute(
        &self,
        client: &mut Client,
        args: Vec<&str>,
        plugin_manager: &PluginManagerType,
    );
}

/// Add a new function that will be executed when the event occurs
#[async_trait]
pub trait Event: Any + Send + Sync {
    /// Event name (onConnect or onSend)
    fn name(&self) -> &'static str;
    /// Event function
    async fn execute(&self, client: &mut Client);
}

/// Plugin Manager
pub struct PluginManager {
    /// Vector with loaded plugins.
    pub plugins: Vec<Box<dyn Plugin>>,
    /// Vector with all commands.
    pub commands: Vec<Box<dyn Command>>,
    /// Vector with all events.
    pub events: Vec<Box<dyn Event>>,
}

impl PluginManager {
    /// Create an empty [PluginManager]
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            commands: Vec::new(),
            events: Vec::new(),
        }
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Type of the [PluginManager]
pub type PluginManagerType = Arc<PluginManager>;

/// Plugin Registrar
pub trait Registrar {
    /// Function to register the plugin
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>);
    /// Function to register the command
    fn register_command(&mut self, command: Box<dyn Command>);
    /// Function to register the event
    fn register_event(&mut self, event: Box<dyn Event>);
}

impl Registrar for PluginManager {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin)
    }

    fn register_command(&mut self, command: Box<dyn Command>) {
        self.commands.push(command)
    }

    fn register_event(&mut self, event: Box<dyn Event>) {
        self.events.push(event)
    }
}
