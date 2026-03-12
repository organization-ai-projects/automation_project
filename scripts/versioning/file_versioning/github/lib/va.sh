#!/usr/bin/env bash
# shellcheck shell=bash

# Shared helper for invoking the Rust versioning automation CLI.

va_exec() {
  local script_dir repo_root local_bin
  script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  repo_root="$(cd "${script_dir}/../../../../.." && pwd)"
  local_bin="${VA_LOCAL_BIN:-${repo_root}/target/debug/versioning_automation}"

  if command -v va >/dev/null 2>&1; then
    va "$@"
    return $?
  fi
  if [[ -x "$local_bin" ]]; then
    "$local_bin" "$@"
    return $?
  fi
  if command -v versioning_automation >/dev/null 2>&1; then
    versioning_automation "$@"
    return $?
  fi
  if command -v cargo >/dev/null 2>&1 && [[ -f "${repo_root}/Cargo.toml" ]]; then
    cargo run -q -p versioning_automation -- "$@"
    return $?
  fi
  return 127
}
