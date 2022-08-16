# Servers - Simple TCP and WebSocket server

[docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
[total-lines]: https://img.shields.io/tokei/lines/github/MedzikUser/servers?style=for-the-badge&logo=github&color=fede00
[code-size]: https://img.shields.io/github/languages/code-size/MedzikUser/servers?style=for-the-badge&color=c8df52&logo=github
[ci]: https://img.shields.io/github/workflow/status/MedzikUser/servers/Rust/main?style=for-the-badge
[image]:https://socialify.git.ci/MedzikUser/servers/image?description=1&font=KoHo&language=1&owner=1&pattern=Circuit%20Board&theme=Light

[![docs-rs]](https://servers.medzik.xyz)
[![total-lines]](https://github.com/MedzikUser/servers)
[![code-size]](https://github.com/MedzikUser/servers)
[![ci]](https://github.com/MedzikUser/servers/actions/workflows/build.yml)

[![image]](https://github.com/MedzikUser/servers)

A simple TCP server for clients and WebSocket server written in Rust 🦀 which can be extended with plugins.

## 👨‍💻 Building

First clone the repository: `git clone https://github.com/MedzikUser/servers.git`

### Requirements
- Rust

To build run the command: `cargo build --release`

The compiled binary can be found in `./target/release/servers`

## Writing plugins

Read the docs from [plugins](https://servers.medzik.xyz/servers/plugins) module.
