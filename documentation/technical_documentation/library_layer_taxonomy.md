# Workspace Library Layer Taxonomy

## Purpose

Define the strict, deterministic layer model for workspace libraries.

## Layer Model

- `L0 Foundation`: ultra-generic technical primitives and utilities.
- `L1 Technical Specialization`: technical adapters/specializations built on `L0` (still not business-domain).
- `L2 Domain`: domain libraries and domain-facing public APIs/contracts.
- `L3 Orchestration`: the only layer allowed to compose/cross multiple domains.

## Strict Dependency Rules (Adjacent-only)

- `L0` must not depend on any workspace crate.
- `L1` may depend on `L0` only.
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

## Exception Governance

- Exceptions must be explicit, minimal, and temporary.
- Each whitelist entry must include:
  - a reason,
  - an owner,
  - a review/expiry date.
