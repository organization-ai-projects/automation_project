#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

manager_issues_legacy_dispatch() {
  # shellcheck source=scripts/versioning/file_versioning/github/issues/required_fields/load.sh
  source "${ISSUES_DIR}/required_fields/load.sh"
  # shellcheck source=scripts/versioning/file_versioning/github/issues/common/cli.sh
  source "${ISSUES_DIR}/common/cli.sh"
  # shellcheck source=scripts/versioning/file_versioning/github/issues/common/refs.sh
  source "${ISSUES_DIR}/common/refs.sh"
  source "${BASH_SOURCE[0]%/*}/cli.sh"
  source "${BASH_SOURCE[0]%/*}/create.sh"
  source "${BASH_SOURCE[0]%/*}/read.sh"
  source "${BASH_SOURCE[0]%/*}/update.sh"
  source "${BASH_SOURCE[0]%/*}/state.sh"

  local entrypoint_fn="manager_issues_main"
  "$entrypoint_fn" "$@"
}

manager_issues_try_va_dispatch() {
  local -a va_cmd=()

  if [[ "${VA_MANAGER_WRAPPER_ACTIVE:-0}" == "1" ]]; then
    return 1
  fi

  # Keep legacy path when explicitly requested or when custom create script is injected
  # (used by regression harness and local debugging).
  if [[ "${VA_ISSUES_FORCE_LEGACY:-0}" == "1" || -n "${MANAGER_ISSUES_CREATE_SCRIPT:-}" ]]; then
    return 1
  fi

  if command -v va >/dev/null 2>&1; then
    va_cmd=(va issue)
  elif command -v versioning_automation >/dev/null 2>&1; then
    va_cmd=(versioning_automation issue)
  else
    return 1
  fi

  VA_MANAGER_WRAPPER_ACTIVE=1 "${va_cmd[@]}" "$@"
}

manager_issues_run() {
  if manager_issues_try_va_dispatch "$@"; then
    return 0
  fi
  manager_issues_legacy_dispatch "$@"
}
