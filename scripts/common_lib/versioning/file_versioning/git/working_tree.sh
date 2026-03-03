#!/bin/bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# shellcheck source=scripts/common_lib/versioning/file_versioning/git/commands.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/commands.sh"

# Functions related to the Git working tree

working_tree_has_unstaged_diff() {
  ! vcs_local_diff --quiet
}

working_tree_has_staged_diff() {
  ! vcs_local_diff --cached --quiet
}

# Backward-compatible aliases for existing commit helpers.
has_staged_changes() {
  working_tree_has_staged_diff
}

has_unstaged_changes() {
  working_tree_has_unstaged_diff
}

# Check if working tree is clean (no staged or unstaged changes)
require_clean_tree() {
  if working_tree_has_unstaged_diff || working_tree_has_staged_diff; then
    die "Working tree is dirty. Commit/stash your changes first."
  fi
}

# Check if there are untracked files
has_untracked_files() {
  [[ -n "$(vcs_local_ls_files --others --exclude-standard)" ]]
}

# Check if working tree has any modifications (staged, unstaged, or untracked)
is_working_tree_dirty() {
  working_tree_has_unstaged_diff || working_tree_has_staged_diff || has_untracked_files
}
