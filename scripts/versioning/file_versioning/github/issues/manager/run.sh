#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

ISSUES_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"

# shellcheck source=scripts/versioning/file_versioning/github/issues/required_fields/load.sh
source "${ISSUES_DIR}/required_fields/load.sh"
source "${BASH_SOURCE[0]%/*}/cli.sh"
source "${BASH_SOURCE[0]%/*}/create.sh"
source "${BASH_SOURCE[0]%/*}/read.sh"
source "${BASH_SOURCE[0]%/*}/update.sh"
source "${BASH_SOURCE[0]%/*}/state.sh"

manager_issues_main "$@"
