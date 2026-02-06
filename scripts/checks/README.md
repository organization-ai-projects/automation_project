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

## Adding New Checks

When adding new validation scripts:
1. Create the script in this directory
2. Make it executable: `chmod +x script_name.sh`
3. Add documentation here in this README
4. Consider adding a CI workflow in `.github/workflows/`
5. Follow the exit code convention: 0 = success, non-zero = failure
