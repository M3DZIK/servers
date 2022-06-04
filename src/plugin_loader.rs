use std::{any::Any, fs, sync::Arc};

use async_trait::async_trait;
use libloading::{Library, Symbol};
use log::{debug, trace};

use crate::{commands, Command, CommandManager, CommandManagerType};

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
    /// Vector with loaded plugins
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    /// Create empty `PluginManager`
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Unload all plugins and loaded plugin libraries, making sure to fire
    /// their `on_plugin_unload()` methods so they can do any necessary cleanup.
    pub async fn unload(&mut self) {
        debug!("Unloading plugins");

        for plugin in self.plugins.drain(..) {
            trace!("Firing on_plugin_unload for {:?}", plugin.name());
            plugin.on_plugin_unload().await;
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

pub trait CommandRegistrar {
    fn register_plugin(&mut self, command: Box<dyn Command>);
}

impl CommandRegistrar for CommandManager {
    fn register_plugin(&mut self, command: Box<dyn Command>) {
        self.commands.push(command)
    }
}

pub type PluginManagerType = Arc<Vec<Box<dyn Plugin>>>;

pub fn loader() -> anyhow::Result<(PluginManagerType, CommandManagerType)> {
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

        // loading library with .so is unsafe
        unsafe {
            // load library
            // Box::new and Box::leak must be there because if it isn't there it throws a segmentation fault
            let lib = Box::leak(Box::new(Library::new(path)?));

            // get `plugin_entry` from library
            let func: Symbol<
                unsafe extern "C" fn(&mut dyn PluginRegistrar, &mut dyn CommandRegistrar) -> (),
            > = lib.get(b"plugin_entry")?;

            // execute initial function
            func(&mut plugin_manager, &mut command_manager);
        }
    }

    // create Arc in Vector
    let mut commands = Vec::new();
    for command in command_manager.commands {
        commands.push(command)
    }

    // create Arc in Vector
    let mut plugins = Vec::new();
    for plugin in plugin_manager.plugins {
        plugins.push(plugin)
    }

    Ok((Arc::new(plugins), Arc::new(commands)))
}
