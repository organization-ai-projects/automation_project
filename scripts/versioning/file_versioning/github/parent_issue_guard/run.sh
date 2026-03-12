#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

PARENT_GUARD_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
ROOT_GITHUB_DIR="$(cd "${PARENT_GUARD_DIR}/.." && pwd)"

# shellcheck source=scripts/versioning/file_versioning/github/lib/va.sh
source "${ROOT_GITHUB_DIR}/lib/va.sh"

if [[ "${VA_PARENT_ISSUE_GUARD_WRAPPER_ACTIVE:-0}" != "1" && "${VA_PARENT_ISSUE_GUARD_FORCE_LEGACY:-0}" != "1" ]]; then
  if command -v va_exec >/dev/null 2>&1; then
    set +e
    VA_PARENT_ISSUE_GUARD_WRAPPER_ACTIVE=1 va_exec issue parent-guard "$@"
    va_status=$?
    set -e
    if [[ "$va_status" -ne 127 ]]; then
      exit "$va_status"
    fi
  fi
fi

source "${ROOT_GITHUB_DIR}/lib/gh_cli.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh
source "${ROOT_GITHUB_DIR}/../../../common_lib/versioning/file_versioning/github/issue_helpers.sh"
source "${PARENT_GUARD_DIR}/cli.sh"
source "${PARENT_GUARD_DIR}/runtime.sh"
source "${PARENT_GUARD_DIR}/parent_lookup.sh"
source "${PARENT_GUARD_DIR}/evaluator.sh"
source "${PARENT_GUARD_DIR}/main.sh"

parent_issue_guard_run "$@"
