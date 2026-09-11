# Backlog App

A native desktop wrapper for [Personal Backlog](https://github.com/anthropics/backlog) — bringing your task management fully offline.

## Why?

Personal Backlog is a minimalist web-based task manager that stores everything in a single Markdown file. The web version requires a server or hosting setup. **Backlog App eliminates that requirement entirely.**

- **Zero setup** — download, run, start managing tasks
- **Fully offline** — no internet, no server, no account, no cloud sync
- **Your data, your files** — tasks live in a plain `backlog.md` file on your computer
- **Edit anywhere** — open `backlog.md` in any text editor, changes sync automatically
- **Portable** — copy your backlog file between devices, back it up however you want

## How It Works

Backlog App bundles the Personal Backlog web interface with an embedded HTTP server, all inside a native desktop application. The server runs locally on your machine — nothing leaves your computer.

```
┌─────────────────────────────────────┐
│         Backlog App (Tauri)         │
│  ┌─────────────┐  ┌──────────────┐  │
│  │  Web UI     │◄─│ HTTP Server  │  │
│  │  (React)    │  │ (tiny_http)  │  │
│  └─────────────┘  └──────┬───────┘  │
└──────────────────────────┼──────────┘
                           │
                    ┌──────▼──────┐
                    │ backlog.md  │
                    └─────────────┘
```

## Platforms

| Platform | Architecture | Format |
|----------|--------------|--------|
| macOS | Apple Silicon (M1/M2/M3) | `.dmg` |
| macOS | Intel x64 | `.dmg` |
| Windows | x64 | `.exe`, `.msi` |
| Windows | ARM64 | `.exe` |
| Linux | x64 | `.deb`, `.AppImage`, `.rpm` |
| Linux | ARM64 | `.deb`, `.AppImage` |
| NixOS | x64/ARM64 | Flake |

## Download

See [Releases](../../releases) for the latest builds.

## Usage

1. **Launch** — open Backlog App
2. **Choose folder** — select where to store your `backlog.md` (or use the default)
3. **Add tasks** — use the UI or edit the Markdown file directly

Your backlog file is plain Markdown. Sync it with Git, Dropbox, or any tool you already use.

## Building from Source

**Prerequisites:** Node.js 20+, Rust 1.70+, [Tauri prerequisites](https://tauri.app/start/prerequisites/)

```bash
npm install
npm run build      # Release build
npm run dev        # Development mode
```

**NixOS:**
```bash
nix build          # Build package
nix develop        # Enter dev shell
```

## Technical Details

| Component | Technology |
|-----------|------------|
| Shell | Tauri 2.x |
| Server | tiny_http (embedded, single-threaded) |
| Frontend | React (pre-bundled, no CDN dependencies) |
| Storage | Markdown file with automatic backups |

See [doc/TDD.md](doc/TDD.md) for architecture documentation.

## License

GPL-3.0 — see [LICENSE](LICENSE)
