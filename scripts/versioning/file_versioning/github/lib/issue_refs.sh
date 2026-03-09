#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

# Shared issue-reference parsing module entrypoint.

ISSUE_REFS_DIR="$(cd "${BASH_SOURCE[0]%/*}/issue_refs" && pwd)"

source "${ISSUE_REFS_DIR}/core.sh"
