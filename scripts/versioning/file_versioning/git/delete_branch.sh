#!/usr/bin/env bash
set -euo pipefail

# Usage: ./delete_branch.sh <branch-name> [--force]
# Deletes a local branch (safe by default) and deletes remote branch if it exists.
# Stores the deleted branch name into /tmp/last_deleted_branch.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/core/string_utils.sh
source "$ROOT_DIR/scripts/common_lib/core/string_utils.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/branch.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/branch.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/synch.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/synch.sh"

require_git_repo

REMOTE="${REMOTE:-origin}"
BASE_BRANCH="${BASE_BRANCH:-dev}"

if [[ $# -lt 1 ]]; then
  die "Usage: $0 <branch-name> [--force]"
fi

BRANCH_NAME="$(string_trim "$1")"
FORCE="false"
if [[ "${2:-}" == "--force" ]]; then
  FORCE="true"
fi

if [[ -z "$BRANCH_NAME" ]]; then
  die "Empty branch name."
fi

# Protections
require_non_protected_branch "$BRANCH_NAME"

CURRENT_BRANCH="$(get_current_branch)"
if [[ "$CURRENT_BRANCH" == "$BRANCH_NAME" ]]; then
  warn "You are currently on '$BRANCH_NAME'. Checking out '$BASE_BRANCH' first."
  git checkout "$BASE_BRANCH"
fi

# Save deleted branch name for create_branch.sh
save_last_deleted_branch "$BRANCH_NAME"

git_fetch_prune "$REMOTE"

info "Deleting branch: $BRANCH_NAME (remote: $REMOTE)"

# Delete local
if branch_exists_local "$BRANCH_NAME"; then
  if [[ "$FORCE" == "true" ]]; then
    git branch -D "$BRANCH_NAME"
    warn "⚠ Local branch '$BRANCH_NAME' deleted (force)."
  else
    git branch -d "$BRANCH_NAME"
    info "✓ Local branch '$BRANCH_NAME' deleted."
  fi
else
  info "Local branch '$BRANCH_NAME' does not exist."
fi

# Delete remote (if exists)
if branch_exists_remote "$REMOTE" "$BRANCH_NAME"; then
  git push "$REMOTE" --delete "$BRANCH_NAME"
  info "✓ Remote branch '$BRANCH_NAME' deleted."
else
  info "Remote branch '$BRANCH_NAME' does not exist."
fi
