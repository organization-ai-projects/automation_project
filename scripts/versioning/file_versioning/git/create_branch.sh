#!/bin/bash
set -euo pipefail

# Usage: ./create_branch.sh <branch-name>
# If branch-name is omitted, tries to reuse /tmp/last_deleted_branch.
# Creates branch from dev and optionally pushes it.

REMOTE="origin"
BASE_BRANCH="dev"
LAST_DELETED_FILE="/tmp/last_deleted_branch"

BRANCH_NAME="${1:-}"

if [[ -z "$BRANCH_NAME" ]]; then
  if [[ -f "$LAST_DELETED_FILE" ]]; then
    BRANCH_NAME="$(tr -d '\r' < "$LAST_DELETED_FILE" | head -n 1 | xargs || true)"
    if [[ -z "$BRANCH_NAME" ]]; then
      echo "Error: $LAST_DELETED_FILE is empty or invalid." >&2
      exit 1
    fi
    echo "No name provided. Recreating the last deleted branch: $BRANCH_NAME"
  else
    echo "Error: you must specify a branch name (or $LAST_DELETED_FILE does not exist)." >&2
    exit 1
  fi
fi

# Protections basiques
if [[ "$BRANCH_NAME" == "$BASE_BRANCH" || "$BRANCH_NAME" == "main" ]]; then
  echo "Error: protected/refused branch name: $BRANCH_NAME" >&2
  exit 1
fi

# Refuse spaces (optional but sane)
if [[ "$BRANCH_NAME" == *" "* ]]; then
  echo "Error: invalid branch name (contains spaces): '$BRANCH_NAME'" >&2
  exit 1
fi

echo "=== Create branch: $BRANCH_NAME (base: $BASE_BRANCH) ==="

git fetch "$REMOTE" --prune

git checkout "$BASE_BRANCH"
git pull "$REMOTE" "$BASE_BRANCH"

# If branch already exists locally, just checkout it (or refuse; your choice)
if git show-ref --verify --quiet "refs/heads/$BRANCH_NAME"; then
  echo "ℹ The local branch '$BRANCH_NAME' already exists. Checkout."
  git checkout "$BRANCH_NAME"
else
  git checkout -b "$BRANCH_NAME" "$BASE_BRANCH"
  echo "✓ Branch '$BRANCH_NAME' created from '$BASE_BRANCH'."
fi

# Optional: push + upstream (highly recommended)
git push --set-upstream "$REMOTE" "$BRANCH_NAME"
echo "✓ Branch '$BRANCH_NAME' pushed to '$REMOTE' with upstream."
