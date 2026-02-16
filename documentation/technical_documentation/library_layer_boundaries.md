# Library Layer Boundaries

This document defines the automated dependency boundary enforced in CI.

## Enforced Rule

- Crates under `projects/libraries/` must not depend on crates under `projects/products/`.

Allowed direction:

- `projects/products/*` -> `projects/libraries/*`

Forbidden direction:

- `projects/libraries/*` -> `projects/products/*`

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
