# Backlog App

Cross-platform desktop application for [Personal Backlog](https://github.com/your/backlog) — a minimalist task manager where your tasks live in a single Markdown file.

## Features

- **Native desktop app** for Windows, macOS, and Linux
- **Embedded HTTP server** — no external dependencies
- **File-based persistence** — data stored as `backlog.md` in your chosen folder
- **Offline-first** — works without internet connection
- **File watching** — automatically detects external edits

## Quick Start

### Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run dev
```

### Build

```bash
# Build for current platform
npm run build
```

The built application will be in `src-tauri/target/release/bundle/`.

## Architecture

- **Frontend**: React-based UI (from Personal Backlog)
- **Backend**: Rust with Tauri 2.x
- **HTTP Server**: actix-web embedded server
- **File Operations**: Native Rust filesystem APIs

See [doc/TDD.md](doc/TDD.md) for detailed technical design.

## Project Structure

```
backlog-app/
├── doc/               # Documentation
├── src/               # Frontend (React)
├── src-tauri/         # Rust backend
│   ├── src/
│   │   ├── server/    # HTTP API handlers
│   │   ├── file_manager/
│   │   └── watcher/
│   └── Cargo.toml
└── package.json
```

## License

MIT
