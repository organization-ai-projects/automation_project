# Check Scripts

This directory contains validation and check scripts for the repository.

## Available Checks

### check_stable_deps.sh

**Purpose**: Enforces Rule 2 of the stable/unstable product structure - prevents stable products from depending on unstable products.

**Usage**:

```bash
./scripts/checks/check_stable_deps.sh
```

**What it does**:

- Scans all `Cargo.toml` files in `projects/products/stable/`
- Looks for path dependencies pointing to `projects/products/unstable/`
- Reports any violations found
- Returns exit code 0 on success, 1 on failure

**CI Integration**: Runs automatically on PRs and pushes via `.github/workflows/check_stable_deps.yml`

**Related Documentation**: See `projects/products/README.md` for the complete stable/unstable product structure rules.

### check_layer_boundaries.sh

**Purpose**: Enforces workspace layer dependency boundaries to prevent architectural drift.

**Usage**:

```bash
./scripts/checks/check_layer_boundaries.sh
```

**What it does**:

- Runs `cargo metadata` to inspect workspace crate dependency edges
- Classifies crates by path:
  - `projects/libraries/*` => `library`
  - `projects/products/*` => `product`
- Fails when a `library -> product` dependency edge is detected

**CI Integration**: Runs automatically in `.github/workflows/ci_reusable.yml`.

**Related Documentation**: See `documentation/technical_documentation/library_layer_boundaries.md`.

### analyze_layer_anomalies.sh

**Purpose**: Semi-automated architectural analysis helper for strict adjacent-only layering decisions.

**Usage**:

```bash
./scripts/checks/analyze_layer_anomalies.sh \
  --protocol-layer UNDECIDED \
  --json-out /tmp/layer_anomalies.json
```

Optional:

- `--map-file <path>` to override provisional crate-to-layer assumptions.
- `--fail-on-anomaly true` to use it as a failing check in experimentation pipelines.

**What it does**:

- Runs `cargo metadata` and extracts workspace dependency edges
- Builds a provisional layer view (optionally overridden by map file)
- Reports:
  - `library -> product` edges
  - foundation internal dependencies
  - lateral / upward / non-adjacent edges
  - unmapped crates and ambiguous hotspots
  - cycle signals (`tsort`-based signal)
- Can emit both human-readable output and JSON.

## Adding New Checks

When adding new validation scripts:

1. Create the script in this directory
2. Make it executable: `chmod +x script_name.sh`
3. Add documentation here in this README
4. Consider adding a CI workflow in `.github/workflows/`
5. Follow the exit code convention: 0 = success, non-zero = failure
