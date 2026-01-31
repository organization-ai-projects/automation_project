#!/usr/bin/env bash
set -euo pipefail

# Ensure the script is executed only in a CI environment
if [[ -z "${CI:-}" || "$CI" != "true" ]]; then
  echo "This script can only be executed in a CI environment." >&2
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

REMOTE="${REMOTE:-origin}"
MAIN="${MAIN:-main}"
DEV="${DEV:-dev}"
SYNC_BRANCH="sync/main-into-dev"

# Fetch branches
git fetch "$REMOTE"

# Handle sync branch conflicts
if git show-ref --verify --quiet "refs/heads/$SYNC_BRANCH"; then
  echo "Sync branch already exists. Reusing existing branch." >&2
  git switch "$SYNC_BRANCH"
else
  git switch -C "$SYNC_BRANCH" "$REMOTE/$DEV"
fi

# Handle merge conflicts explicitly
if ! git merge --no-edit "$REMOTE/$MAIN"; then
  echo "Merge conflict detected. Please resolve conflicts manually." >&2
  exit 1
fi

# Create PR
PR_URL=$(gh pr create \
  --base "$DEV" \
  --head "$SYNC_BRANCH" \
  --title "chore: sync main into dev" \
  --body "Automated sync after merge into main.")
info "Created PR: $PR_URL"

# Avoid duplicate issue creation
EXISTING_ISSUE=$(gh issue list --label "sync-failure" --state open --json title --jq '.[] | select(.title == "Sync Failure: main → dev")')
if [[ -n "$EXISTING_ISSUE" ]]; then
  echo "An open issue for sync failure already exists. Skipping issue creation." >&2
else
  gh issue create --title "Sync Failure: main → dev" --body "The sync operation failed. Please investigate." --label "sync-failure"
fi

# Check if gh CLI is available
if ! command -v gh &> /dev/null; then
  echo "gh CLI not found. Please install GitHub CLI before running this script." >&2
  exit 1
fi

# Standardize token usage
# Use APP_GH_TOKEN for all operations to ensure consistent permissions.
export GITHUB_TOKEN="$APP_GH_TOKEN"

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

# Enable auto-merge
gh pr merge "$PR_URL" --auto --merge
info "Auto-merge enabled for PR: $PR_URL"