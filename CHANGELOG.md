# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

<!-- next-header -->

## [Unreleased]
### Features
- **plugins**: add `Result<()>` in `fn execute()` (Event and Command traits)
- **websocket**: WS Client <-> TCP Proxy (default port 9998) <-> TCP (default port 9999)

## [0.1.0] - 2022-06-17
### Default commands
- help

### Dynamic plugins loader
You can create custom commands and events (events executed if client connected or send message)

### Cli
You set custom host and port `./servers --host 0.0.0.0 --port 9999`

Show cli help `./servers --help`

<!-- next-url -->
[Unreleased]: https://github.com/MedzikUser/servers/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/MedzikUser/servers/commits/v0.1.0
