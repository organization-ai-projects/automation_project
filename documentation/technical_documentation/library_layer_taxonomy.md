# Workspace Library Layer Taxonomy

## Purpose

Define the strict, deterministic layer model for workspace libraries.

## Layer Model

- `L1 Technical Building Blocks`: technical building blocks used to compose domain crates (`L2`), still not business-domain.
- `L2 Domain`: domain libraries and domain-facing public APIs/contracts.
- `L3 Orchestration`: the only layer allowed to compose/cross multiple domains.

## Core Overlay Model

The checker also applies a core overlay that is orthogonal to numeric layers.

- `core/foundation`: shared internal technical building blocks.
- `core/contracts`: shared cross-cutting contracts/protocol crates.

Core is outside numeric layering (`L1..L3`): it is not above or below the layered model.

Overlay enforcement:

- `layer -> core` allowed
- `core -> layer` forbidden
- `core -> core` allowed

## Strict Dependency Rules (Adjacent-only)

- `L1` must not depend on any layer crate.
- `L2` may depend on `L1` only.
- `L3` may depend on `L2` only.
- Upward dependencies are forbidden.
- Lateral dependencies are forbidden by default (`L1 -> L1`, `L2 -> L2`, `L3 -> L3`), unless explicitly whitelisted.

## Checker Behavior Contract

- Layer checks evaluate workspace crate edges only.
- External crates are ignored for layer-direction rules.
- `path`/workspace dependencies are treated exactly like named workspace dependencies.
- Enforcement scope targets `dependencies` and `build-dependencies` by default.
- `dev-dependencies` are excluded by default.

## Layer Placement Guidance

- Purely technical/shared contracts belong to `L1`.
- Domain-facing contracts belong to `L2`.
- `L3` stays orchestration-only and should consume `L2` contracts, not internal `L1` implementation details.
- `L1` should expose reusable technical components that help implement `L2` domains.

## Resolved Placement Decisions

The following decisions are finalized and should be treated as architecture policy:

- `protocol` is fixed as a `core/contracts` crate (not a layer level).
- `security_core` is fixed as a `core/contracts` crate (not a layer level).
- `ui-lib` (crate under `projects/libraries/layers/domain/ui`) is fixed to `L2`.
- Shared technical crates previously mapped as low-level layers are now treated as `core/foundation` (not layer levels).
- `ai` remains `L3` and must consume `L2` contracts/facades only.
- Product-facing policy: products should depend on `ai` for AI workflows; direct product dependencies on `neural` or `symbolic` are not part of the target architecture.
  Migration target: remove direct `L3 -> L1` edges (notably to `common_json`) via `L2` boundaries before strict closure of migration issues.

## Migration Impact (Current Wave)

- Existing direct `L3 -> L1` anomalies are migration debt, not policy ambiguity.
- Follow-up refactors must align code to this finalized placement without redefining layers.

## Exception Governance

- Exceptions must be explicit, minimal, and temporary.
- Each whitelist entry must include:
  - a reason,
  - an owner,
  - a review/expiry date.
