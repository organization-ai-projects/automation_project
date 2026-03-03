#!/bin/bash

# Functions related to Git branches

last_deleted_branch_file() {
  local git_dir
  git_dir="$(git rev-parse --git-dir 2>/dev/null || true)"

  if [[ -n "$git_dir" ]]; then
    echo "${git_dir}/last_deleted_branch"
    return
  fi

  # Fallback path when repository metadata is temporarily unavailable.
  echo "/tmp/last_deleted_branch"
}

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
  local state_file

  state_file="$(last_deleted_branch_file)"
  mkdir -p "$(dirname "$state_file")"
  echo "$branch" > "$state_file"
}

# Get last deleted branch name
get_last_deleted_branch() {
  local state_file
  state_file="$(last_deleted_branch_file)"

  if [[ ! -f "$state_file" ]]; then
    return 1
  fi

  local branch
  branch="$(cat "$state_file" | head -n 1 | xargs)"

  if [[ -z "$branch" ]]; then
    return 1
  fi

  echo "$branch"
}
