#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091,SC2034,SC2154

# Compare module entrypoint.

PR_COMPARE_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"

source "${PR_COMPARE_DIR}/loaders.sh"
