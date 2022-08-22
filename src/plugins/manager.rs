use core::fmt;
use std::sync::Arc;

use crate::plugins::prelude::*;

/// Plugins manager struct with Clone derive added by Arc.
pub type PluginsManagerType = Arc<PluginsManager>;

/// A plugins manager that stores all plugins, commands and events.
#[derive(Default)]
pub struct PluginsManager {
    /// Vector with all loaded plugins.
    pub plugins: Vec<Box<dyn Plugin>>,
    /// Vector with all loaded commands.
    pub commands: Vec<Box<dyn Command>>,
    /// Vector with all loaded events.
    pub events: Vec<Box<dyn Event>>,
}

impl PluginsManager {
    /// Returns an empty instance of [PluginsManager]
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            commands: Vec::new(),
            events: Vec::new(),
        }
    }

    /// Returns the instance in [PluginsManagerType].
    pub fn into(self) -> PluginsManagerType {
        Arc::new(self)
    }
}

impl fmt::Debug for PluginsManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PluginsManager")
            .field("plugins", &self.plugins.len())
            .field("commands", &self.commands.len())
            .field("events", &self.events.len())
            .finish()
    }
}
