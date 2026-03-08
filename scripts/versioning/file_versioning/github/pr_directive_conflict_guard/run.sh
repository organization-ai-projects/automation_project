#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

PR_DIRECTIVE_CONFLICT_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
ROOT_GITHUB_DIR="$(cd "${PR_DIRECTIVE_CONFLICT_DIR}/.." && pwd)"

source "${ROOT_GITHUB_DIR}/lib/gh_cli.sh"
source "${ROOT_GITHUB_DIR}/lib/issue_refs.sh"
source "${PR_DIRECTIVE_CONFLICT_DIR}/cli.sh"
source "${PR_DIRECTIVE_CONFLICT_DIR}/helpers.sh"
source "${PR_DIRECTIVE_CONFLICT_DIR}/github.sh"
source "${PR_DIRECTIVE_CONFLICT_DIR}/workflow.sh"

pr_directive_conflict_guard_run "$@"
