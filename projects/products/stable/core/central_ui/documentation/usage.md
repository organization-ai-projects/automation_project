# Central UI Usage

- [Back to Documentation Index](TOC.md)

## Purpose

Central UI is the main desktop interface. It loads product UI bundles at runtime and connects to the Engine over WebSocket.

## Key Behaviors

- Displays product list and system status
- Routes user actions to the Engine
- Shows real-time events and logs
- Provides admin buttons to start and stop product services

## Startup

Central UI is usually started by the Launcher. For local development:

```bash
cargo run -p central_ui
```

To point at a custom Engine address:

```bash
cargo run -p central_ui -- --engine ws://localhost:9000
```

## Product Activation (Admin)

Administrators can start or stop product services via the central_ui admin interface. The flow is:

1. The admin is authenticated and holds a valid `Admin` JWT.
2. The admin triggers a start or stop action on a product.
3. `central_ui` POSTs to its own proxy endpoints:
   - `POST /api/projects/{project_id}/start` — start the product service
   - `POST /api/projects/{project_id}/stop` — stop the product service
4. `central_ui` forwards the request to the engine (`POST /projects/{project_id}/start` or `/stop`).
5. The engine validates the admin token, locates the registered product backend, and forwards the command.
6. The product backend performs the action and the engine returns the result.

### Expected Lifecycle

| Step | Component | Action |
|------|-----------|--------|
| 1 | Launcher | Starts `engine`, `watcher`, `central_ui` |
| 2 | Product backend | Connects to engine WebSocket, sends `backend.hello` |
| 3 | Admin via central_ui | Clicks Start/Stop button |
| 4 | central_ui | POSTs to `/api/projects/{id}/start` or `/stop` |
| 5 | Engine | Validates admin token, forwards `project.start`/`project.stop` to backend |
| 6 | Product backend | Executes action, responds with acknowledgement |
