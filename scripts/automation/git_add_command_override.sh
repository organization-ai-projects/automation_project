#!/usr/bin/env bash
set -euo pipefail

# Git add command override.
# Source this file in your shell to route `git add ...` through
# scripts/automation/git_add_guard.sh while leaving other git commands untouched.
#
# Usage:
#   source /path/to/repo/scripts/automation/git_add_command_override.sh

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  echo "This file must be sourced, not executed." >&2
  echo "Example: source scripts/automation/git_add_command_override.sh" >&2
  exit 1
fi

_git_add_override_root() {
  # Resolve repository root from this file location.
  local script_dir
  script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  cd "$script_dir/../.." && pwd
}

git() {
  if [[ "${1:-}" == "add" ]]; then
    shift
    "$(_git_add_override_root)/scripts/automation/git_add_guard.sh" "$@"
  else
    command git "$@"
  fi
}

echo "git add override enabled: git add now uses scripts/automation/git_add_guard.sh"
