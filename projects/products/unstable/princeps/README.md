# princeps

Deterministic satirical political campaign simulation product.

## Binaries

- `princeps_backend`: deterministic campaign simulation, replay, and canonical reporting.
- `princeps_ui`: UI entrypoint crate.

## Backend Commands

- `run --days N --seed S --json [--replay-out <replay.json>]`
- `replay <replay.json> --json`
- `export --format markdown|json`
