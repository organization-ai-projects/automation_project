#!/bin/bash
set -euo pipefail

# Usage: ./delete_branch.sh <branch-name> [--force]
# Deletes a local branch (safe by default) and deletes remote branch if it exists.
# Stores the deleted branch name into /tmp/last_deleted_branch.

REMOTE="origin"
BASE_BRANCH="dev"
LAST_DELETED_FILE="/tmp/last_deleted_branch"

if [[ $# -lt 1 ]]; then
  echo "Usage: $0 <branch-name> [--force]" >&2
  exit 1
fi

BRANCH_NAME="$1"
FORCE="false"
if [[ "${2:-}" == "--force" ]]; then
  FORCE="true"
fi

BRANCH_NAME="$(printf '%s' "$BRANCH_NAME" | xargs || true)"
if [[ -z "$BRANCH_NAME" ]]; then
  echo "Error: empty branch name." >&2
  exit 1
fi

# Protections
if [[ "$BRANCH_NAME" == "$BASE_BRANCH" || "$BRANCH_NAME" == "main" ]]; then
  echo "Error: refusing to delete a protected branch ($BRANCH_NAME)." >&2
  exit 1
fi

CURRENT_BRANCH="$(git branch --show-current || true)"
if [[ "$CURRENT_BRANCH" == "$BRANCH_NAME" ]]; then
  echo "Refusal: you are currently on '$BRANCH_NAME'. Checkout '$BASE_BRANCH' first." >&2
  git checkout "$BASE_BRANCH"
fi

# Save deleted branch name for create_branch.sh
echo "$BRANCH_NAME" > "$LAST_DELETED_FILE"

git fetch "$REMOTE" --prune

echo "=== Delete branch: $BRANCH_NAME (remote: $REMOTE) ==="

# Delete local
if git show-ref --verify --quiet "refs/heads/$BRANCH_NAME"; then
  if [[ "$FORCE" == "true" ]]; then
    git branch -D "$BRANCH_NAME"
    echo "⚠ Local branch '$BRANCH_NAME' deleted (force)."
  else
    git branch -d "$BRANCH_NAME"
    echo "✓ Local branch '$BRANCH_NAME' deleted."
  fi
else
  echo "ℹ Local branch '$BRANCH_NAME' does not exist."
fi

# Delete remote (if exists)
if git ls-remote --exit-code --heads "$REMOTE" "$BRANCH_NAME" >/dev/null 2>&1; then
  git push "$REMOTE" --delete "$BRANCH_NAME"
  echo "✓ Remote branch '$BRANCH_NAME' deleted."
else
  echo "ℹ Remote branch '$BRANCH_NAME' does not exist."
fi
