use std::{any::Any, fs, sync::Arc};

use libloading::{Library, Symbol};
use log::{debug, trace};

use crate::Client;

/// A plugin which allows you to add extra functionality.
pub trait Plugin: Any + Send + Sync {
    /// Get a name describing the `Plugin`.
    fn name(&self) -> &'static str;
    /// A function that runs immediately after plugin loading.
    /// Usually used for initialization.
    fn on_plugin_load(&self);
    /// A function that runs immediately before the plugin is unloaded.
    /// Use this if you want to do any cleanup.
    fn on_plugin_unload(&self);
    /// Plugin command.
    /// For example: `/command`
    fn command(&self) -> &'static str;
    /// Help message of this command.
    fn help(&self) -> &'static str;
    /// The function will be executed, when sending plugin command.
    fn execute(&self, client: &mut Client, args: Vec<&str>);
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
    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for plugin in self.plugins.drain(..) {
            trace!("Firing on_plugin_unload for {:?}", plugin.name());
            plugin.on_plugin_unload();
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

pub type PluginManagerType = Vec<Arc<Box<dyn Plugin>>>;

pub fn loader() -> anyhow::Result<PluginManagerType> {
    // get path to .so lib from command argument
    let config_dir = "./plugins";
    let paths = fs::read_dir(config_dir)?;

    // create a plugin manager where all loaded plugins will be located
    let mut plugin_manager = PluginManager::new();

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
            let func: Symbol<unsafe extern "C" fn(&mut dyn PluginRegistrar) -> ()> =
                lib.get(b"plugin_entry")?;

            // execute initial function
            func(&mut plugin_manager);
        }
    }

    // create Arc in Vector
    let mut plugins: PluginManagerType = Vec::new();
    for plugin in plugin_manager.plugins {
        plugins.push(Arc::new(plugin))
    }

    Ok(plugins)
}
