# Colony Manager

Deterministic colony simulation with strict backend/UI split.

## Structure

- `metadata.ron`: product metadata (stable-style product descriptor)
- `backend/`: `colony_manager_backend` deterministic simulation + replay binary
- `ui/`: `colony_manager_ui` UI/orchestrator crate + `ui_manifest.ron`

## Crates

- `colony_manager_backend`: deterministic simulation engine and protocol server
- `colony_manager_ui`: UI/orchestrator layer (native bridge + wasm app entrypoint)

## Build

```sh
cargo build -p colony_manager_backend
cargo build -p colony_manager_ui
```

## Run

```sh
# Run simulation
colony_manager_backend run --ticks N --seed S [--scenario <path>] --out <path> [--replay-out <path>]

# Replay
colony_manager_backend replay --replay <path> --out <path>
```

## UI Orchestration

Native `colony_manager_ui` forwards `run|replay` commands to `colony_manager_backend` and keeps UI app state transitions deterministic.
