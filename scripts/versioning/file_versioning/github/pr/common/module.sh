#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091,SC2034,SC2154

# Shared PR-domain helpers.

PR_COMMON_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"

source "${PR_COMMON_DIR}/crate_path.sh"
source "${PR_COMMON_DIR}/gh_repo.sh"
source "${PR_COMMON_DIR}/markdown_sections.sh"
