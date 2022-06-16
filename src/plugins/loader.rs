use std::{any::Any, fs, sync::Arc};

use async_trait::async_trait;
use libloading::{Library, Symbol};
use log::{debug, trace};

use crate::{commands, tcp::Client};

/// A plugin which allows you to add extra functionality.
#[async_trait]
pub trait Plugin: Any + Send + Sync {
    /// Get a name describing the `Plugin`.
    fn name(&self) -> &'static str;
    /// A function that runs immediately after plugin loading.
    /// Usually used for initialization.
    async fn on_plugin_load(&self);
    /// A function that runs immediately before the plugin is unloaded.
    /// Use this if you want to do any cleanup.
    async fn on_plugin_unload(&self);
}

pub trait PluginRegistrar {
    /// Function to register the plugin
    fn register(&mut self, plugin: Box<dyn Plugin>);
}

impl PluginRegistrar for PluginManager {
    fn register(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin)
    }
}

/// Plugin Manager
pub struct PluginManager {
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    /// Create empty `PluginManager`
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

pub type PluginManagerType = Arc<PluginManager>;

#[async_trait]
pub trait Command: Any + Send + Sync {
    /// Command name
    fn name(&self) -> &'static str;
    /// Help message of this command
    fn help(&self) -> &'static str;
    /// Command function
    async fn execute(
        &self,
        client: &mut Client,
        args: Vec<&str>,
        command_manager: &CommandManagerType,
    );
}

/// Command Manager
pub struct CommandManager {
    pub commands: Vec<Box<dyn Command>>,
}

impl CommandManager {
    /// Create empty `CommandManager`
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

pub type CommandManagerType = Arc<CommandManager>;

pub trait CommandRegistrar {
    /// Function to register the plugin and the commands in the plugin
    fn register(&mut self, command: Box<dyn Command>);
}

impl CommandRegistrar for CommandManager {
    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command)
    }
}

#[async_trait]
pub trait Event: Any + Send + Sync {
    /// Event name (onConnect, onSend)
    fn name(&self) -> &'static str;
    /// Event function
    async fn execute(&self, client: &mut Client);
}

/// Event Manager
pub struct EventManager {
    pub events: Vec<Box<dyn Event>>,
}

impl EventManager {
    /// Create empty `EventManager`
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self::new()
    }
}

pub type EventManagerType = Arc<EventManager>;

pub trait EventRegistrar {
    /// Function to register the plugin and the commands in the plugin
    fn register(&mut self, command: Box<dyn Event>);
}

impl EventRegistrar for EventManager {
    fn register(&mut self, command: Box<dyn Event>) {
        self.events.push(command)
    }
}

/// Plugins and Commands loader
pub fn loader() -> anyhow::Result<(CommandManagerType, PluginManagerType, EventManagerType)> {
    // get path to .so lib from command argument
    let config_dir = "./plugins";
    let paths = fs::read_dir(config_dir)?;

    // create a plugin manager where all loaded plugins will be located
    let mut plugin_manager = PluginManager::new();

    // create a command manager where located all commands
    let mut command_manager = CommandManager::new();

    // create a command manager where located all events from plugins
    let mut event_manager = EventManager::new();

    // register default commands
    for command in commands::register_commands() {
        command_manager.commands.push(command)
    }

    // for all plugin in directory
    for path in paths {
        // get library file path
        let path = path?.path();

        let plugin_path = path.to_str().unwrap_or("unknown");

        // log debug info
        debug!("Loading plugin `{}`", plugin_path);

        // loading library with .so is unsafe
        unsafe {
            // load library
            // Box::new and Box::leak must be there because if it isn't there it throws a segmentation fault
            let lib = Box::leak(Box::new(Library::new(&path)?));

            // get `plugin_entry` from library
            trace!("Finding symbol `plugin_entry` in `{}`", plugin_path);
            let func: Symbol<
                unsafe extern "C" fn(
                    &mut dyn PluginRegistrar,
                    &mut dyn CommandRegistrar,
                    &mut dyn EventRegistrar,
                ) -> (),
            > = lib.get(b"plugin_entry")?;

            // execute initial plugin function
            trace!("Running `plugin_entry(...)` in plugin `{}`", plugin_path);
            func(
                &mut plugin_manager,
                &mut command_manager,
                &mut event_manager,
            );
        }
    }

    // return CommandManager, PluginManager and EventManager
    Ok((
        Arc::new(command_manager),
        Arc::new(plugin_manager),
        Arc::new(event_manager),
    ))
}
