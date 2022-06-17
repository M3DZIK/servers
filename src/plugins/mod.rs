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
//! ```
//! use async_trait::async_trait;
//! use servers::{
//!     plugins::{
//!         Command, CommandManagerType, CommandRegistrar, EventRegistrar, Plugin,
//!         PluginRegistrar,
//!     },
//!     tcp::Client,
//! };
//!
//! struct PluginTest;
//!
//! #[async_trait]
//! impl Plugin for PluginTest {
//!     /// Name of the plugin.
//!     fn name(&self) -> &'static str {
//!         "test"
//!     }
//!
//!     /// Function will be executed when plugin loading.
//!     async fn on_plugin_load(&self) {
//!         println!("Loading plugin `test`...")
//!     }
//!
//!     /// Function will be executed when plugin unloading.
//!     async fn on_plugin_unload(&self) {
//!         println!("Unloading plugin `test`...")
//!     }
//! }
//!
//! #[async_trait]
//! impl Command for PluginTest {
//!     /// Command name
//!     fn name(&self) -> &'static str {
//!         "/test"
//!     }
//!
//!     /// Command help message
//!     fn help(&self) -> &'static str {
//!         "test command"
//!     }
//!
//!     /// Function will be executed when client send command `/test`
//!     async fn execute(&self, client: &mut Client, _args: Vec<&str>, _commands: &CommandManagerType) {
//!         client.send("Message sended by `test` plugin").expect("send message")
//!     }
//! }
//!
//! /// Register plugin and event
//! #[no_mangle]
//! pub fn plugin_entry(
//!     plugin: &mut dyn PluginRegistrar,
//!     command: &mut dyn CommandRegistrar,
//!     _event: &mut dyn EventRegistrar,
//! ) {
//!     // register plugin
//!     plugin.register(Box::new(PluginTest));
//!     // register command
//!     command.register(Box::new(PluginTest));
//! }
//! ```
//!
//! ### Event plugin
//!
//! In file `src/lib.rs`
//!
//! ```
//! use async_trait::async_trait;
//! use servers::{
//!     plugins::{
//!         CommandManagerType, CommandRegistrar, Event, EventRegistrar, Plugin,
//!         PluginRegistrar,
//!     },
//!     tcp::Client,
//! };
//!
//! struct PluginTest;
//!
//! #[async_trait]
//! impl Plugin for PluginTest {
//!     /// Name of the plugin.
//!     fn name(&self) -> &'static str {
//!         "test"
//!     }
//!
//!     /// Function will be executed when plugin loading.
//!     async fn on_plugin_load(&self) {
//!         println!("Loading plugin `test`...")
//!     }
//!
//!     /// Function will be executed when plugin unloading.
//!     async fn on_plugin_unload(&self) {
//!         println!("Unloading plugin `test`...")
//!     }
//! }
//!
//! #[async_trait]
//! impl Event for PluginTest {
//!     /// Event name (onConnect, onSend)
//!     fn name(&self) -> &'static str {
//!         "onConnect"
//!     }
//!
//!     /// Function will be executed when client connected
//!    async fn execute(&self, client: &mut Client) {
//!        client
//!            .send(&format!("Welcome {}", client.stream.peer_addr().unwrap()))
//!            .expect("send message")
//!     }
//! }
//!
//! /// Register plugin and command
//! #[no_mangle]
//! pub fn plugin_entry(
//!     plugin: &mut dyn PluginRegistrar,
//!     _command: &mut dyn CommandRegistrar,
//!     event: &mut dyn EventRegistrar,
//! ) {
//!     // register plugin
//!     plugin.register(Box::new(PluginTest));
//!     // register event
//!     event.register(Box::new(PluginTest));
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

pub use loader::*;
