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

## Resolved Placement Decisions

The following decisions are finalized and should be treated as architecture policy:

- `protocol` is fixed to `L1` (technical contract layer).
- `ui-lib` (crate under `projects/libraries/ui`) is fixed to `L2`.
- Shared technical crates are fixed as:
  - `L0`: `common_time`, `common_calendar`, `common_binary`, `common_parsing`, `common_tokenize`, `hybrid_arena`, `ast_core`, `ast_macros`, `pjson_proc_macros`, `protocol_macros`.
  - `L1`: `common`, `common_json`, `common_ron`, `command_runner`, `protocol`.
- `ai` remains `L3` and must consume `L2` contracts/facades only.
  Migration target: remove direct `L3 -> L1` edges (notably to `common_json` and `protocol`) via `L2` boundaries before strict closure of migration issues.

## Migration Impact (Current Wave)

- Existing direct `L3 -> L1` and `L2 -> L0` anomalies are migration debt, not policy ambiguity.
- Follow-up refactors must align code to this finalized placement without redefining layers.

## Exception Governance

- Exceptions must be explicit, minimal, and temporary.
- Each whitelist entry must include:
  - a reason,
  - an owner,
  - a review/expiry date.
