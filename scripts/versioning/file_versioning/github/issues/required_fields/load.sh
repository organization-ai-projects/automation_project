#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

# Shared issue required-fields module entrypoint.

set -u

ISSUE_REQUIRED_FIELDS_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
ISSUES_DIR="$(cd "${ISSUE_REQUIRED_FIELDS_DIR}/.." && pwd)"

# shellcheck source=scripts/versioning/file_versioning/github/issues/common/gh.sh
source "${ISSUES_DIR}/common/gh.sh"

source "${ISSUE_REQUIRED_FIELDS_DIR}/contract.sh"
source "${ISSUE_REQUIRED_FIELDS_DIR}/parse.sh"
source "${ISSUE_REQUIRED_FIELDS_DIR}/validate.sh"
source "${ISSUE_REQUIRED_FIELDS_DIR}/compliance.sh"
source "${ISSUE_REQUIRED_FIELDS_DIR}/fetch.sh"
