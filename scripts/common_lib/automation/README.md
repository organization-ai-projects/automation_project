# Automation Shared Library

This directory contains shared automation helpers reused by git hooks,
CI workflows, and standalone scripts.

Current modules:

- `file_types.sh`: file classifiers (docs, tests, workflows, scripts, shell).
- `scope_resolver.sh`: path-to-scope and path-to-crate resolution helpers.
- `rust_checks.sh`: shared cargo check runners (`fmt`, `clippy`, `test`).
