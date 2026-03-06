#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091,SC2034,SC2154

# Pipeline module entrypoint.

PR_PIPELINE_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"

source "${PR_PIPELINE_DIR}/init.sh"
source "${PR_PIPELINE_DIR}/collect.sh"
source "${PR_PIPELINE_DIR}/render.sh"
