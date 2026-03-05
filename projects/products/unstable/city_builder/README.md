# City Builder

Deterministic city simulation product (SimCity-like) implemented as two binaries:

- `backend/`: simulation engine and CLI execution
- `ui/`: native CLI entry + Dioxus WASM bundle entry

## Product Goal

Model a city over deterministic ticks with:

- zoning and building growth
- service coverage (power/water/health/police/fire)
- simplified traffic routing
- economy updates
- replay verification and snapshot export

The backend is deterministic by design: same scenario + seed + scripted actions must produce equivalent report JSON values and replay hashes.

## Repository Layout

```text
city_builder/
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
    src/main.rs
```

## Backend Commands

From workspace root:

```bash
cargo run -p city_builder_backend -- run --ticks 100 --seed 42 --scenario <scenario.json> --out <report.json> [--replay-out <replay.json>]
cargo run -p city_builder_backend -- replay --replay <replay.json> --out <report.json>
cargo run -p city_builder_backend -- snapshot --replay <replay.json> --at-tick 25 --out <snapshot.json>
cargo run -p city_builder_backend -- validate --scenario <scenario.json>
```

Exit codes:

- `0` success
- `2` invalid CLI usage
- `3` invalid scenario/config
- `4` replay mismatch/invalid replay
- `5` internal runtime error

## Tests

```bash
cargo check -p city_builder_backend -p city_builder_ui
cargo test -p city_builder_backend
```

Fixtures are in `backend/tests/fixtures/`.

## Engine / Central UI Promotion Notes

- Product metadata is declared in `metadata.ron` for registry discovery.
- Backend crate metadata is declared in `backend/backend_manifest.ron`.
- UI bundle metadata is declared in `ui/ui_manifest.ron`.
- WASM UI calls `CITY_BUILDER_UI_API_BASE` (default: `/api/city_builder`) for run/replay/snapshot/validate flows.
- For stable promotion, engine and central_ui must expose/forward the `/api/city_builder/*` contract.
