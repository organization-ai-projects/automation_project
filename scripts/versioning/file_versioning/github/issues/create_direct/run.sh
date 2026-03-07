#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

ISSUES_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
# shellcheck source=scripts/versioning/file_versioning/github/issues/required_fields/load.sh
source "${ISSUES_DIR}/required_fields/load.sh"
source "${BASH_SOURCE[0]%/*}/cli_contract.sh"
source "${BASH_SOURCE[0]%/*}/builder.sh"

run_create_direct_issue "$@"
