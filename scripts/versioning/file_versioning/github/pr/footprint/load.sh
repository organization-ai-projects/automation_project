#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091,SC2034,SC2154

# Footprint module entrypoint.

PR_FOOTPRINT_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"

source "${PR_FOOTPRINT_DIR}/render.sh"
