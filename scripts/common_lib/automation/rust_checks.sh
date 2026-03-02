#!/usr/bin/env bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# Shared Rust check runner helpers used by hooks, CI workflows, and scripts.

rust_checks_has_lockfile() {
  [[ -f Cargo.lock ]]
}

rust_checks_run_check() {
  cargo check "$@"
}

rust_checks_run_fmt_check() {
  cargo fmt --all -- --check
}

rust_checks_run_clippy() {
  cargo clippy "$@" -- -D warnings
}

rust_checks_run_tests() {
  cargo test "$@"
}
