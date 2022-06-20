//! Plugins loader
//!
//! ## Writing plugins
//!
//! Create a new project `cargo new --lib plugin`
//!
//! Set a `crate-type` in Cargo.toml (to build a `.so` plugin)
//!
//! ```toml
//! [lib]
//! crate-type = ["dylib"]
//! ```
//!
//! Add a `servers` and `async-trait` dependencies to Cargo.toml
//!
//! ```toml
//! [dependencies]
//! async-trait = "0.1.56"
//! servers = "0.1.0"
//! ```
//!
//! ### Command plugin
//!
//! In file `src/lib.rs`
//!
//! ```no_run
//! use async_trait::async_trait;
//! use servers::{
//!     plugins::{Command, Plugin, PluginManagerType, Registrar},
//!     tcp::Client,
//! };
//!
//! struct PluginTest;
//!
//! /// Create a new plugin.
//! #[async_trait]
//! impl Plugin for PluginTest {
//!     /// Name of the plugin.
//!     fn name(&self) -> &'static str {
//!         "test"
//!     }
//!
//!     /// A function will be executed when plugin loading.
//!     /// Usally used for initialization.
//!     async fn on_plugin_load(&self) {}
//! }
//!
//! /// Create a new command.
//! #[async_trait]
//! impl Command for PluginTest {
//!     /// Command name
//!     fn name(&self) -> &'static str {
//!         "/test"
//!     }
//!
//!     /// Help message of the command
//!     fn help(&self) -> &'static str {
//!         "test command"
//!     }
//!
//!     /// Command function
//!     async fn execute(&self, client: &mut Client, _args: Vec<&str>, _commands: &PluginManagerType) {
//!         client.send("content").expect("send message")
//!     }
//! }
//!
//! /// Regsiter plugin
//! #[no_mangle]
//! pub fn plugin_entry(registrar: &mut dyn Registrar) {
//!     registrar.register_plugin(Box::new(PluginTest));
//!     registrar.register_command(Box::new(PluginTest));
//! }
//! ```
//!
//! ### Event plugin
//!
//! In file `src/lib.rs`
//!
//! ```no_run
//! use async_trait::async_trait;
//! use servers::{
//!     plugins::{Event, Plugin, PluginManagerType, Registrar},
//!     tcp::Client,
//! };
//!
//! struct PluginTest;
//!
//! /// Create a new plugin.
//! #[async_trait]
//! impl Plugin for PluginTest {
//!     /// Name of the plugin.
//!     fn name(&self) -> &'static str {
//!         "test"
//!     }
//!
//!     /// A function will be executed when plugin loading.
//!     /// Usally used for initialization.
//!     async fn on_plugin_load(&self) {}
//! }
//!
//! /// Create a new event
//! #[async_trait]
//! impl Event for PluginTest {
//!     /// Event name (onConnect or onSend)
//!     fn name(&self) -> &'static str {
//!         "onConnect"
//!     }
//!
//!     /// Event function
//!     async fn execute(&self, client: &mut Client) {
//!         client
//!             .send(&format!("Welcome {}", client.stream.peer_addr().unwrap()))
//!             .expect("send message")
//!     }
//! }
//!
//! /// Regsiter plugin
//! #[no_mangle]
//! pub fn plugin_entry(registrar: &mut dyn Registrar) {
//!     registrar.register_plugin(Box::new(PluginTest));
//!     registrar.register_event(Box::new(PluginTest));
//! }
//! ```
//!
//! ## Build plugin
//!
//! To build plugin run command: `cargo build --release`
//!
//! The compiled plugin can be found in `./target/release/libplugin.so`
//!
//! Move compiled plugin to the `plugin` directory where servers is located

mod loader;
mod types;

pub use loader::*;
pub use types::*;
