#!/usr/bin/env bash
set -euo pipefail

# Clean closure of a work branch (local + remote deletion)
# Usage: ./finish_branch.sh [branch-name]
# If no branch name provided, uses current branch

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/branch.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/branch.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/synch.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/synch.sh"

require_git_repo

REMOTE="${REMOTE:-origin}"
BASE_BRANCH="${BASE_BRANCH:-dev}"

# Determine branch to finish
if [[ "$#" -ge 1 ]]; then
  BRANCH_TO_FINISH="$1"
else
  BRANCH_TO_FINISH="$(get_current_branch)"
fi

info "Finishing branch: $BRANCH_TO_FINISH"

# Safety: cannot finish protected branches
require_non_protected_branch "$BRANCH_TO_FINISH"

# Fetch latest changes
git_fetch_prune "$REMOTE"

# Switch to base branch if currently on the branch to finish
CURRENT_BRANCH="$(git branch --show-current || true)"
if [[ "$CURRENT_BRANCH" == "$BRANCH_TO_FINISH" ]]; then
  info "Switching to $BASE_BRANCH..."
  if branch_exists_local "$BASE_BRANCH"; then
    git checkout "$BASE_BRANCH"
  else
    git checkout -b "$BASE_BRANCH" "$REMOTE/$BASE_BRANCH"
  fi
  git pull "$REMOTE" "$BASE_BRANCH"
fi

# Delete local branch
if branch_exists_local "$BRANCH_TO_FINISH"; then
  info "Deleting local branch '$BRANCH_TO_FINISH'..."
  if git branch -d "$BRANCH_TO_FINISH" 2>/dev/null; then
    info "✓ Local branch deleted (safe)."
  elif git branch -D "$BRANCH_TO_FINISH" 2>/dev/null; then
    warn "⚠ Local branch deleted (forced)."
  else
    die "Failed to delete local branch '$BRANCH_TO_FINISH'."
  fi
else
  info "Local branch '$BRANCH_TO_FINISH' does not exist."
fi

# Delete remote branch
if branch_exists_remote "$REMOTE" "$BRANCH_TO_FINISH"; then
  info "Deleting remote branch '$REMOTE/$BRANCH_TO_FINISH'..."
  if git push "$REMOTE" --delete "$BRANCH_TO_FINISH"; then
    info "✓ Remote branch deleted."
  else
    warn "⚠ Failed to delete remote branch (may be protected or lack permissions)."
  fi
else
  info "Remote branch '$REMOTE/$BRANCH_TO_FINISH' does not exist."
fi

# Run fetch with prune to clean up tracking refs
git_fetch_prune "$REMOTE"

info "✅ Branch '$BRANCH_TO_FINISH' finished successfully."
