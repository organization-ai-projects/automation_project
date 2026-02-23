#!/usr/bin/env bash
set -euo pipefail

# Create a clean work branch from dev with naming conventions
# Usage: ./create_work_branch.sh <type> <description>
# Types: feature, feat, fixture, fix, chore, refactor, doc, docs, test, tests

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/working_tree.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/working_tree.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/branch.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/branch.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/synch.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/synch.sh"

if [[ "$#" -lt 2 ]]; then
  echo "Usage: $0 <type> <description>" >&2
  echo "Types: feature, feat, fixture, fix, chore, refactor, doc, docs, test, tests" >&2
  echo "Example: $0 feat add-user-authentication" >&2
  exit 1
fi

TYPE="$1"
DESCRIPTION="$2"
REMOTE="${REMOTE:-origin}"
BASE_BRANCH="${BASE_BRANCH:-dev}"

# Validate type
case "$TYPE" in
  feature|feat|fixture|fix|chore|refactor|doc|docs|test|tests)
    ;;
  *)
    die "Invalid type: $TYPE. Must be one of: feature, feat, fixture, fix, chore, refactor, doc, docs, test, tests"
    ;;
esac

# Sanitize description (lowercase, replace spaces/underscores with dashes)
DESCRIPTION=$(echo "$DESCRIPTION" | tr '[:upper:]' '[:lower:]' | tr ' _' '--' | sed 's/[^a-z0-9-]//g')

BRANCH_NAME="${TYPE}/${DESCRIPTION}"

info "Creating work branch: $BRANCH_NAME"

require_git_repo
require_clean_tree

# Fetch latest changes
git_fetch_prune "$REMOTE"

# Check if branch already exists
if branch_exists_local "$BRANCH_NAME"; then
  die "Branch '$BRANCH_NAME' already exists locally."
fi

if branch_exists_remote "$REMOTE" "$BRANCH_NAME"; then
  die "Branch '$BRANCH_NAME' already exists on remote."
fi

# Checkout base branch and pull
info "Updating $BASE_BRANCH from $REMOTE..."
if branch_exists_local "$BASE_BRANCH"; then
  git checkout "$BASE_BRANCH"
else
  git checkout -b "$BASE_BRANCH" "$REMOTE/$BASE_BRANCH"
fi

git pull "$REMOTE" "$BASE_BRANCH"

# Create new branch
info "Creating branch '$BRANCH_NAME' from '$BASE_BRANCH'..."
git checkout -b "$BRANCH_NAME" "$BASE_BRANCH"

# Set up upstream
info "Pushing and setting upstream..."
git push --set-upstream "$REMOTE" "$BRANCH_NAME"

info "✓ Work branch '$BRANCH_NAME' created and pushed to $REMOTE."
info "You are now on branch '$BRANCH_NAME'."
