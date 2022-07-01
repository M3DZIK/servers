# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->

## [Unreleased]
## Chore
- **tcp**: use tcp from tokio instead of std

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
[Unreleased]: https://github.com/MedzikUser/servers/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/MedzikUser/servers/commits/v0.2.0
[0.1.0]: https://github.com/MedzikUser/servers/commits/v0.1.0
