#!/bin/bash
set -euo pipefail

REMOTE="${REMOTE:-origin}"
BASE_BRANCH="${BASE_BRANCH:-dev}"
PROTECTED_BRANCHES=("dev" "main")

echo "=== Updating branch $BASE_BRANCH ==="
CURRENT_BRANCH="$(git branch --show-current || true)"

git checkout "$BASE_BRANCH"
git pull "$REMOTE" "$BASE_BRANCH"

echo "✓ Branch $BASE_BRANCH updated."
echo ""
echo "=== Detecting local branches behind $BASE_BRANCH ==="

git fetch "$REMOTE" --prune

OUTDATED_BRANCHES=()

# Ignore protected branches
# Check if BASE_BRANCH has commits that the branch lacks (branch behind)
for branch in $(git for-each-ref --format='%(refname:short)' refs/heads); do
  for p in "${PROTECTED_BRANCHES[@]}"; do
    [[ "$branch" == "$p" ]] && continue 2
  done

  BEHIND_COUNT=$(git rev-list --count "$branch..$BASE_BRANCH" 2>/dev/null || echo "0")

  if (( BEHIND_COUNT > 0 )); then
    echo "  → $branch is behind by $BEHIND_COUNT commit(s) on $BASE_BRANCH"
    OUTDATED_BRANCHES+=("$branch")
  fi
done

if (( ${#OUTDATED_BRANCHES[@]} == 0 )); then
  echo "No outdated local branches detected."
  exit 0
fi

echo "Target branches:"
printf ' - %s\n' "${OUTDATED_BRANCHES[@]}"

echo ""
echo "=== Deleting and recreating branches ==="
for branch in "${OUTDATED_BRANCHES[@]}"; do
  echo "Processing: $branch"

  # Delete the local branch
  if git branch -d "$branch" 2>/dev/null; then
    echo "  ✓ Local branch deleted."
  elif git branch -D "$branch" 2>/dev/null; then
    echo "  ⚠ Local branch deleted (force)."
  else
    echo "  ℹ Local branch does not exist."
  fi

  # Delete the remote branch if it exists
  if git ls-remote --exit-code --heads "$REMOTE" "$branch" >/dev/null 2>&1; then
    if git push "$REMOTE" --delete "$branch" >/dev/null 2>&1; then
      echo "  ✓ Remote branch deleted."
    else
      echo "  ℹ Remote branch not deleted (permissions/protection?)."
    fi
  else
    echo "  ℹ Remote branch does not exist."
  fi

  # Recreate the branch from BASE_BRANCH
  git checkout -b "$branch" "$BASE_BRANCH"
  git push --set-upstream "$REMOTE" "$branch"
  echo "  ✓ Branch recreated."
done

if [[ -n "$CURRENT_BRANCH" ]] && git show-ref --verify --quiet "refs/heads/$CURRENT_BRANCH"; then
  git checkout "$CURRENT_BRANCH"
  echo "✓ Returned to $CURRENT_BRANCH"
else
  echo "✓ Remained on $BASE_BRANCH"
fi

echo ""
echo "=== Cleanup complete ==="
