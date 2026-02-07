#!/usr/bin/env bash
# Common Git operations for bot_ci_harness

# Check if a branch exists locally
git_branch_exists_local() {
  local branch="$1"
  git show-ref --verify --quiet "refs/heads/$branch"
}

# Check if a branch exists on remote
git_branch_exists_remote() {
  local remote="$1"
  local branch="$2"
  git ls-remote --exit-code --heads "$remote" "$branch" >/dev/null 2>&1
}

# Delete local branch if it exists
git_delete_branch_local() {
  local branch="$1"
  if git_branch_exists_local "$branch"; then
    git branch -D "$branch" || true
  fi
}

# Delete remote branch if it exists
git_delete_branch_remote() {
  local remote="$1"
  local branch="$2"
  if git_branch_exists_remote "$remote" "$branch"; then
    git push "$remote" --delete "$branch" || true
  fi
}

# Check if two branches are at the same commit
git_branches_identical() {
  local ref1="$1"
  local ref2="$2"
  local sha1 sha2
  
  sha1=$(git rev-parse "$ref1" 2>/dev/null) || return 1
  sha2=$(git rev-parse "$ref2" 2>/dev/null) || return 1
  
  [[ "$sha1" == "$sha2" ]]
}

# Check if ref1 is an ancestor of ref2 (i.e., ref2 contains all commits from ref1)
git_is_ancestor() {
  local ref1="$1"
  local ref2="$2"
  git merge-base --is-ancestor "$ref1" "$ref2" 2>/dev/null
}

# Get the commit SHA for a ref
git_get_sha() {
  local ref="$1"
  git rev-parse "$ref" 2>/dev/null
}

# Check if working directory is clean
git_is_clean() {
  git diff-index --quiet HEAD --
}

# Fetch from remote quietly
git_fetch_quiet() {
  local remote="${1:-origin}"
  git fetch "$remote" >/dev/null 2>&1
}

# Push to remote quietly
git_push_quiet() {
  local remote="$1"
  local branch="$2"
  git push "$remote" "$branch" >/dev/null 2>&1
}
