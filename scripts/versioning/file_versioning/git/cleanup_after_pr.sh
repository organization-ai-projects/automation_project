#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

# shellcheck source=scripts/common_lib/versioning/file_versioning/git/commands.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/commands.sh"

REMOTE="${REMOTE:-origin}"
BASE_BRANCH="${BASE_BRANCH:-dev}"
PROTECTED_BRANCHES=("dev" "main")
DELETE_ONLY=false

# Parse arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    --delete-only)
      DELETE_ONLY=true
      shift
      ;;
    --help|-h)
      cat << 'EOF'
Usage: cleanup_after_pr.sh [OPTIONS]

Detects and manages local branches that are behind the base branch.

Options:
  --delete-only    Delete outdated branches without recreating them
  --help, -h       Show this help message

Environment variables:
  REMOTE           Git remote name (default: origin)
  BASE_BRANCH      Base branch to compare against (default: dev)

Examples:
  # Recreate outdated branches from base branch (default)
  ./cleanup_after_pr.sh

  # Only delete outdated branches
  ./cleanup_after_pr.sh --delete-only

For complete workflow documentation, see:
  scripts/versioning/file_versioning/git/sync_after_pr.md - Manual vs automated cleanup guide
EOF
      exit 0
      ;;
    *)
      echo "Unknown option: $1"
      echo "Use --help for usage information"
      exit 1
      ;;
  esac
done

echo "=== Updating branch $BASE_BRANCH ==="
CURRENT_BRANCH="$(vcs_local_branch --show-current || true)"

vcs_local_checkout "$BASE_BRANCH"
vcs_local_pull "$REMOTE" "$BASE_BRANCH"

echo "✓ Branch $BASE_BRANCH updated."
echo ""
echo "=== Detecting local branches behind $BASE_BRANCH ==="

vcs_local_fetch "$REMOTE" --prune

OUTDATED_BRANCHES=()

# Ignore protected branches
# Check if BASE_BRANCH has commits that the branch lacks (branch behind)
for branch in $(vcs_local_for_each_ref --format='%(refname:short)' refs/heads); do
  for p in "${PROTECTED_BRANCHES[@]}"; do
    [[ "$branch" == "$p" ]] && continue 2
  done

  BEHIND_COUNT=$(vcs_local_rev_list --count "$branch..$BASE_BRANCH" 2>/dev/null || echo "0")

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
echo "=== Deleting outdated branches ==="
for branch in "${OUTDATED_BRANCHES[@]}"; do
  echo "Processing: $branch"

  # Delete the local branch
  if vcs_local_branch -d "$branch" 2>/dev/null; then
    echo "  ✓ Local branch deleted."
  elif vcs_local_branch -D "$branch" 2>/dev/null; then
    echo "  ⚠ Local branch deleted (force)."
  else
    echo "  ℹ Local branch does not exist."
  fi

  # Delete the remote branch if it exists
  if vcs_local_ls_remote --exit-code --heads "$REMOTE" "$branch" >/dev/null 2>&1; then
    if vcs_local_push "$REMOTE" --delete "$branch" >/dev/null 2>&1; then
      echo "  ✓ Remote branch deleted."
    else
      echo "  ℹ Remote branch not deleted (permissions/protection?)."
    fi
  else
    echo "  ℹ Remote branch does not exist."
  fi

  # Recreate the branch from BASE_BRANCH if not in delete-only mode
  if [[ "$DELETE_ONLY" == "false" ]]; then
    vcs_local_checkout -b "$branch" "$BASE_BRANCH"
    vcs_local_push --set-upstream "$REMOTE" "$branch"
    echo "  ✓ Branch recreated."
  fi
done

if [[ -n "$CURRENT_BRANCH" ]] && vcs_local_show_ref --verify --quiet "refs/heads/$CURRENT_BRANCH"; then
  vcs_local_checkout "$CURRENT_BRANCH"
  echo "✓ Returned to $CURRENT_BRANCH"
else
  echo "✓ Remained on $BASE_BRANCH"
fi

echo ""
echo "=== Cleanup complete ==="
