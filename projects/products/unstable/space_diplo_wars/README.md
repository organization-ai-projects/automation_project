# Space Diplo Wars

Deterministic hybrid strategy simulation product with:

- economy ticks
- simultaneous strategic order resolution at turn boundaries
- diplomacy treaties and influence
- deterministic war resolution
- replay/snapshot reproducibility

## Layout

```text
space_diplo_wars/
  README.md
  metadata.ron
  backend/
    Cargo.toml
    backend_manifest.ron
    src/
    tests/
  ui/
    Cargo.toml
    ui_manifest.ron
    src/
```

## Backend CLI

```bash
cargo run -p space_diplo_wars_backend -- run --turns N --ticks-per-turn K --seed S --scenario <file> --out <report.json> [--replay-out <replay.json>]
cargo run -p space_diplo_wars_backend -- replay --replay <replay.json> --out <report.json>
cargo run -p space_diplo_wars_backend -- snapshot --replay <replay.json> --at-turn T --out <snapshot.json>
cargo run -p space_diplo_wars_backend -- validate --scenario <file>
```

Exit codes:

- `0`: success
- `2`: invalid CLI usage
- `3`: invalid scenario/config/orders
- `4`: replay mismatch / invalid replay
- `5`: internal error
