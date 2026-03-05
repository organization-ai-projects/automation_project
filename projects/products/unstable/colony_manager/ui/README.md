# Colony Manager UI

UI/orchestrator crate for Colony Manager.

## Role

This crate owns UI-facing flow and delegates simulation commands to the backend binary.

## Binary

Package: `colony_manager_ui`

Native mode:

- Accepts `run` / `replay` commands
- Forwards to backend executable
- Backend executable default: `colony_manager_backend`
- Override with env var: `COLONY_MANAGER_BACKEND_BIN`

WASM mode:

- Entrypoint uses Dioxus (`dioxus::launch(app::app)`)
