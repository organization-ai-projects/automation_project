#!/bin/bash
set -euo pipefail

# Usage: ./push_branch.sh
# Description: Push the current branch to the remote, dev/main denied.

REMOTE="${REMOTE:-origin}"
PROTECTED_BRANCHES=("dev" "main")

BRANCH_NAME="$(git branch --show-current || true)"

if [[ -z "$BRANCH_NAME" ]]; then
  echo "Error: No active local branch (detached HEAD). Switch to a branch and retry." >&2
  exit 1
fi

for b in "${PROTECTED_BRANCHES[@]}"; do
  if [[ "$BRANCH_NAME" == "$b" ]]; then
    echo "Error: Direct push to '$b' is forbidden." >&2
    exit 1
  fi
done

git fetch "$REMOTE" --prune

echo "=== Push branch: $BRANCH_NAME -> $REMOTE ==="

# If upstream exists, simple push, otherwise push -u
if git rev-parse --abbrev-ref --symbolic-full-name "@{u}" >/dev/null 2>&1; then
  git push "$REMOTE" "$BRANCH_NAME"
  echo "✓ Branch '$BRANCH_NAME' pushed to '$REMOTE'."
else
  git push --set-upstream "$REMOTE" "$BRANCH_NAME"
  echo "✓ Branch '$BRANCH_NAME' pushed to '$REMOTE' (upstream configured)."
fi
