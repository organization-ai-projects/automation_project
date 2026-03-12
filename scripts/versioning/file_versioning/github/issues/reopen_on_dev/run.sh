#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

ISSUES_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
# shellcheck source=scripts/versioning/file_versioning/github/lib/va.sh
source "${ISSUES_DIR}/../lib/va.sh"

if [[ "${VA_REOPEN_ON_DEV_WRAPPER_ACTIVE:-0}" != "1" && "${VA_REOPEN_ON_DEV_FORCE_LEGACY:-0}" != "1" ]]; then
  set +e
  VA_REOPEN_ON_DEV_WRAPPER_ACTIVE=1 va_exec issue reopen-on-dev "$@"
  va_status=$?
  set -e
  if [[ "$va_status" -ne 127 ]]; then
    exit "$va_status"
  fi
fi

# shellcheck source=scripts/versioning/file_versioning/github/lib/issue_refs.sh
source "${ISSUES_DIR}/../lib/issue_refs.sh"
# shellcheck source=scripts/versioning/file_versioning/github/issues/common/gh.sh
source "${ISSUES_DIR}/common/gh.sh"
# shellcheck source=scripts/versioning/file_versioning/github/issues/common/cli.sh
source "${ISSUES_DIR}/common/cli.sh"
# shellcheck source=scripts/versioning/file_versioning/github/issues/common/refs.sh
source "${ISSUES_DIR}/common/refs.sh"
source "${BASH_SOURCE[0]%/*}/cli_refs.sh"
source "${BASH_SOURCE[0]%/*}/project_sync.sh"
source "${BASH_SOURCE[0]%/*}/actions.sh"

reopen_on_dev_run "$@"
