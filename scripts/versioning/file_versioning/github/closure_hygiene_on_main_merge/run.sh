#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_GITHUB_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
# shellcheck source=scripts/versioning/file_versioning/github/lib/va.sh
source "${ROOT_GITHUB_DIR}/lib/va.sh"

if [[ "${VA_CLOSURE_HYGIENE_WRAPPER_ACTIVE:-0}" != "1" && "${VA_CLOSURE_HYGIENE_FORCE_LEGACY:-0}" != "1" ]]; then
  if command -v va_exec >/dev/null 2>&1; then
    set +e
    VA_CLOSURE_HYGIENE_WRAPPER_ACTIVE=1 va_exec issue closure-hygiene "$@"
    va_status=$?
    set -e
    if [[ "$va_status" -ne 127 ]]; then
      exit "$va_status"
    fi
  fi
fi

# shellcheck disable=SC1091
source "${ROOT_GITHUB_DIR}/lib/gh_cli.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh
source "${ROOT_GITHUB_DIR}/../../../common_lib/versioning/file_versioning/github/issue_helpers.sh"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/bootstrap.sh"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/parents.sh"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/milestones.sh"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/main.sh"

closure_hygiene_main "$@"
