#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

set -euo pipefail

AUTO_ADD_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"
source "${AUTO_ADD_DIR}/dispatch.sh"

auto_add_closes_entry_run "$@"
