#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

PARENT_GUARD_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
ROOT_GITHUB_DIR="$(cd "${PARENT_GUARD_DIR}/.." && pwd)"

source "${ROOT_GITHUB_DIR}/lib/gh_cli.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh"
source "${PARENT_GUARD_DIR}/cli.sh"
source "${PARENT_GUARD_DIR}/runtime.sh"
source "${PARENT_GUARD_DIR}/parent_lookup.sh"
source "${PARENT_GUARD_DIR}/evaluator.sh"
source "${PARENT_GUARD_DIR}/main.sh"

parent_issue_guard_run "$@"
