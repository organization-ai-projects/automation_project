# Colony Manager Backend

Deterministic simulation backend for Colony Manager.

## Role

This crate executes simulation and replay flows and owns deterministic state transitions.

## Responsibilities

- Run deterministic simulation ticks from a scenario + seed
- Persist canonical run reports
- Persist/load replay files with deterministic RNG draw sequences
- Validate replay consistency (`ReplayMismatch` on divergence)

## Binary

Package: `colony_manager_backend`

Commands:

- `run --ticks N --seed S [--scenario <path>] --out <report.json> [--replay-out <replay.json>]`
- `replay --replay <replay.json> --out <report.json>`
