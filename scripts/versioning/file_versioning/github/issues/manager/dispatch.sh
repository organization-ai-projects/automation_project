#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

# shellcheck source=scripts/versioning/file_versioning/github/lib/va.sh
source "${ISSUES_DIR}/../lib/va.sh"

manager_issues_legacy_dispatch() {
  # shellcheck source=scripts/versioning/file_versioning/github/issues/required_fields/load.sh
  source "${ISSUES_DIR}/required_fields/load.sh"
  # shellcheck source=scripts/versioning/file_versioning/github/issues/common/gh.sh
  source "${ISSUES_DIR}/common/gh.sh"
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
  local subcommand="${1:-}"

  if [[ "${VA_MANAGER_WRAPPER_ACTIVE:-0}" == "1" ]]; then
    return 2
  fi

  # Keep legacy path when explicitly requested.
  if [[ "${VA_ISSUES_FORCE_LEGACY:-0}" == "1" ]]; then
    return 2
  fi

  # Keep legacy path for create when custom create script is injected
  # (used by regression harness and local debugging).
  if [[ "$subcommand" == "create" && -n "${MANAGER_ISSUES_CREATE_SCRIPT:-}" ]]; then
    return 2
  fi

  VA_MANAGER_WRAPPER_ACTIVE=1 va_exec issue "$@"
}

manager_issues_run() {
  local status=0
  manager_issues_try_va_dispatch "$@" || status=$?

  # status=2 means "explicitly use legacy path", not a runtime failure.
  if [[ "$status" -eq 2 ]]; then
    manager_issues_legacy_dispatch "$@"
    return $?
  fi

  return "$status"
}
