# Market Tycoon

A deterministic tycoon simulation covering supply chain, inventory, dynamic pricing, customer demand, profits, replay, and canonical reporting.

## Binaries

- `market_tycoon_backend`: All simulation and domain logic — supply chain, pricing engine, demand curves, logistics, finance ledger, event log, replay, snapshots, and reports.
- `market_tycoon_ui`: User interface that communicates with the backend through a strict local transport boundary (subprocess CLI or IPC).

## Architecture

- **Backend** is the single source of truth for all simulation state.
- **UI** delegates all business logic to the backend; it contains no simulation code.
- Communication uses subprocess invocation (CLI) or JSON Lines IPC (serve mode).

## Determinism Contract

- All simulation state transitions are pure and deterministic.
- Uses a seeded LCG (Linear Congruential Generator) for all randomness — no external `rand` crate.
- `BTreeMap` used throughout for deterministic iteration order.
- Identical scenario + seed + tick count produces identical report and replay outputs.
- Run hash is computed from canonical state (seed, ticks, event count, net profit) via SHA-256.
- Replay equivalence: replaying the same seed/ticks produces the same run hash.

## CLI Usage

### Backend

```bash
# Run a simulation
market_tycoon_backend run --ticks 100 --seed 42 --scenario scenario.json --out report.json [--replay-out replay.json]

# Replay a previous run
market_tycoon_backend replay --replay replay.json --out report.json

# Snapshot at a specific tick
market_tycoon_backend snapshot --replay replay.json --at-tick 50 --out snapshot.json

# Validate a scenario file
market_tycoon_backend validate --scenario scenario.json

# Start IPC server mode
market_tycoon_backend serve --scenario scenario.json
```

### UI

```bash
# Run simulation via backend
market_tycoon_ui run --scenario scenario.json --seed 42 --ticks 100 --out report.json

# Replay a previous run
market_tycoon_ui replay --replay replay.json --out report.json

# Validate a scenario
market_tycoon_ui scenario --scenario scenario.json
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 2 | CLI error |
| 3 | Invalid scenario/config |
| 4 | Replay mismatch |
| 5 | Internal error |
