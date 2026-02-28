# Engine Operations

- [Back to Documentation Index](TOC.md)

## Purpose

Engine is the single hub for commands and events. It handles authentication, authorization, routing, and backend coordination.

## Key Endpoints

- `GET /health`: health check
- `POST /auth/login`: issue a session token (JWT)
- `GET /ws?token=<JWT>`: WebSocket command/event channel
- `GET /projects`: list registered product projects (requires auth)
- `POST /projects/{project_id}/start`: start a product service (requires Admin role)
- `POST /projects/{project_id}/stop`: stop a product service (requires Admin role)

## Startup

Engine is typically started by the Launcher or Watcher. For local development:

```bash
cargo run -p engine
```

## Registry

Engine reads product metadata and UI bundle paths from `.automation_project/registry.json`.

## Product Orchestration

### WebSocket Commands

Admin users can trigger product start/stop via WebSocket commands:

```json
{
  "metadata": { "product_id": "<product_id_hex>" },
  "command_type": "StartJob",
  "action": "project.start"
}
```

```json
{
  "metadata": { "product_id": "<product_id_hex>" },
  "command_type": "StartJob",
  "action": "project.stop"
}
```

Both commands require `Admin` role. The engine forwards them to the registered product backend and returns the backend's acknowledgement.

### HTTP Endpoints

Alternatively, admin clients (such as `central_ui`) can use the HTTP endpoints:

```
POST /projects/{project_id}/start
Authorization: Bearer <admin_jwt>
```

```
POST /projects/{project_id}/stop
Authorization: Bearer <admin_jwt>
```

These endpoints require a valid admin token in the `Authorization` header. The engine forwards the action to the product backend and returns the result.

### Expected Flow

1. **Core boot**: Launcher starts `engine`, `watcher`, and `central_ui`.
2. **Backend registration**: Product backends connect to the engine WebSocket and register via `backend.hello`.
3. **Product activation**: An admin user triggers start/stop via `central_ui` admin buttons, which POST to `central_ui`'s proxy routes.
4. **Orchestration**: `central_ui` forwards the request to `engine`, which routes it to the product backend. The backend performs the action and responds with an acknowledgement event.
