#!/usr/bin/env bash
set -euo pipefail

# Usage: ./create_branch.sh <branch-name>
# If branch-name is omitted, tries to reuse /tmp/last_deleted_branch.
# Creates branch from dev and optionally pushes it.

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

BRANCH_NAME="${1:-}"

if [[ -z "$BRANCH_NAME" ]]; then
  if BRANCH_NAME="$(get_last_deleted_branch)"; then
    info "No name provided. Recreating the last deleted branch: $BRANCH_NAME"
  else
    die "You must specify a branch name (or no last deleted branch found)."
  fi
fi

# Protections
require_non_protected_branch "$BRANCH_NAME"

# Refuse spaces
if string_contains "$BRANCH_NAME" " "; then
  die "Invalid branch name (contains spaces): '$BRANCH_NAME'"
fi

# Validate branch naming convention
# Prefixes documented in CONTRIBUTING.md, with common plural variants
ALLOWED_PREFIXES=("feat/" "fix/" "doc/" "docs/" "refactor/" "test/" "tests/" "chore/")
has_valid_prefix=false
for prefix in "${ALLOWED_PREFIXES[@]}"; do
  if [[ "$BRANCH_NAME" == "$prefix"* ]]; then
    has_valid_prefix=true
    break
  fi
done

if [[ "$has_valid_prefix" == false ]]; then
  error "Invalid branch name: '$BRANCH_NAME'"
  error "Branch names must start with one of: ${ALLOWED_PREFIXES[*]}"
  error ""
  error "Examples:"
  error "  - feat/user-authentication"
  error "  - fix/json-parser-panic"
  error "  - doc/update-api-docs"
  error "  - refactor/simplify-error-handling"
  error "  - test/add-integration-tests"
  error "  - chore/update-dependencies"
  die ""
fi

info "Creating branch: $BRANCH_NAME (base: $BASE_BRANCH)"

git_fetch_prune "$REMOTE"

git checkout "$BASE_BRANCH"
git pull "$REMOTE" "$BASE_BRANCH"

# If branch already exists locally, just checkout it
if branch_exists_local "$BRANCH_NAME"; then
  info "The local branch '$BRANCH_NAME' already exists. Checking out."
  git checkout "$BRANCH_NAME"
else
  git checkout -b "$BRANCH_NAME" "$BASE_BRANCH"
  info "✓ Branch '$BRANCH_NAME' created from '$BASE_BRANCH'."
fi

# Push with upstream
git push --set-upstream "$REMOTE" "$BRANCH_NAME"
info "✓ Branch '$BRANCH_NAME' pushed to '$REMOTE' with upstream."
