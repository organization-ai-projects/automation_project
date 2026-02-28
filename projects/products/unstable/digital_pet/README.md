# digital_pet

A deterministic Digimon 1-like digital pet simulator. Strict backend/UI separation via IPC.

## Architecture

Two binary crates:

- `backend/` — simulation engine, IPC server, all business logic
- `ui/` — thin terminal client, zero business logic, communicates via JSON-lines IPC

## Features

- Pet lifecycle: hunger, fatigue, happiness, discipline, sickness
- Care mistake tracking
- Deterministic digivolution via data-driven `EvolutionTree`
- Simple battle/arena system
- Replay: identical seed + scenario + actions = identical evolution, identical `RunHash`
- Canonical JSON reports with stable `RunHash`

## Usage

### Backend

```bash
digital_pet_backend serve --scenario scenario.json
```

### UI (spawns backend automatically)

```bash
digital_pet_ui run --scenario scenario.json --seed 42 --ticks 100 --out report.json
```

## IPC Protocol

JSON-lines over stdin/stdout. Each message: `{"id": N, "request": {...}}`.

### Requests

| Type | Fields |
|---|---|
| `NewRun` | `seed`, `ticks` |
| `Step` | `n` |
| `CareAction` | `kind` |
| `Training` | `kind` |
| `StartBattle` | — |
| `BattleStep` | — |
| `GetSnapshot` | — |
| `GetReport` | — |
| `SaveReplay` | `path` |
| `LoadReplay` | `path` |
| `ReplayToEnd` | — |
| `LoadScenario` | `path` |

### Responses

`Ok`, `Error`, `PetState`, `BattleState`, `Snapshot`, `Report`

## Core Invariants

- No `SystemTime` in core; ticks only
- Same seed + same scenario + same care actions ⟹ identical outcome, identical `RunHash`
- Digivolution rules are data-driven from `EvolutionTree`
- Replay reproduces identical pet state and evolution history
