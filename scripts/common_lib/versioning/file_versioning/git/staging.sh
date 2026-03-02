#!/bin/bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# shellcheck source=scripts/common_lib/versioning/file_versioning/git/commands.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/commands.sh"

# Functions related to staging (git add, git reset, git status)
# This orchestrates both working tree and index

# Add all changes to staging area (including untracked, deleted, etc.)
git_add_all() {
  vcs_local_add -A
}

# Add specific files/paths
git_add_files() {
  local files=("$@")
  if [[ ${#files[@]} -eq 0 ]]; then
    die "No files specified to add."
  fi
  vcs_local_add "${files[@]}"
}

# Unstage all changes
git_reset_all() {
  vcs_local_reset HEAD
}

# Unstage specific files
git_reset_files() {
  local files=("$@")
  if [[ ${#files[@]} -eq 0 ]]; then
    die "No files specified to unstage."
  fi
  vcs_local_reset HEAD "${files[@]}"
}

# Show git status (orchestrates working tree + index view)
git_status() {
  vcs_local_status "$@"
}

# Get short status
git_status_short() {
  vcs_local_status --short "$@"
}
