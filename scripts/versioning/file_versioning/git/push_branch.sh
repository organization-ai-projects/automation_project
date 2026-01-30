#!/usr/bin/env bash
set -euo pipefail

# Usage: ./push_branch.sh
# Description: Push the current branch to the remote, dev/main denied.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"

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

BRANCH_NAME="$(get_current_branch)"
require_non_protected_branch "$BRANCH_NAME"

git_fetch_prune "$REMOTE"

info "Pushing branch: $BRANCH_NAME -> $REMOTE"

# If upstream exists, simple push, otherwise push -u
if git rev-parse --abbrev-ref --symbolic-full-name "@{u}" >/dev/null 2>&1; then
  git push "$REMOTE" "$BRANCH_NAME"
  info "✓ Branch '$BRANCH_NAME' pushed to '$REMOTE'."
else
  git push --set-upstream "$REMOTE" "$BRANCH_NAME"
  info "✓ Branch '$BRANCH_NAME' pushed to '$REMOTE' (upstream configured)."
fi
