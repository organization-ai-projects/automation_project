# diplo_sim

Deterministic diplomacy-like strategy simulation product.

## Binaries

- `diplo_sim_backend`: deterministic adjudication, replay, and reporting.
- `diplo_sim_ui`: UI entrypoint crate.

## Backend Commands

- `run --turns N --seed S (--map <file> | --map-id <id>) --players P --out <report.json> [--replay-out <replay.json>]`
- `replay --replay <replay.json> --out <report.json>`
- `validate-map --map <file>`
- `validate-orders --map <file> --orders <file>`
- `list-maps --out <maps.json>`
