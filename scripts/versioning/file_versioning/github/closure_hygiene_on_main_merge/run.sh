#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_GITHUB_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
# shellcheck disable=SC1091
source "${ROOT_GITHUB_DIR}/lib/gh_cli.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/bootstrap.sh"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/parents.sh"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/milestones.sh"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/main.sh"

closure_hygiene_main "$@"
