# Automation Shared Library

This directory contains shared automation helpers reused by git hooks,
CI workflows, and standalone scripts.

Current modules:

- `file_types.sh`: file classifiers (docs, tests, workflows, scripts, shell).
- `scope_resolver.sh`: orchestrates scope/crate resolution across workspace and fallback modes.
- `workspace_rust.sh`: workspace-first scope resolution from `Cargo.toml` workspace members.
- `non_workspace_rust.sh`: fallback scope resolution using nearest `Cargo.toml`.
- `change_policy.sh`: shared predicates for staged/changed files (docs-only, mixed changes, multi-scope).
- `rust_checks.sh`: shared cargo check runners (`check`, `fmt`, `clippy`, `test`).

## Scope Mapping Rules

| Staged file set | Required scope |
|---|---|
| Files mapped to Rust workspace members under `projects/**` | Matching crate/product scope from resolver |
| Shell-only files (`*.sh`) with no Rust scope | `shell` |
| Markdown-only files (`*.md`) with no Rust scope | `markdown` |
| Mixed categories (`rust/shell/markdown/other`) | Blocked by `commit-msg` policy |

## Debug Helper

Use `scripts/automation/explain_scope.sh` to inspect:
- staged files
- per-file resolved scope/candidate
- detected format categories
- final required scope set used by `commit-msg`
