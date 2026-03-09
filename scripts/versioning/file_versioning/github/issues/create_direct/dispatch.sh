#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

create_direct_issue_legacy_dispatch() {
  # shellcheck source=scripts/versioning/file_versioning/github/issues/required_fields/load.sh
  source "${ISSUES_DIR}/required_fields/load.sh"
  # shellcheck source=scripts/versioning/file_versioning/github/issues/common/cli.sh
  source "${ISSUES_DIR}/common/cli.sh"
  source "${BASH_SOURCE[0]%/*}/cli_contract.sh"
  source "${BASH_SOURCE[0]%/*}/builder.sh"

  local entrypoint_fn="create_direct_issue_run"
  "$entrypoint_fn" "$@"
}

create_direct_issue_try_va_dispatch() {
  local -a va_cmd=()

  if [[ "${VA_CREATE_DIRECT_WRAPPER_ACTIVE:-0}" == "1" ]]; then
    return 1
  fi

  if [[ "${VA_CREATE_DIRECT_FORCE_LEGACY:-0}" == "1" ]]; then
    return 1
  fi

  if command -v va >/dev/null 2>&1; then
    va_cmd=(va issue create)
  elif command -v versioning_automation >/dev/null 2>&1; then
    va_cmd=(versioning_automation issue create)
  else
    return 1
  fi

  VA_CREATE_DIRECT_WRAPPER_ACTIVE=1 "${va_cmd[@]}" "$@"
}

create_direct_issue_dispatch() {
  if create_direct_issue_try_va_dispatch "$@"; then
    return 0
  fi
  create_direct_issue_legacy_dispatch "$@"
}

create_direct_issue_entry_run() {
  create_direct_issue_dispatch "$@"
}
