#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

PR_DIRECTIVE_CONFLICT_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
ROOT_GITHUB_DIR="$(cd "${PR_DIRECTIVE_CONFLICT_DIR}/.." && pwd)"

source "${ROOT_GITHUB_DIR}/lib/va.sh"

if [[ "${VA_PR_DIRECTIVE_CONFLICT_GUARD_WRAPPER_ACTIVE:-0}" != "1" && "${VA_PR_DIRECTIVE_CONFLICT_GUARD_FORCE_LEGACY:-0}" != "1" ]]; then
  if command -v va_exec >/dev/null 2>&1; then
    set +e
    VA_PR_DIRECTIVE_CONFLICT_GUARD_WRAPPER_ACTIVE=1 va_exec pr directive-conflict-guard "$@"
    va_status=$?
    set -e
    if [[ "$va_status" -ne 127 ]]; then
      exit "$va_status"
    fi
  fi
fi

source "${ROOT_GITHUB_DIR}/lib/gh_cli.sh"
source "${ROOT_GITHUB_DIR}/lib/issue_refs.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh
source "${ROOT_GITHUB_DIR}/../../../common_lib/versioning/file_versioning/github/issue_helpers.sh"
source "${PR_DIRECTIVE_CONFLICT_DIR}/cli.sh"
source "${PR_DIRECTIVE_CONFLICT_DIR}/helpers.sh"
source "${PR_DIRECTIVE_CONFLICT_DIR}/github.sh"
source "${PR_DIRECTIVE_CONFLICT_DIR}/workflow.sh"

pr_directive_conflict_guard_run "$@"
