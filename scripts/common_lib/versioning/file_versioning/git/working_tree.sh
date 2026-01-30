#!/bin/bash

# Functions related to the Git working tree

# Check if working tree is clean (no staged or unstaged changes)
require_clean_tree() {
  if ! git diff --quiet || ! git diff --cached --quiet; then
    die "Working tree is dirty. Commit/stash your changes first."
  fi
}

# Check if there are untracked files
has_untracked_files() {
  [[ -n "$(git ls-files --others --exclude-standard)" ]]
}

# Check if working tree has any modifications (staged, unstaged, or untracked)
is_working_tree_dirty() {
  ! git diff --quiet || ! git diff --cached --quiet || has_untracked_files
}