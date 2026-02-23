#!/usr/bin/env bash
set -euo pipefail

# Check for potential merge conflicts in local branches

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/core/command.sh
source "$ROOT_DIR/scripts/common_lib/core/command.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/branch.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/branch.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/synch.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/synch.sh"

require_git_repo

cd "$ROOT_DIR"

REMOTE="${REMOTE:-origin}"
BASE_BRANCH="${BASE_BRANCH:-dev}"
CURRENT_BRANCH="$(get_current_branch)"

info "Checking for merge conflicts between '$CURRENT_BRANCH' and '$BASE_BRANCH'..."

# Fetch latest changes
git_fetch_prune "$REMOTE"

# Check if base branch exists remotely
if ! branch_exists_remote "$REMOTE" "$BASE_BRANCH"; then
  die "Base branch '$REMOTE/$BASE_BRANCH' does not exist."
fi

# Try a test merge without committing
info "Attempting test merge of $REMOTE/$BASE_BRANCH into $CURRENT_BRANCH..."

# Save current state
CURRENT_HEAD="$(git rev-parse HEAD)"

# Create a temporary branch for testing
TEST_BRANCH="__test_merge_$$"
git branch "$TEST_BRANCH" "$CURRENT_BRANCH"

# Switch to test branch and try merge
git checkout "$TEST_BRANCH" >/dev/null 2>&1

if git merge --no-commit --no-ff "$REMOTE/$BASE_BRANCH" >/dev/null 2>&1; then
  info "✓ No merge conflicts detected."
  RESULT=0
else
  warn "⚠ Merge conflicts detected!"
  info "Conflicting files:"
  git diff --name-only --diff-filter=U || true
  RESULT=1
fi

# Abort the test merge
git merge --abort 2>/dev/null || true

# Switch back and delete test branch
git checkout "$CURRENT_BRANCH" >/dev/null 2>&1
git branch -D "$TEST_BRANCH" >/dev/null 2>&1

exit $RESULT
