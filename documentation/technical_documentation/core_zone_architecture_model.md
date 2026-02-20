# Core Zone Architecture Model (Draft RFC)

## Purpose

Define a practical architecture model that matches real workspace usage better than strict numeric layer adjacency.

## Problem

The current `L1/L2/L3` adjacent-only model is too rigid for shared internal-publishable libraries such as:

- `common_json` (internal JSON abstraction)
- `common_time` (internal time abstraction)
- `protocol` (cross-cutting contracts)

This creates repeated "violations" even when dependencies are architecturally intentional.

## Proposed Model

Use logical zones:

- `core/foundation`: shared technical building blocks.
- `core/contracts`: shared contracts and cross-cutting protocol types.
- `domain`: business/domain libraries.
- `orchestration`: composition/integration libraries.

## Dependency Rules (v1)

- `core/foundation` -> `core/foundation` only.
- `core/contracts` -> `core/foundation` (and optional `core/contracts`, to validate explicitly).
- `domain` -> `core/foundation` + `core/contracts`.
- `orchestration` -> `domain` + `core/contracts` + `core/foundation`.
- Forbidden:
  - upward dependencies,
  - direct dependency on orchestration from lower zones,
  - uncontrolled lateral domain coupling.

## Migration Strategy

1. Update architecture documentation and checker semantics first.
2. Keep current folder layout temporarily; migrate by logical mapping first.
3. Reclassify open strict-layer issues under this model.
4. Execute incremental refactors only where still required after reclassification.
5. Move folders physically later (`projects/libraries/core/...`) if desired.

## Scope Boundary

This RFC is a model-definition step, not a full refactor PR.

Hierarchy:

- Parent: #565
