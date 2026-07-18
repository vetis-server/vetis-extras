# front

A very tiny frontend server built with Rust and the vetis framework.

## Description

front is a lightweight static file server that serves files from a specified root directory. It provides a simple HTTP/1 server for development and testing purposes.

## Features

- Serve static files from a configurable root directory
- Configurable interface and port binding
- Support for common web file extensions (html, js, css, svg, png, ico, jsx, ts, tsx, json, mjs)
- Automatic index file serving (index.html)
- HTTP/1 protocol support
- Built with vetis framework for efficient HTTP handling

## Installation

Install front:

```bash
cargo install front
```

## Usage

Run the server with default settings:

```bash
front
```

This will serve the current directory on port 4444.

### Command Line Options

- `-i, --interface <INTERFACE>` - Interface to bind to (default: 0.0.0.0)
- `-p, --port <PORT>` - Port to bind to (default: 4444)
- `-r, --root <ROOT>` - Root directory to serve (default: .)
- `-h, --help` - Print help information
- `-V, --version` - Print version information

### Examples

Serve a specific directory:

```bash
front -r /path/to/your/files
```

Bind to a specific interface and port:

```bash
front -i 127.0.0.1 -p 8080
```

Combine options:

```bash
front -r ./public -i 0.0.0.0 -p 3000
```

## License

This project is licensed under the same license as the vetis workspace.

## Contributing

Contributions are welcome. Please feel free to submit a Pull Request.
