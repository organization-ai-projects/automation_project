#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091,SC2034,SC2154


# PR body module entrypoint.

PR_BODY_LIB_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"

source "${PR_BODY_LIB_DIR}/confirm.sh"
source "${PR_BODY_LIB_DIR}/merge.sh"
source "${PR_BODY_LIB_DIR}/validation_gate.sh"
source "${PR_BODY_LIB_DIR}/builder.sh"
source "${PR_BODY_LIB_DIR}/publish.sh"
