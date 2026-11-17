# Async TCP Chat Engine

A Rust project focused on building an asynchronous TCP chat server using Tokio, with an emphasis on concurrent client handling, message broadcasting, graceful shutdown, and integration testing.

## Overview

Async TCP Chat Engine is a personal backend engineering project focused on asynchronous networking, concurrent connection handling, and low-level network communication in Rust.

## Technology Stack

- **Language:** Rust
- **Async Runtime:** Tokio
- **Network Protocol:** TCP

## Implemented Features
- Asynchronous TCP server.
- Concurrent handling of multiple client connections.
- Echoes receiving data back to connected clients.
- Track the number of active connections.
- Supports the 'connections_count' administrative command.

## Project Goals

- Explore asynchronous networking in Rust.
- Implement TCP communication using Tokio.
- Design a custom communication protocol.
- Build a maintainable and well-tested backend application.

## Project Status

Early development - core TCP server functionality implemented. More advanced features are planned.

## License

See the [LICENSE](LICENSE) file.