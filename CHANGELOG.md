# Changelog

<!-- next-header -->

## [Unreleased]

## [0.3.0] - 2022-08-04
## **Breaking**
- **tcp**: use tcp from tokio instead of std

## Features
- **client**: added `peer_addr` function
- **server**: added `/help` command
- **api**: re-export `async_trait` so that it doesn't have to be added to dependencies in plugins

## Changed
- **server**: the `/help` command has been accelerated
- **cli**: moved to the `cli.rs` file
- **logger**: changed `log` to `tracing`
- **dependencies**: updated
- **cli**: deleted option `--disable-websocket` and added `--enable-websocket`

## [0.2.0] - 2022-06-26
### Features
- **plugins**: add `Result<()>` in `fn execute()` (Event and Command traits)
- **websocket**: WS Client <-> TCP Proxy (default port 9998) <-> TCP (default port 9999)

### Chore
- **deps**: upgrade

## [0.1.0] - 2022-06-17
### Default commands
- help

### Dynamic plugins loader
You can create custom commands and events (events executed if client connected or send message)

### Cli
You set custom host and port `./servers --host 0.0.0.0 --port 9999`

Show cli help `./servers --help`

<!-- next-url -->
[Unreleased]: https://github.com/MedzikUser/servers/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/MedzikUser/servers/commits/v0.3.0
[0.2.0]: https://github.com/MedzikUser/servers/commits/v0.2.0
[0.1.0]: https://github.com/MedzikUser/servers/commits/v0.1.0
