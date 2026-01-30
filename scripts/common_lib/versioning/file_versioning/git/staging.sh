#!/bin/bash

# Functions related to staging (git add, git reset, git status)
# This orchestrates both working tree and index

# Add all changes to staging area (including untracked, deleted, etc.)
git_add_all() {
  git add -A
}

# Add specific files/paths
git_add_files() {
  local files=("$@")
  if [[ ${#files[@]} -eq 0 ]]; then
    die "No files specified to add."
  fi
  git add "${files[@]}"
}

# Unstage all changes
git_reset_all() {
  git reset HEAD
}

# Unstage specific files
git_reset_files() {
  local files=("$@")
  if [[ ${#files[@]} -eq 0 ]]; then
    die "No files specified to unstage."
  fi
  git reset HEAD "${files[@]}"
}

# Show git status (orchestrates working tree + index view)
git_status() {
  git status "$@"
}

# Get short status
git_status_short() {
  git status --short "$@"
}