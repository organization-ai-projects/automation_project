#!/usr/bin/env bash
set -euo pipefail

# Ensure the script is executed only in a CI environment
if [[ -z "${CI:-}" || "$CI" != "true" ]]; then
  echo "This script can only be executed in a CI environment." >&2
  exit 1
fi

# Check if gh CLI is available
if ! command -v gh &> /dev/null; then
  echo "gh CLI not found. Please install GitHub CLI before running this script." >&2
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOGGING_SH="$SCRIPT_DIR/../../../../common_lib/core/logging.sh"
if [[ -f "$LOGGING_SH" ]]; then
  # shellcheck disable=SC1090
  source "$LOGGING_SH"
else
  info() { echo "$*"; }
fi

# Standardize token usage
# Use APP_GH_TOKEN for all operations to ensure consistent permissions.
export GITHUB_TOKEN="${APP_GH_TOKEN:-${GH_TOKEN}}"

REMOTE="${REMOTE:-origin}"
MAIN="${MAIN:-main}"
DEV="${DEV:-dev}"
SYNC_BRANCH="sync/main-into-dev"

# Fetch branches
git fetch "$REMOTE"

# Clean up existing sync branch if it exists
if git show-ref --verify --quiet "refs/heads/$SYNC_BRANCH"; then
  info "Removing existing local sync branch..."
  git branch -D "$SYNC_BRANCH" || true
fi

# Delete remote sync branch if it exists
if git ls-remote --heads "$REMOTE" "$SYNC_BRANCH" | grep -q "$SYNC_BRANCH"; then
  info "Removing existing remote sync branch..."
  git push "$REMOTE" --delete "$SYNC_BRANCH" || true
fi

# Create sync branch from main (not dev)
info "Creating sync branch from $MAIN..."
git switch -C "$SYNC_BRANCH" "$REMOTE/$MAIN"

# Push sync branch to remote
info "Pushing sync branch to remote..."
git push -f "$REMOTE" "$SYNC_BRANCH"

# Create PR
info "Creating PR to merge $MAIN into $DEV..."
PR_CREATE_OUTPUT=$(gh pr create \
  --base "$DEV" \
  --head "$SYNC_BRANCH" \
  --title "chore: sync main into dev" \
  --body "Automated sync after merge into main." 2>&1) || {

  # PR creation failed - check if it's because branches are identical
  if echo "$PR_CREATE_OUTPUT" | grep -q "No commits between"; then
    info "ℹ️ No sync needed - dev is already up to date with main"
    exit 0
  else
    echo "❌ Failed to create PR: $PR_CREATE_OUTPUT" >&2
    exit 1
  fi
}

PR_URL="$PR_CREATE_OUTPUT"
info "Created PR: $PR_URL"

# Wait for PR to stabilize
info "Waiting for PR to stabilize..."
STABLE_TIMEOUT_SECS="${STABLE_TIMEOUT_SECS:-120}"
STABLE_DEADLINE=$(( $(date +%s) + STABLE_TIMEOUT_SECS ))
while true; do
  now=$(date +%s)
  if [[ $now -ge $STABLE_DEADLINE ]]; then
    echo "PR did not stabilize in time. Exiting." >&2
    exit 1
  fi

  MERGEABLE=$(gh pr view "$PR_URL" --json mergeable --jq '.mergeable // "UNKNOWN"')
  if [[ "$MERGEABLE" != "UNKNOWN" ]]; then
    break
  fi

  sleep 5
done

# Enable auto-merge with branch deletion
gh pr merge "$PR_URL" --auto --merge --delete-branch
info "Auto-merge enabled for PR: $PR_URL (branch will be deleted after merge)"