#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091,SC2034,SC2154

# Runtime module entrypoint.

PR_RUNTIME_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"

source "${PR_RUNTIME_DIR}/logging.sh"
source "${PR_RUNTIME_DIR}/git.sh"
source "${PR_RUNTIME_DIR}/state.sh"
