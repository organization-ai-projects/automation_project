#!/bin/bash

# Functions related to Git branches

LAST_DELETED_BRANCH_FILE="/tmp/last_deleted_branch"

# Check if branch exists locally
branch_exists_local() {
  local branch="$1"
  git show-ref --verify --quiet "refs/heads/$branch"
}

# Check if branch exists on remote
branch_exists_remote() {
  local remote="$1"
  local branch="$2"
  git ls-remote --exit-code --heads "$remote" "$branch" >/dev/null 2>&1
}

# Check if branch is protected
is_protected_branch() {
  local branch="$1"
  [[ "$branch" == "main" || "$branch" == "dev" ]]
}

# Get current branch name
get_current_branch() {
  git branch --show-current || die "Not on a branch (detached HEAD)."
}

# Ensure branch is not protected
require_non_protected_branch() {
  local branch="$1"
  if is_protected_branch "$branch"; then
    die "Cannot operate on protected branch: $branch"
  fi
}

# Save last deleted branch name for recreation
save_last_deleted_branch() {
  local branch="$1"
  echo "$branch" > "$LAST_DELETED_BRANCH_FILE"
}

# Get last deleted branch name
get_last_deleted_branch() {
  if [[ ! -f "$LAST_DELETED_BRANCH_FILE" ]]; then
    return 1
  fi

  local branch
  branch="$(cat "$LAST_DELETED_BRANCH_FILE" | head -n 1 | xargs)"

  if [[ -z "$branch" ]]; then
    return 1
  fi

  echo "$branch"
}