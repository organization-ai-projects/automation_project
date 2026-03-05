# Colony Manager

Deterministic colony simulation with strict backend/UI split.

## Crates

- `colony_manager_backend` (`backend/`): deterministic simulation engine and protocol server
- `colony_manager_ui` (`ui/`): UI/orchestrator layer (native CLI bridge + wasm app entrypoint)

## Build

```sh
cargo build -p colony_manager_backend
cargo build -p colony_manager_ui
```

## Run

```sh
# Run simulation
colony_manager run --ticks N --seed S [--scenario <path>] --out <path> [--replay-out <path>]

# Replay
colony_manager replay --replay <path> --out <path>
```

## UI Orchestration

Native `colony_manager_ui` forwards `run|replay` commands to `colony_manager_backend` and keeps UI app state transitions deterministic.
