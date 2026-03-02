#!/bin/bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# shellcheck source=scripts/common_lib/versioning/file_versioning/git/commands.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/commands.sh"

# Functions related to Git commits

require_commit_message() {
  local message="${1:-}"
  if [[ -z "$message" ]]; then
    die "Commit message is required."
  fi
}

commit_run() {
  vcs_local_commit "$@"
}

git_has_diff() {
  vcs_local_diff "$@" --quiet
}

# Create a commit with a message
git_commit() {
  local message="$1"
  require_commit_message "$message"
  commit_run -m "$message"
}

# Amend the last commit (keeping the same message)
git_commit_amend() {
  commit_run --amend --no-edit
}

# Amend the last commit with a new message
git_commit_amend_message() {
  local message="$1"
  require_commit_message "$message"
  commit_run --amend -m "$message"
}

# Check if there are staged changes
has_staged_changes() {
  ! git_has_diff --cached
}

# Check if there are unstaged changes
has_unstaged_changes() {
  ! git_has_diff
}
