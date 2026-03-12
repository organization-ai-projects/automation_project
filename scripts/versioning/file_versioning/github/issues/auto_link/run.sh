#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

ISSUES_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
# shellcheck source=scripts/versioning/file_versioning/github/lib/va.sh
source "${ISSUES_DIR}/../lib/va.sh"

if [[ "${VA_AUTO_LINK_WRAPPER_ACTIVE:-0}" != "1" && "${VA_AUTO_LINK_FORCE_LEGACY:-0}" != "1" ]]; then
  set +e
  VA_AUTO_LINK_WRAPPER_ACTIVE=1 va_exec issue auto-link "$@"
  va_status=$?
  set -e
  if [[ "$va_status" -ne 127 ]]; then
    exit "$va_status"
  fi
fi

# shellcheck source=scripts/versioning/file_versioning/github/issues/required_fields/load.sh
source "${ISSUES_DIR}/required_fields/load.sh"
# shellcheck source=scripts/versioning/file_versioning/github/issues/common/gh.sh
source "${ISSUES_DIR}/common/gh.sh"
# shellcheck source=scripts/versioning/file_versioning/github/issues/common/cli.sh
source "${ISSUES_DIR}/common/cli.sh"
# shellcheck source=scripts/versioning/file_versioning/github/issues/common/refs.sh
source "${ISSUES_DIR}/common/refs.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh
source "${ISSUES_DIR}/../../../../common_lib/versioning/file_versioning/github/issue_helpers.sh"
source "${BASH_SOURCE[0]%/*}/cli_graphql.sh"
source "${BASH_SOURCE[0]%/*}/state.sh"
source "${BASH_SOURCE[0]%/*}/validation.sh"
source "${BASH_SOURCE[0]%/*}/parent_link.sh"
source "${BASH_SOURCE[0]%/*}/parent_none.sh"
source "${BASH_SOURCE[0]%/*}/workflow.sh"

auto_link_run "$@"
