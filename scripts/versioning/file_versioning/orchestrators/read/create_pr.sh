#!/usr/bin/env bash
set -euo pipefail

# Automate the creation of a pull request with smart defaults
# Usage: ./create_pr.sh [--base <branch>] [--title <title>] [--body <body>] [--draft]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/core/command.sh
source "$ROOT_DIR/scripts/common_lib/core/command.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/branch.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/branch.sh"

require_git_repo
require_cmd gh

CURRENT_BRANCH="$(get_current_branch)"
BASE_BRANCH="${BASE_BRANCH:-dev}"
TITLE=""
BODY=""
DRAFT=false

# Parse arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    --base)
      BASE_BRANCH="$2"
      shift 2
      ;;
    --title)
      TITLE="$2"
      shift 2
      ;;
    --body)
      BODY="$2"
      shift 2
      ;;
    --draft)
      DRAFT=true
      shift
      ;;
    *)
      die "Unknown argument: $1"
      ;;
  esac
done

# Safety: cannot create PR from protected branches
require_non_protected_branch "$CURRENT_BRANCH"

info "Creating PR for branch: $CURRENT_BRANCH → $BASE_BRANCH"

# Auto-generate title if not provided
if [[ -z "$TITLE" ]]; then
  # Extract type and description from branch name (e.g., feat/add-login → Add login)
  if [[ "$CURRENT_BRANCH" =~ ^(feat|fix|chore|refactor|docs|test)/(.+)$ ]]; then
    TYPE="${BASH_REMATCH[1]}"
    DESC="${BASH_REMATCH[2]}"
    # Capitalize first letter and replace hyphens with spaces
    TYPE_CAPITALIZED="$(echo "${TYPE:0:1}" | tr '[:lower:]' '[:upper:]')${TYPE:1}"
    DESC_FORMATTED="$(echo "$DESC" | sed -E 's|-| |g')"
    TITLE="${TYPE_CAPITALIZED}: ${DESC_FORMATTED}"
  else
    TITLE="$CURRENT_BRANCH"
  fi
fi

# Auto-generate body if not provided
if [[ -z "$BODY" ]]; then
  BODY="### Summary

$(git log --oneline "$BASE_BRANCH..$CURRENT_BRANCH" | sed 's/^/- /')

## Test Plan

- [ ] Tests added/updated
- [ ] Manual testing completed
- [ ] Documentation updated if needed

"
fi

# Check if PR already exists
EXISTING_PR=$(gh pr list --head "$CURRENT_BRANCH" --base "$BASE_BRANCH" --json number --jq '.[0].number' || echo "")

if [[ -n "$EXISTING_PR" && "$EXISTING_PR" != "null" ]]; then
  info "PR already exists: #$EXISTING_PR"
  exit 0
fi

# Create the PR
info "Creating PR with title: $TITLE"

PR_ARGS=(--base "$BASE_BRANCH" --head "$CURRENT_BRANCH" --title "$TITLE" --body "$BODY")

if [[ "$DRAFT" == true ]]; then
  PR_ARGS+=(--draft)
fi

PR_URL=$(gh pr create "${PR_ARGS[@]}")

info "✅ PR created: $PR_URL"

# Auto-add labels based on branch type
if [[ "$CURRENT_BRANCH" =~ ^(feat|fix|chore|refactor|docs|test)/ ]]; then
  LABELS=("$(echo "$CURRENT_BRANCH" | sed -E 's|/.*||')")
  gh pr edit "$PR_URL" --add-label "${LABELS[@]}"
fi

info "PR URL: $PR_URL"
