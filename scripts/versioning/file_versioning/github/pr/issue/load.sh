#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091,SC2034,SC2154

# Issue module entrypoint.

PR_ISSUE_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"

source "${PR_ISSUE_DIR}/collector.sh"
source "${PR_ISSUE_DIR}/decision.sh"
source "${PR_ISSUE_DIR}/actions.sh"
