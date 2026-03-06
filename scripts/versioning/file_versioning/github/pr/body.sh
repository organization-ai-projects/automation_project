#!/usr/bin/env bash

# PR body module entrypoint.

PR_BODY_LIB_DIR="$(cd "${BASH_SOURCE[0]%/*}" && pwd)"

source "${PR_BODY_LIB_DIR}/body_confirm.sh"
source "${PR_BODY_LIB_DIR}/body_merge.sh"
source "${PR_BODY_LIB_DIR}/body_builder.sh"
source "${PR_BODY_LIB_DIR}/body_publish.sh"
