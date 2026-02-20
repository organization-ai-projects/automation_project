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

- `L1` -> no layer dependency
- `L2` -> `L1` only
- `L3` -> `L2` only

Layer intent:

- `L1` is for technical building blocks used by `L2` domains.
- `core/*` (`core/foundation`, `core/contracts`) is outside `L1..L3` and governed by the core overlay rules below.

Additional constraints:

- no upward dependencies
- no lateral dependencies by default (unless explicitly whitelisted)
- exceptions must be explicit, temporary, and governed

## Core Overlay (Checker-managed)

Strict checks also apply a checker-managed core overlay for selected crates.

Overlay policy:

- `layer -> core`: allowed
- `core -> layer`: forbidden
- `core -> core`: allowed

Current core groups:

- `core/foundation`: `ast_core`, `ast_macros`, `command_runner`, `common`, `common_binary`, `common_calendar`, `common_json`, `common_parsing`, `common_ron`, `common_time`, `common_tokenize`, `hybrid_arena`, `pjson_proc_macros`, `protocol_macros`
- `core/contracts`: `protocol`, `security_core`

If violated, the checker reports class `core-to-layer`.

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
