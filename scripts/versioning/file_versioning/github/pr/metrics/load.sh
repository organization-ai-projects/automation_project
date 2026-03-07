#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091,SC2034,SC2154

# Metrics module entrypoint.

PR_METRICS_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"

source "${PR_METRICS_DIR}/breaking.sh"
source "${PR_METRICS_DIR}/ci.sh"
