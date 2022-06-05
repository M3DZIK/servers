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

pub trait PluginRegistrar {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>);
}

impl PluginRegistrar for PluginManager {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin)
    }
}

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

pub type CommandManagerType = Arc<CommandManager>;

impl Default for CommandManager {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CommandRegistrar {
    fn register_plugin(&mut self, command: Box<dyn Command>);
}

impl CommandRegistrar for CommandManager {
    fn register_plugin(&mut self, command: Box<dyn Command>) {
        self.commands.push(command)
    }
}

pub fn loader() -> anyhow::Result<(Arc<CommandManager>, Arc<PluginManager>)> {
    // get path to .so lib from command argument
    let config_dir = "./plugins";
    let paths = fs::read_dir(config_dir)?;

    // create a plugin manager where all loaded plugins will be located
    let mut plugin_manager = PluginManager::new();

    // create a command manager where located all commands
    let mut command_manager = CommandManager::new();

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
                unsafe extern "C" fn(&mut dyn PluginRegistrar, &mut dyn CommandRegistrar) -> (),
            > = lib.get(b"plugin_entry")?;

            // execute initial plugin function
            trace!("Running `plugin_entry(...)` in plugin `{}`", plugin_path);
            func(&mut plugin_manager, &mut command_manager);
        }
    }

    Ok((Arc::new(command_manager), Arc::new(plugin_manager)))
}
