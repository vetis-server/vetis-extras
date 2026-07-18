---
layout: default
title: VeTiS Contrib - Very Tiny Server
nav_order: 1
description: "A blazingly fast, minimalist HTTP server built for modern Rust applications"
permalink: /
---

## Overview

Vetis Contrib extends the core Vetis server with modular functionality for different web application protocols and deployment scenarios. Each crate is designed to be optional and can be enabled based on your specific needs.

## Crates

### vetis-proxy

Very Tiny Server Reverse Proxy support

Provides reverse proxy functionality for the Vetis server, allowing it to forward requests to upstream servers.

- **Features**: HTTP/1.1, RustTLS support (via `runtime-tokio` feature)
- **Dependencies**: `deboa`, `deboa-tokio`, `once_cell`, `serde`, `vetis`, `vetis-tokio`
- **Status**: ✅ Active (enabled in workspace)

### vetis-static

Very Tiny Server Static Files support

Efficient static file serving with support for multiple async runtimes.

- **Features**:
  - MIME type detection
  - Configurable cache headers
  - Multiple runtime support: `runtime-tokio`, `runtime-smol`, `runtime-compio`
- **Dependencies**: `compio`, `duration-str`, `http`, `hyper-body-utils`, `log`, `minimime`, `regex`, `send_wrapper`, `serde`, `smol`, `time`, `tokio`, `typetag`, `vetis`
- **Status**: ✅ Active (enabled in workspace)

### vetis-asgi

Very Tiny Server ASGI support

Enables running Python ASGI (Asynchronous Server Gateway Interface) applications with Vetis.

- **Features**: Python async runtime integration via PyO3
- **Dependencies**: `crossfire`, `http`, `hyper-body-utils`, `log`, `pyo3`, `pyo3-async-runtimes`, `vetis`, `vetis-tokio`
- **Status**: 🚧 Development (commented out in workspace)

### vetis-fcgi

Very Tiny Server FCGI support

Provides FastCGI protocol support for interfacing with FastCGI applications.

- **Status**: 🚧 Development (commented out in workspace)

### vetis-rack

Very Tiny Server Rack support

Enables running Ruby Rack applications with Vetis.

- **Features**: Ruby integration via Magnus
- **Status**: 🚧 Development (commented out in workspace)

### vetis-rsgi

Very Tiny Server RSGI support

Provides support for the RSGI (Rack Server Gateway Interface) protocol.

- **Status**: 🚧 Development (commented out in workspace)

### vetis-sapi

Very Tiny Server SAPI support

Enables PHP integration via the Server API (SAPI) interface.

- **Status**: 🚧 Development (commented out in workspace)

### vetis-scgi

Very Tiny Server SCGI support

Provides SCGI (Simple Common Gateway Interface) protocol support.

- **Status**: 🚧 Development (commented out in workspace)

### vetis-wsgi

Very Tiny Server WSGI support

Enables running Python WSGI (Web Server Gateway Interface) applications with Vetis.

- **Features**: Python runtime integration via PyO3
- **Status**: 🚧 Development (commented out in workspace)

## Workspace Configuration

This is a Cargo workspace with the following configuration:

- **Rust Edition**: 2021 (most crates), 2024 (vetis-sapi)
- **Minimum Rust Version**: 1.75.0
- **License**: MIT OR Apache-2.0
- **Repository**: <https://github.com/vetis-server/vetis-extras>

## Building

To build the active crates (currently `vetis-proxy` and `vetis-static`):

```bash
cargo build --release
```

To build all crates including those in development, uncomment them in `Cargo.toml` first.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
