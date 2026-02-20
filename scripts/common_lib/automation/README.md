# Automation Shared Library

This directory contains shared automation helpers reused by git hooks,
CI workflows, and standalone scripts.

Current modules:

- `file_types.sh`: file classifiers (docs, tests, workflows, scripts, shell).
- `scope_resolver.sh`: path-to-scope and path-to-crate resolution helpers.
- `change_policy.sh`: shared predicates for staged/changed files (docs-only, mixed changes, multi-scope).
- `rust_checks.sh`: shared cargo check runners (`check`, `fmt`, `clippy`, `test`).
