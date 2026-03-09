#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

ISSUES_DIR="$(cd "${BASH_SOURCE[0]%/*}/.." && pwd)"
source "${BASH_SOURCE[0]%/*}/dispatch.sh"

create_direct_issue_entry_run "$@"
