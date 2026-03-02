#!/bin/bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# shellcheck source=scripts/common_lib/versioning/file_versioning/git/commands.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/commands.sh"

# Functions related to the Git working tree

# Check if working tree is clean (no staged or unstaged changes)
require_clean_tree() {
  if ! vcs_local_diff --quiet || ! vcs_local_diff --cached --quiet; then
    die "Working tree is dirty. Commit/stash your changes first."
  fi
}

# Check if there are untracked files
has_untracked_files() {
  [[ -n "$(vcs_local_ls_files --others --exclude-standard)" ]]
}

# Check if working tree has any modifications (staged, unstaged, or untracked)
is_working_tree_dirty() {
  ! vcs_local_diff --quiet || ! vcs_local_diff --cached --quiet || has_untracked_files
}
