#!/usr/bin/env bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# Compatibility shim. Canonical implementation lives in scripts/common_lib/automation.
# shellcheck source=scripts/common_lib/automation/change_policy.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/automation/change_policy.sh"
