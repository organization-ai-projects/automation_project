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

**Purpose**: Enforces strict adjacent-only workspace layer dependency boundaries.

**Usage**:

```bash
./scripts/checks/check_layer_boundaries.sh
```

Strict mode:

```bash
./scripts/checks/check_layer_boundaries.sh --strict
```

**What it does**:

- Runs `cargo metadata` to inspect workspace crate dependency edges
- Default mode enforces:
  - `library -> product` forbidden
- `--strict` mode additionally enforces:
  - `library -> product` forbidden
  - `L0 -> no workspace deps`
  - `L1 -> L0` only
  - `L2 -> L1` only
  - `L3 -> L2` only
  - no lateral/upward/non-adjacent edges by default
  - `L1` semantics: technical building blocks for `L2` domains
- Uses:
  - `scripts/checks/layer_map.txt` (canonical `crate -> layer`)
  - built-in checker core overlay (`foundation|contracts|none`, script-managed)
    - core is outside numeric layering (`L0..L3`)
    - `layer -> core` allowed
    - `core -> layer` forbidden
    - `core -> core` allowed
  - `scripts/checks/layer_whitelist.txt` (governed temporary exceptions)
- Emits stable actionable diagnostics:
  - `VIOLATION class=<class> edge=<from>(<layer>)-><to>(<layer>) suggestion="<remediation>"`
  - Classes include `library-to-product`, `core-to-layer`, `foundation-internal`, `lateral`, `upward`, `non-adjacent`, `unmapped`

**CI Integration**: Runs automatically in `.github/workflows/ci_reusable.yml`.

**Related Documentation**: See `documentation/technical_documentation/library_layer_boundaries.md`.

### analyze_layer_anomalies.sh

**Purpose**: Semi-automated architectural analysis helper for strict adjacent-only layering decisions.

**Usage**:

```bash
./scripts/checks/analyze_layer_anomalies.sh \
  --json-out /tmp/layer_anomalies.json
```

Optional:

- `--map-file <path>` to override canonical crate-to-layer assumptions.
- `--protocol-layer <L1|L2|UNDECIDED>` is deprecated and ignored (kept for backward compatibility).
- `--fail-on-anomaly true` to use it as a failing check in experimentation pipelines.

**What it does**:

- Runs `cargo metadata` and extracts workspace dependency edges
- Builds a layer view from `scripts/checks/layer_map.txt` when present
- Falls back to provisional built-in mapping when no map file is available
- Reports:
  - `library -> product` edges
  - foundation internal dependencies
  - lateral / upward / non-adjacent edges
  - unmapped crates and ambiguous hotspots
  - cycle signals (`tsort`-based signal)
- Can emit both human-readable output and JSON.

### layer_map.txt

**Purpose**: Canonical machine-readable `crate -> layer` mapping for workspace libraries.

**Format**:

```text
crate_name=L0|L1|L2|L3|UNMAPPED
```

**Current location**: `scripts/checks/layer_map.txt`

## Adding New Checks

When adding new validation scripts:

1. Create the script in this directory
2. Make it executable: `chmod +x script_name.sh`
3. Add documentation here in this README
4. Consider adding a CI workflow in `.github/workflows/`
5. Follow the exit code convention: 0 = success, non-zero = failure
