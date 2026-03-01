# Theme Park

Deterministic theme park management game with strict backend/UI separation.

## Architecture

- `backend/` — headless simulation server (all game logic)
- `ui/` — client (zero business logic; communicates only via IPC)

## IPC Protocol

JSON Lines over stdin/stdout. Each message: `{"id": N, "payload": {...}}`.

## Usage

```bash
# Run the backend server (scenario path required)
theme_park_backend serve --scenario scenario.json

# Run the UI client
theme_park_ui run --scenario scenario.json --seed 42 --ticks 200 [--out report.json] [--replay replay.json]
```

## Exit Codes

### Backend
- `0` — clean shutdown
- `2` — invalid CLI usage
- `3` — invalid scenario/config
- `5` — internal error

### UI
- `0` — clean shutdown
- `2` — invalid CLI usage
- `5` — internal error

## Determinism Guarantee

Same `seed` + same `scenario` + same command stream ⟹ identical `EventLog`, `SnapshotHash`, `RunReport`, and `RunHash`.

No wall-clock time in simulation core (`SystemTime`, `Instant`, `chrono` are forbidden in sim logic).
All entity iteration uses stable sorted order (BTreeMap / explicit sort). No HashMap iteration in sim.

## Request Types

| Request           | Description                           |
|-------------------|---------------------------------------|
| Ping              | Health check                          |
| LoadScenario      | Load scenario from file               |
| NewRun            | Start a new simulation run            |
| Step              | Advance N ticks                       |
| RunToEnd          | Run until all visitors leave          |
| GetSnapshot       | Get state snapshot at a tick          |
| GetReport         | Get the canonical run report          |
| SaveReplay        | Save replay file                      |
| LoadReplay        | Load replay file                      |
| ReplayToEnd       | Replay to completion                  |
| Shutdown          | Stop the backend                      |

## Response Types

| Response          | Description                           |
|-------------------|---------------------------------------|
| Ok                | Success acknowledgement               |
| Error             | Error with code, message, details     |
| Snapshot          | State snapshot summary                |
| Report            | Full run report with run_hash         |

## Error Codes

| Code               | Meaning                              |
|--------------------|--------------------------------------|
| PARSE_ERROR        | Could not parse JSON input           |
| INVALID_REQUEST    | Unknown or malformed request type    |
| NO_RUN             | No active run                        |
| NO_SCENARIO        | Scenario not loaded                  |
| INVALID_SCENARIO   | Scenario file invalid                |
| REPLAY_MISMATCH    | Replay hash does not match           |
| IO_ERROR           | File I/O failure                     |
| INTERNAL_ERROR     | Unexpected internal failure          |
