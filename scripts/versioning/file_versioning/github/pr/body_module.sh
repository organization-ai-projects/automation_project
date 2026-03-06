#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091,SC2034,SC2154


# PR body module entrypoint.

PR_BODY_LIB_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"

source "${PR_BODY_LIB_DIR}/body_confirm.sh"
source "${PR_BODY_LIB_DIR}/body_merge.sh"
source "${PR_BODY_LIB_DIR}/body.sh"
source "${PR_BODY_LIB_DIR}/body_publish.sh"
