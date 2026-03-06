#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091,SC2034,SC2154

# CLI module entrypoint.

PR_CLI_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"

source "${PR_CLI_DIR}/help.sh"
source "${PR_CLI_DIR}/defaults.sh"
source "${PR_CLI_DIR}/parse.sh"
source "${PR_CLI_DIR}/finalize.sh"
