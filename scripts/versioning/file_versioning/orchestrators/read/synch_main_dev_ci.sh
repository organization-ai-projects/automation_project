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

# Configure Git
info "Configuring Git..."
git config user.name "automation-project-bot"
git config user.email "automation-bot@users.noreply.github.com"

# Fetch branches
git fetch "$REMOTE"

# Check if sync is needed
if git merge-base --is-ancestor "$REMOTE/$MAIN" "$REMOTE/$DEV"; then
  echo "No sync needed. Exiting."
  exit 0
fi

# Create sync branch
git switch -C "$SYNC_BRANCH" "$REMOTE/$DEV"
git merge --no-edit "$REMOTE/$MAIN"
git push -u "$REMOTE" "$SYNC_BRANCH" --force-with-lease

# Create PR
PR_URL=$(gh pr create \
  --base "$DEV" \
  --head "$SYNC_BRANCH" \
  --title "chore: sync main into dev" \
  --body "Automated sync after merge into main.")
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

# Enable auto-merge
gh pr merge "$PR_URL" --auto --merge
info "Auto-merge enabled for PR: $PR_URL"