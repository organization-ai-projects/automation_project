#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

ISSUES_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
# shellcheck source=scripts/versioning/file_versioning/github/lib/issue_refs.sh
source "${ISSUES_DIR}/../lib/issue_refs.sh"
source "${BASH_SOURCE[0]%/*}/common.sh"
source "${BASH_SOURCE[0]%/*}/project_sync.sh"
source "${BASH_SOURCE[0]%/*}/actions.sh"

issue_reopen_on_dev_merge_main() {
  reopen_on_dev_run "$@"
}
