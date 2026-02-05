# Watcher Supervision

- [Back to Documentation Index](TOC.md)

## Purpose

Watcher supervises core services and restarts them if they crash. It performs basic health checks and applies backoff to avoid restart loops.

## Startup

Watcher is usually started by the Launcher. For local development:

```bash
cargo run -p watcher
```

## What It Watches

- Engine
- Central UI
- Product backends (when configured)
