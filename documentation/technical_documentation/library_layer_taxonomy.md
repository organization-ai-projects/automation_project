# Workspace Library Layer Taxonomy

## Purpose

Define a single layer model for all workspace crates and enforce dependency direction rules.

## Layer Model

- `L0 Foundation`: shared primitives, format helpers, macros, parsing/tokenizing/time/calendar, and core infra utilities.
- `L1 Domain`: business/domain libraries built on foundation crates.
- `L2 Interface`: interface and boundary libraries (contracts/UI-level abstractions).
- `L3 Applications`: runnable products/components (stable and unstable products).

## Dependency Direction Rules

- `L0` may depend only on `L0`.
- `L1` may depend on `L0` and `L1`.
- `L2` may depend on `L0`, `L1`, and `L2`.
- `L3` may depend on `L0`, `L1`, and `L2`.
- Upward dependencies are forbidden (for example: `L0 -> L1/L2/L3`, `L1 -> L2/L3`, `L2 -> L3`).

## Allowed Dependency Examples

1. `projects/libraries/symbolic` (`L1`) -> `projects/libraries/common` (`L0`)
2. `projects/libraries/ui` (`L2`) -> `projects/libraries/protocol` (`L2`)
3. `projects/products/stable/accounts/backend` (`L3`) -> `projects/libraries/security` (`L1`)

## Forbidden Dependency Examples

1. `projects/libraries/common` (`L0`) -> `projects/libraries/symbolic` (`L1`)
2. `projects/libraries/protocol` (`L2`) -> `projects/products/stable/core/engine` (`L3`)
3. `projects/libraries/identity` (`L1`) -> `projects/libraries/ui` (`L2`)

## Workspace Crate-to-Layer Mapping

Each workspace member is assigned to exactly one layer.

| Workspace member | Layer |
|---|---|
| `projects/libraries/ai` | `L1 Domain` |
| `projects/libraries/ast_core` | `L0 Foundation` |
| `projects/libraries/ast_macros` | `L0 Foundation` |
| `projects/libraries/command_runner` | `L0 Foundation` |
| `projects/libraries/common` | `L0 Foundation` |
| `projects/libraries/common_binary` | `L0 Foundation` |
| `projects/libraries/common_calendar` | `L0 Foundation` |
| `projects/libraries/common_json` | `L0 Foundation` |
| `projects/libraries/common_parsing` | `L0 Foundation` |
| `projects/libraries/common_ron` | `L0 Foundation` |
| `projects/libraries/common_time` | `L0 Foundation` |
| `projects/libraries/common_tokenize` | `L0 Foundation` |
| `projects/libraries/hybrid_arena` | `L0 Foundation` |
| `projects/libraries/identity` | `L1 Domain` |
| `projects/libraries/neural` | `L1 Domain` |
| `projects/libraries/pjson_proc_macros` | `L0 Foundation` |
| `projects/libraries/protocol` | `L2 Interface` |
| `projects/libraries/protocol_macros` | `L0 Foundation` |
| `projects/libraries/security` | `L1 Domain` |
| `projects/libraries/symbolic` | `L1 Domain` |
| `projects/libraries/ui` | `L2 Interface` |
| `projects/libraries/versioning` | `L1 Domain` |
| `projects/products/stable/accounts/backend` | `L3 Applications` |
| `projects/products/stable/accounts/ui` | `L3 Applications` |
| `projects/products/stable/code_agent_sandbox` | `L3 Applications` |
| `projects/products/stable/core/central_ui` | `L3 Applications` |
| `projects/products/stable/core/engine` | `L3 Applications` |
| `projects/products/stable/core/launcher` | `L3 Applications` |
| `projects/products/stable/core/watcher` | `L3 Applications` |
| `projects/products/stable/varina/backend` | `L3 Applications` |
| `projects/products/stable/varina/ui` | `L3 Applications` |
| `projects/products/unstable/auto_manager_ai` | `L3 Applications` |
| `projects/products/unstable/autonomous_dev_ai` | `L3 Applications` |

## Enforcement Follow-up

This document defines the policy baseline. Automated CI boundary checks are handled in the dedicated enforcement follow-up issue.
