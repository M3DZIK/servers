use std::{fs, path::Path, sync::Arc};

use libloading::{Library, Symbol};
use tracing::{info, trace};

use crate::{
    commands,
    plugins::{
        manager::{PluginsManager, PluginsManagerType},
        prelude::*,
    },
};

pub fn loader(plugins_dir: &str) -> anyhow::Result<PluginsManagerType> {
    // if plugins directory doesn't exists, create it
    if !Path::new(plugins_dir).exists() {
        fs::create_dir_all(plugins_dir)?;
    }

    // get all files from the plugins directory
    let plugins_files = fs::read_dir(plugins_dir)?;

    // init a plugins manager
    let mut plugins_manager = PluginsManager::new();

    // add default commands
    plugins_manager.commands = commands::register_commands();

    for plugin_path in plugins_files {
        let path = plugin_path?.path();
        let path_str = path.to_str().unwrap();

        info!("Loading plugin {}", path_str);

        // loading library from .so is unsafe
        unsafe {
            // Box::new and Box::leak must be there because
            // if it isn't there it throws an segmentation fault
            let lib = Box::leak(Box::new(Library::new(&path)?));

            trace!("Finding symbol `plugin_entry` in {}", path_str);
            let func: Symbol<unsafe extern "C" fn(&mut dyn Registrar) -> ()> =
                lib.get(b"plugin_entry")?;

            // execute the function `plugin_entry` to load the plugin (possible segmentation fault)
            trace!("Running function `plugin_entry` from plugin {}", path_str);
            func(&mut plugins_manager);
        }
    }

    Ok(Arc::new(plugins_manager))
}
