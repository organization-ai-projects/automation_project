# Library Layer Boundaries

This document defines boundary checks for workspace dependencies and their target strict model.

## Current Enforced Rule

- Crates under `projects/libraries/` must not depend on crates under `projects/products/`.

Allowed direction:

- `projects/products/*` -> `projects/libraries/*`

Forbidden direction:

- `projects/libraries/*` -> `projects/products/*`

## Target Strict Rule Set (Adjacent-only)

For workspace libraries, dependency direction follows:

- `L0` -> no workspace dependencies
- `L1` -> `L0` only
- `L2` -> `L1` only
- `L3` -> `L2` only

Additional constraints:

- no upward dependencies
- no lateral dependencies by default (unless explicitly whitelisted)
- exceptions must be explicit, temporary, and governed

## Checker Scope

- Evaluate workspace crate edges only.
- Ignore external crate edges for layer-direction checks.
- Treat `path`/workspace dependencies exactly like named workspace dependencies.
- Include `dependencies` and `build-dependencies` by default.
- Exclude `dev-dependencies` by default.

## Validation

CI runs:

```bash
./scripts/checks/check_layer_boundaries.sh
```

The check uses `cargo metadata` to inspect workspace dependency edges and fails when a forbidden edge is found.

## Fix Guidance

If CI reports a forbidden edge:

1. Move shared logic into an appropriate crate under `projects/libraries/`.
2. Make product crates consume the shared library crate.
3. Remove direct product coupling from library crates.
