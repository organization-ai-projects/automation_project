# Tactics Grid

Deterministic grid tactics engine (XCOM/FFT-like) with strict backend/UI split.

## Structure

- `metadata.ron`: product metadata (stable-style product descriptor)
- `backend/`: `tactics_grid_backend` deterministic tactics engine + replay binary
- `ui/`: `tactics_grid_ui` UI/orchestrator crate + `ui_manifest.ron`

## Crates

- `tactics_grid_backend`: deterministic grid tactics engine with turn-based combat, AI, and replay
- `tactics_grid_ui`: UI/orchestrator layer (native bridge + wasm app entrypoint)

## Build

```sh
cargo build -p tactics_grid_backend
cargo build -p tactics_grid_ui
```

## Run

```sh
# Run battle
tactics_grid_backend run --seed S --scenario <name|file> --out <path> [--replay-out <path>]

# Replay
tactics_grid_backend replay --replay <path> --out <path>
```

## Core Invariants

- Deterministic initiative order with tie-break rules (speed desc, unit id asc)
- Deterministic AI decisions with deterministic tie-breaks
- Replay yields identical battle report + snapshot hashes
- Canonical JSON outputs

## Public API

- `BattleConfig`: battle configuration (grid size, max turns)
- `Scenario`: battle scenario (units, abilities, config)
- `TurnEngine`: battle execution engine
- `BattleReport`: canonical battle report with run hash
- `ReplayEngine`: deterministic replay validation
- `TacticsGridError`: error type
