use std::{fs, sync::Arc};

use libloading::{Library, Symbol};
use log::{debug, trace};

use crate::{commands, plugins::Registrar};

use super::{PluginManager, PluginManagerType};

/// Plugins and Commands loader
pub fn loader() -> anyhow::Result<PluginManagerType> {
    // get path to .so lib from command argument
    let config_dir = "./plugins";
    let paths = fs::read_dir(config_dir)?;

    // create a plugin manager
    let mut plugin_manager = PluginManager::new();

    // register default commands
    for command in commands::register_commands() {
        plugin_manager.commands.push(command)
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
            let func: Symbol<unsafe extern "C" fn(&mut dyn Registrar) -> ()> =
                lib.get(b"plugin_entry")?;

            // execute initial plugin function
            trace!("Running `plugin_entry(...)` in plugin `{}`", plugin_path);
            func(&mut plugin_manager);
        }
    }

    // return a `PluginManager`
    Ok(Arc::new(plugin_manager))
}
