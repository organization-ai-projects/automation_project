#!/usr/bin/env bash
# Shared scope resolver for prepare-commit-msg.

# shellcheck source=scripts/automation/git_hooks/lib/scope_resolver.sh
source "$(git rev-parse --show-toplevel)/scripts/automation/git_hooks/lib/scope_resolver.sh"
