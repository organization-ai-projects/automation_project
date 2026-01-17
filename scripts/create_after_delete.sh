#!/bin/bash
set -euo pipefail

# Usage: ./create_after_delete.sh
# Recreates the current branch from dev after local + remote deletion.

REMOTE="origin"
BASE_BRANCH="dev"

BRANCH_NAME="$(git branch --show-current || true)"

if [[ -z "$BRANCH_NAME" ]]; then
  echo "Error: No active local branch (detached HEAD). Switch to a branch and try again." >&2
  exit 1
fi

if [[ "$BRANCH_NAME" == "$BASE_BRANCH" || "$BRANCH_NAME" == "main" ]]; then
  echo "Error: Refusal to delete a protected branch ($BRANCH_NAME)." >&2
  exit 1
fi

echo "=== Recreate branch: $BRANCH_NAME (base: $BASE_BRANCH, remote: $REMOTE) ==="

# Always switch to the base before deleting the current branch
git fetch "$REMOTE" --prune

echo "-> Checkout $BASE_BRANCH"
git checkout "$BASE_BRANCH"
git pull "$REMOTE" "$BASE_BRANCH"

echo "-> Delete local branch $BRANCH_NAME (safe)"
if git show-ref --verify --quiet "refs/heads/$BRANCH_NAME"; then
  if git branch -d "$BRANCH_NAME"; then
    echo "✓ Local branch \"$BRANCH_NAME\" deleted."
  else
    echo "Error: Local branch \"$BRANCH_NAME\" not merged, deletion refused." >&2
    echo "Tip: merge it or force delete with: git branch -D \"$BRANCH_NAME\"" >&2
    exit 1
  fi
else
  echo "ℹ Local branch \"$BRANCH_NAME\" does not exist."
fi

echo "-> Delete remote branch $BRANCH_NAME (if exists)"
if git ls-remote --exit-code --heads "$REMOTE" "$BRANCH_NAME" >/dev/null 2>&1; then
  git push "$REMOTE" --delete "$BRANCH_NAME"
  echo "✓ Remote branch \"$BRANCH_NAME\" deleted."
else
  echo "ℹ Remote branch \"$BRANCH_NAME\" does not exist."
fi

echo "-> Create branch from $BASE_BRANCH"
git checkout -b "$BRANCH_NAME" "$BASE_BRANCH"

echo "-> Push & set upstream"
git push --set-upstream "$REMOTE" "$BRANCH_NAME"

echo "✓ Branch \"$BRANCH_NAME\" recreated from \"$BASE_BRANCH\" and pushed to \"$REMOTE\"."
