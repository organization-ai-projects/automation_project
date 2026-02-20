#!/usr/bin/env bash

# Shared Rust check runner helpers used by hooks and CI workflows.

rust_checks_has_lockfile() {
  [[ -f Cargo.lock ]]
}

rust_checks_run_fmt_check() {
  # Use rustfmt check mode consistently across local hooks and CI.
  cargo fmt --all -- --check
}

rust_checks_run_clippy() {
  cargo clippy "$@" -- -D warnings
}

rust_checks_run_tests() {
  cargo test "$@"
}
