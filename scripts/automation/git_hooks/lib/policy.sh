#!/usr/bin/env bash

# Compatibility shim. Canonical implementation lives in scripts/common_lib/automation.
# shellcheck source=scripts/common_lib/automation/change_policy.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/automation/change_policy.sh"
