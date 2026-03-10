#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

# shellcheck source=scripts/versioning/file_versioning/github/lib/va.sh
source "${ISSUES_DIR}/../lib/va.sh"

reevaluate_legacy_dispatch() {
  # shellcheck source=scripts/versioning/file_versioning/github/lib/issue_refs.sh
  source "${ISSUES_DIR}/../lib/issue_refs.sh"
  # shellcheck source=scripts/versioning/file_versioning/github/issues/common/gh.sh
  source "${ISSUES_DIR}/common/gh.sh"
  # shellcheck source=scripts/versioning/file_versioning/github/issues/common/cli.sh
  source "${ISSUES_DIR}/common/cli.sh"
  # shellcheck source=scripts/versioning/file_versioning/github/issues/common/refs.sh
  source "${ISSUES_DIR}/common/refs.sh"
  source "${BASH_SOURCE[0]%/*}/cli.sh"

  local entrypoint_fn="reevaluate_main"
  "$entrypoint_fn" "$@"
}

reevaluate_try_va_dispatch() {
  if [[ "${VA_REEVALUATE_WRAPPER_ACTIVE:-0}" == "1" ]]; then
    return 1
  fi

  if [[ "${VA_REEVALUATE_FORCE_LEGACY:-0}" == "1" ]]; then
    return 1
  fi

  if ! command -v va_exec >/dev/null 2>&1; then
    return 1
  fi

  VA_REEVALUATE_WRAPPER_ACTIVE=1 va_exec issue reevaluate "$@"
}

reevaluate_dispatch() {
  if reevaluate_try_va_dispatch "$@"; then
    return 0
  fi
  reevaluate_legacy_dispatch "$@"
}

reevaluate_entry_run() {
  reevaluate_dispatch "$@"
}
