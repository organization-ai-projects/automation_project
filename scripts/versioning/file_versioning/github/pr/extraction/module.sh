#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091,SC2034,SC2154

# Extraction module entrypoint.

PR_EXTRACTION_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"

source "${PR_EXTRACTION_DIR}/from_compare.sh"
source "${PR_EXTRACTION_DIR}/from_pr_api.sh"
