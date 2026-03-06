#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

ISSUES_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
CREATE_DIRECT_ISSUE_SCRIPT="${MANAGER_ISSUES_CREATE_SCRIPT:-${ISSUES_DIR}/create_direct_issue.sh}"

# shellcheck source=scripts/versioning/file_versioning/github/issues/required_fields/module.sh
source "${ISSUES_DIR}/required_fields/module.sh"
source "${BASH_SOURCE[0]%/*}/common.sh"
source "${BASH_SOURCE[0]%/*}/create.sh"
source "${BASH_SOURCE[0]%/*}/read.sh"
source "${BASH_SOURCE[0]%/*}/update.sh"
source "${BASH_SOURCE[0]%/*}/state.sh"

manager_issues_main() {
  local subcommand="${1:-}"
  if [[ -z "$subcommand" ]]; then
    usage
    exit 2
  fi
  shift || true

  case "$subcommand" in
  create) cmd_create "$@" ;;
  read) cmd_read "$@" ;;
  update) cmd_update "$@" ;;
  close) cmd_close "$@" ;;
  reopen) cmd_reopen "$@" ;;
  delete) cmd_delete "$@" ;;
  -h | --help) usage ;;
  *) die_usage "Unknown subcommand: $subcommand" ;;
  esac
}
