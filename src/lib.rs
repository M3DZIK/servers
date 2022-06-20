//! # Servers - Simple TCP server
//!
//! [image]: https://socialify.git.ci/MedzikUser/servers/image?description=1&font=KoHo&language=1&owner=1&pattern=Circuit%20Board&theme=Light
//!
//! [![image]](https://github.com/MedzikUser/servers)
//!
//! ## 👨‍💻 Building
//!
//! First clone the repository: `git clone https://github.com/MedzikUser/servers.git`
//!
//! ### Requirements
//! - Rust
//!
//! To build run the command: `cargo build --release`
//!
//! The compiled binary can be found in `./target/release/servers`
//!
//! ## Writing plugins
//!
//! Go to [plugins](plugins) module

#![doc(html_root_url = "https://servers.medzik.xyz")]
#![warn(missing_docs)]

pub mod commands;
pub mod plugins;
pub mod tcp;
