#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

ISSUES_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
# shellcheck source=scripts/versioning/file_versioning/github/lib/issue_refs.sh
source "${ISSUES_DIR}/../lib/issue_refs.sh"
# shellcheck source=scripts/versioning/file_versioning/github/issues/required_fields/module.sh
source "${ISSUES_DIR}/required_fields/module.sh"
source "${BASH_SOURCE[0]%/*}/common.sh"
source "${BASH_SOURCE[0]%/*}/processing.sh"

neutralize_non_compliant_closure_refs_main() {
  neutralize_run "$@"
}
