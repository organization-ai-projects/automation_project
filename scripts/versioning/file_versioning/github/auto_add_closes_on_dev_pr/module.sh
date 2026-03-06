#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

AUTO_ADD_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
ROOT_GITHUB_DIR="$(cd "${AUTO_ADD_DIR}/.." && pwd)"

source "${ROOT_GITHUB_DIR}/lib/gh_cli.sh"
source "${ROOT_GITHUB_DIR}/lib/issue_refs.sh"
source "${AUTO_ADD_DIR}/cli.sh"
source "${AUTO_ADD_DIR}/helpers.sh"
source "${AUTO_ADD_DIR}/workflow.sh"
