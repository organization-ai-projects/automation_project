# Patchsmith — WASM Console UI

A WASM UI shell with deterministic state model and plugin panels.

## Binaries

- `wasm_console_ui_backend`: domain logic, state orchestration, reducer, plugin registry, snapshot codec, IPC server.
- `wasm_console_ui_ui`: Dioxus WASM frontend, display-only screens, typed IPC client.

## Architecture

Communication between UI and backend is strictly via local IPC (stdin/stdout JSON lines).

The backend is the single source of truth for:

- application state model
- action model and reducer execution
- plugin registration and builtin plugin definitions
- snapshot serialization/deserialization and canonical ordering
- diagnostics and stable result mapping

The UI only:

- collects user inputs
- sends typed requests to backend
- displays logs, reports, graphs, and snapshot state
- renders backend-derived status and diagnostics

## Determinism Contract

- State transitions are pure and deterministic (reducer-only mutation).
- UI snapshot is canonical (stable serialization with SHA-256 checksum).
- Plugin registry order is deterministic (sorted by plugin ID).
- Re-importing the same snapshot reconstructs the same state.

## Builtin Plugins

- **Log Viewer** — loads and displays JSON log files
- **Report Viewer** — loads and displays JSON report files
- **Graph Viewer** — minimal node/edge display from JSON

## Tests

```bash
cargo test -p wasm_console_ui_backend
cargo test -p wasm_console_ui_ui
```
