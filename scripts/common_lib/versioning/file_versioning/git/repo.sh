#!/bin/bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# Functions related to Git repositories

# Verify if inside a Git repository
require_git_repo() {
  git rev-parse --is-inside-work-tree >/dev/null 2>&1 || die "Not a git repository."
}
