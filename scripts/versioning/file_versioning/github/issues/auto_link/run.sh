#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

ISSUES_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
# shellcheck source=scripts/versioning/file_versioning/github/issues/required_fields/load.sh
source "${ISSUES_DIR}/required_fields/load.sh"
# shellcheck source=scripts/versioning/file_versioning/github/issues/common/gh.sh
source "${ISSUES_DIR}/common/gh.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh"
source "${BASH_SOURCE[0]%/*}/cli_graphql.sh"
source "${BASH_SOURCE[0]%/*}/state.sh"
source "${BASH_SOURCE[0]%/*}/validation.sh"
source "${BASH_SOURCE[0]%/*}/parent_none.sh"
source "${BASH_SOURCE[0]%/*}/parent_link.sh"
source "${BASH_SOURCE[0]%/*}/workflow.sh"

auto_link_run "$@"
