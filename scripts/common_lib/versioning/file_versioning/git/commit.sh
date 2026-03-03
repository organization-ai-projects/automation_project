#!/bin/bash

# Functions related to Git commits

# Create a commit with a message
git_commit() {
  local message="$1"
  if [[ -z "$message" ]]; then
    die "Commit message is required."
  fi
  git commit -m "$message"
}

# Amend the last commit (keeping the same message)
git_commit_amend() {
  git commit --amend --no-edit
}

# Amend the last commit with a new message
git_commit_amend_message() {
  local message="$1"
  if [[ -z "$message" ]]; then
    die "Commit message is required."
  fi
  git commit --amend -m "$message"
}

# Check if there are staged changes
has_staged_changes() {
  ! git diff --cached --quiet
}

# Check if there are unstaged changes
has_unstaged_changes() {
  ! git diff --quiet
}
