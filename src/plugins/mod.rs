//! Plugins loader
//!
//! # Writing a plugins
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
//! servers = { git = "https://github.com/MedzikUser/servers" }
//! ```
//!
//! In file `src/lib.rs`
//!
//! ```no_run
//! use servers::{plugins::{Plugin, Registrar}, tcp::Client, async_trait};
//!
//! struct PluginTest;
//!
//! /// Create a new plugin
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
//! /// Register plugin
//! #[no_mangle]
//! pub fn plugin_entry(registrar: &mut dyn Registrar) {
//!     registrar.register_plugin(Box::new(PluginTest));
//! }
//! ```
//!
//! ## Add command
//!
//! ```no_run
//! use servers::{
//!     plugins::{Command, PluginManagerType, Registrar, Result, async_trait},
//!     tcp::Client,
//! };
//! #
//! # struct PluginTest;
//! #
//! # #[async_trait]
//! # impl servers::plugins::Plugin for PluginTest {
//! #     /// Name of the plugin.
//! #     fn name(&self) -> &'static str {
//! #         "test"
//! #     }
//! #
//! #     /// A function will be executed when plugin loading.
//! #     /// Usally used for initialization.
//! #     async fn on_plugin_load(&self) {}
//! # }
//!
//! /// Create a new command
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
//!     async fn execute(&self, client: &mut Client, _args: Vec<&str>, _commands: &PluginManagerType) -> Result<()> {
//!         client.send("Command executed!").await?;
//!
//!         Ok(())
//!     }
//! }
//!
//! /// Register plugin
//! #[no_mangle]
//! pub fn plugin_entry(registrar: &mut dyn Registrar) {
//!     # registrar.register_plugin(Box::new(PluginTest));
//!     registrar.register_command(Box::new(PluginTest));
//! }
//! ```
//!
//! ## Add event
//!
//! In file `src/lib.rs`
//!
//! ```no_run
//! use servers::{
//!     plugins::{Event, Registrar, Result, async_trait},
//!     tcp::Client,
//! };
//! #
//! # struct PluginTest;
//! #
//! # #[async_trait]
//! # impl servers::plugins::Plugin for PluginTest {
//! #     /// Name of the plugin.
//! #     fn name(&self) -> &'static str {
//! #         "test"
//! #     }
//! #
//! #     /// A function will be executed when plugin loading.
//! #     /// Usally used for initialization.
//! #     async fn on_plugin_load(&self) {}
//! # }
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
//!     async fn execute(&self, client: &mut Client) -> Result<()> {
//!         client
//!             .send(format!("Welcome {}", client.peer_addr()?))
//!             .await?;
//!
//!         Ok(())
//!     }
//! }
//!
//! /// Register plugin
//! #[no_mangle]
//! pub fn plugin_entry(registrar: &mut dyn Registrar) {
//!     # registrar.register_plugin(Box::new(PluginTest));
//!     registrar.register_event(Box::new(PluginTest));
//! }
//! ```
//!
//! ## Build plugin
//!
//! To build plugin run command: `cargo build --release`
//!
//! The compiled plugin can be found in `target/release/libplugin.so`
//!
//! Move (or create a symlink) the built plugin to the `plugin/` directory in the server root directory.

mod loader;
mod types;

pub use loader::*;
pub use types::*;
