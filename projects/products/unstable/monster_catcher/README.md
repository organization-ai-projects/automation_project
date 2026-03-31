# monster_catcher

Deterministic Pokémon-like monster catcher with backend+ui binaries, JSON IPC, seeded RNG, turn-based combat, capture, progression, and replay.

## Architecture

- `backend/` — bin crate (`monster_catcher_backend`) with all game logic
- `ui/` — bin crate (`monster_catcher_ui`) with zero business logic, IPC-only

## Backend CLI

```
monster_catcher_backend serve [--scenario <file>]
```

Reads JSON-line requests from stdin, writes JSON-line responses to stdout.

## UI CLI

```
monster_catcher_ui run [--backend <path>]
```

Spawns backend process and communicates via stdin/stdout IPC.

## Core Invariants

- Same seed + same scenario + same commands ⇒ identical RNG draws, event log, battle reports, RunReport, RunHash
- RNG draws are logged as `RngDraw` events
- Type effectiveness is deterministic and data-driven (TypeChart)
- Canonical JSON with stable key ordering everywhere
- IPC uses strict JSON lines with request id
