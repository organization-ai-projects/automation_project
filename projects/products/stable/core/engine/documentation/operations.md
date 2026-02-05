# Engine Operations

- [Back to Documentation Index](TOC.md)

## Purpose

Engine is the single hub for commands and events. It handles authentication, authorization, routing, and backend coordination.

## Key Endpoints

- `GET /health`: health check
- `POST /auth/login`: issue a session token (JWT)
- `GET /ws?token=<JWT>`: WebSocket command/event channel

## Startup

Engine is typically started by the Launcher or Watcher. For local development:

```bash
cargo run -p engine
```

## Registry

Engine reads product metadata and UI bundle paths from `.automation_project/registry.json`.
