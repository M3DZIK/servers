use core::fmt;
use std::sync::Arc;

use crate::plugins::prelude::*;

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
    /// Returns an empty PluginsManager
    pub fn new() -> PluginsManager {
        Self {
            plugins: Vec::new(),
            commands: Vec::new(),
            events: Vec::new(),
        }
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

pub type PluginsManagerType = Arc<PluginsManager>;
