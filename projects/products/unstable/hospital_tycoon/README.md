# Hospital Tycoon

A deterministic hospital management simulation with IPC-based backend/UI split.

## Structure

- `backend/` — `hospital_tycoon_backend` binary: deterministic simulation engine
- `ui/` — `hospital_tycoon_ui` binary: orchestrator and report renderer

## Building

```sh
cargo build -p hospital_tycoon_backend
cargo build -p hospital_tycoon_ui
```

## Running

```sh
# Run a scenario
hospital_tycoon_ui run --scenario scenario.json --seed 42 --ticks 100

# Replay
hospital_tycoon_ui replay --replay replay.json
```

## Backend IPC Protocol (JSON Lines)

Requests: `Ping`, `LoadScenario`, `NewRun`, `Step`, `RunToEnd`, `GetSnapshot`, `GetReport`, `SaveReplay`, `LoadReplay`, `ReplayToEnd`

Responses: `Ok`, `Error`, `Snapshot`, `Report`
