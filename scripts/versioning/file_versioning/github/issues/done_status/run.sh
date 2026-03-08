#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

ISSUES_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
# shellcheck source=scripts/versioning/file_versioning/github/lib/issue_refs.sh
source "${ISSUES_DIR}/../lib/issue_refs.sh"
# shellcheck source=scripts/versioning/file_versioning/github/issues/common/gh.sh
source "${ISSUES_DIR}/common/gh.sh"
# shellcheck source=scripts/versioning/file_versioning/github/issues/common/cli.sh
source "${ISSUES_DIR}/common/cli.sh"
# shellcheck source=scripts/versioning/file_versioning/github/issues/common/refs.sh
source "${ISSUES_DIR}/common/refs.sh"
source "${BASH_SOURCE[0]%/*}/cli_refs.sh"
source "${BASH_SOURCE[0]%/*}/actions.sh"

done_status_run "$@"
