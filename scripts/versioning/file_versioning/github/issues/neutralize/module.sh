#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

ISSUES_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
# shellcheck source=scripts/versioning/file_versioning/github/lib/issue_refs.sh
source "${ISSUES_DIR}/../lib/issue_refs.sh"
# shellcheck source=scripts/versioning/file_versioning/github/issues/required_fields/module.sh
source "${ISSUES_DIR}/required_fields/module.sh"
source "${BASH_SOURCE[0]%/*}/cli.sh"
source "${BASH_SOURCE[0]%/*}/processing.sh"
