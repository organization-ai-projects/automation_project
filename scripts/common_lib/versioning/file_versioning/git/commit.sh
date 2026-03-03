#!/bin/bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# shellcheck source=scripts/common_lib/versioning/file_versioning/git/commands.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/commands.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/working_tree.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/working_tree.sh"

# Functions related to Git commits

require_commit_message() {
  local message="${1:-}"
  if [[ -z "$message" ]]; then
    die "Commit message is required."
  fi
}

git_commit_run() {
  vcs_local_commit "$@"
}

# Prepared by git_add_message and consumed by commit functions.
GIT_COMMIT_MESSAGE_ARGS=()

git_add_message() {
  local message="${1:-}"
  require_commit_message "$message"
  GIT_COMMIT_MESSAGE_ARGS=(-m "$message")
}

# Create a commit with a message
git_commit() {
  local message="$1"
  git_add_message "$message"
  git_commit_run "${GIT_COMMIT_MESSAGE_ARGS[@]}"
}

# Amend the last commit (base primitive).
git_commit_amend() {
  git_commit_run --amend "$@"
}

# Amend the last commit (keeping the same message).
git_commit_amend_no_edit() {
  git_commit_amend --no-edit
}

# Amend the last commit with a new message.
git_commit_amend_message() {
  local message="$1"
  git_add_message "$message"
  git_commit_amend "${GIT_COMMIT_MESSAGE_ARGS[@]}"
}
