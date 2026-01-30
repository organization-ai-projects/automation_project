#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/core/command.sh
source "$ROOT_DIR/scripts/common_lib/core/command.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/working_tree.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/working_tree.sh"

REMOTE="${REMOTE:-origin}"
MAIN="${MAIN:-main}"
DEV="${DEV:-dev}"

SYNC_BRANCH_PREFIX="sync/dev-with-main"

PR_TITLE="chore: sync dev with main"

# Add existing labels
PR_LABELS=("chore" "sync_branch")

# Add an explicit author
GIT_AUTHOR_NAME="Automate Bot"
GIT_AUTHOR_EMAIL="automate-bot@users.noreply.github.com"
GIT_COMMITTER_NAME="$GIT_AUTHOR_NAME"
GIT_COMMITTER_EMAIL="$GIT_AUTHOR_EMAIL"
export GIT_AUTHOR_NAME GIT_AUTHOR_EMAIL GIT_COMMITTER_NAME GIT_COMMITTER_EMAIL

# Choose merge method for the PR into dev:
#   --merge  = merge commit
#   --squash = squash merge
#   --rebase = rebase merge (if allowed)
MERGE_METHOD="${MERGE_METHOD:---merge}"

# Timeout for waiting for the PR to merge (in seconds)
MAX_WAIT_SECONDS="${MAX_WAIT_SECONDS:-600}"  # Default to 10 minutes

# Max retry attempts if main advances during sync (default 1)
MAX_RETRIES="${MAX_RETRIES:-1}"

# Strict main SHA enforcement (default "true")
STRICT_MAIN_SHA="${STRICT_MAIN_SHA:-true}"

# Save current branch to restore on exit
INITIAL_BRANCH="$(git branch --show-current)"
trap 'git switch "$INITIAL_BRANCH" 2>/dev/null || true' EXIT

# Validate environment
require_git_repo
require_clean_tree

# Retry loop
attempt=0

while [[ $attempt -le $MAX_RETRIES ]]; do
  if [[ $attempt -gt 0 ]]; then
    info "Retry attempt $attempt/$MAX_RETRIES due to main branch advancement..."
  fi

  # Generate fresh user, timestamp, and sync branch name for each attempt
  GIT_USER_RAW="$(git config user.name 2>/dev/null || echo "")"
  if [[ -z "$GIT_USER_RAW" ]]; then
    GIT_USER_RAW="$(whoami)"
  fi
  GIT_USER="${GIT_USER:-$(echo "$GIT_USER_RAW" | tr ' ' '-' | tr '[:upper:]' '[:lower:]')}"
  TIMESTAMP="$(date '+%Y%m%d-%H%M%S')"

  info "Fetching remote refs..."
  git fetch --prune "$REMOTE"

  # Ensure remote branches exist
  if ! git rev-parse --verify "$REMOTE/$MAIN" >/dev/null 2>&1; then
    die "Remote branch '$REMOTE/$MAIN' does not exist."
  fi
  if ! git rev-parse --verify "$REMOTE/$DEV" >/dev/null 2>&1; then
    die "Remote branch '$REMOTE/$DEV' does not exist."
  fi

  # Capture the SHA of main at the start to ensure it doesn't change
  MAIN_SHA_START="$(git rev-parse "$REMOTE/$MAIN")"
  MAIN_SHORT_SHA="$(git rev-parse --short "$REMOTE/$MAIN")"
  info "Main branch SHA at start: $MAIN_SHA_START"

  # Generate unique sync branch name
  SYNC_BRANCH="${SYNC_BRANCH_PREFIX}/${GIT_USER}-${TIMESTAMP}-${MAIN_SHORT_SHA}"
  info "Sync branch name: $SYNC_BRANCH"

  # Safety check: ensure SYNC_BRANCH is not a protected branch
  [[ "$SYNC_BRANCH" == "$MAIN" ]] && die "SYNC_BRANCH cannot be the same as MAIN ($MAIN)"
  [[ "$SYNC_BRANCH" == "$DEV" ]] && die "SYNC_BRANCH cannot be the same as DEV ($DEV)"

  # Check if dev already contains main (nothing to sync)
  if git merge-base --is-ancestor "$REMOTE/$MAIN" "$REMOTE/$DEV"; then
    info "✅ Dev already contains all commits from main. Nothing to sync."
    exit 0
  fi

  # Ensure local main exists and is up to date with remote main
  info "Updating local '$MAIN' from $REMOTE/$MAIN..."
  if ! git show-ref --verify --quiet "refs/heads/$MAIN"; then
    info "Local branch '$MAIN' not found. Creating from $REMOTE/$MAIN..."
    git switch -c "$MAIN" "$REMOTE/$MAIN" >/dev/null
  else
    git switch "$MAIN" >/dev/null
  fi
  git pull --ff-only "$REMOTE" "$MAIN"

  # Branch existence safety: check if local sync branch already exists
  if git show-ref --verify --quiet "refs/heads/$SYNC_BRANCH"; then
    die "Local branch '$SYNC_BRANCH' already exists. Please delete it first or use a different name."
  fi

  # Create unique sync branch from main (exact copy, no merge)
  info "Creating sync branch '$SYNC_BRANCH' from '$MAIN'..."
  git switch -c "$SYNC_BRANCH" "$MAIN" >/dev/null

  # Branch existence safety: check if remote sync branch already exists
  if git ls-remote --heads "$REMOTE" "$SYNC_BRANCH" | awk '{print $2}' | grep -qx "refs/heads/$SYNC_BRANCH"; then
    die "Remote branch '$REMOTE/$SYNC_BRANCH' already exists. Please delete it first or use a different name."
  fi

  # Push sync branch to remote
  info "Pushing '$SYNC_BRANCH' to $REMOTE..."
  git push -u "$REMOTE" "$SYNC_BRANCH"

  # Generate PR body with sync details
  PR_BODY="Automated sync PR to bring dev up-to-date with main.

**Sync details:**
- Main SHA: \`$MAIN_SHA_START\` (\`$MAIN_SHORT_SHA\`)
- Initiated by: $GIT_USER
- Timestamp: $TIMESTAMP
- Attempt: $((attempt + 1))

This PR syncs the latest changes from main into dev via a clean sync branch."

  # Find existing PR (if any) for this head -> base, else create it
  # Use REMOTE:SYNC_BRANCH to avoid head ambiguity
  info "Finding existing PR for '$SYNC_BRANCH' -> '$DEV'..."
  PR_NUMBER="$(gh pr list --head "$REMOTE:$SYNC_BRANCH" --base "$DEV" --json number --jq '.[0].number' || true)"

  if [[ -z "${PR_NUMBER:-}" || "${PR_NUMBER:-null}" == "null" ]]; then
    info "No PR found. Creating PR..."

    # Create the PR without labels first (labels may not exist)
    PR_URL="$(gh pr create \
      --base "$DEV" \
      --head "$REMOTE:$SYNC_BRANCH" \
      --title "$PR_TITLE" \
      --body "$PR_BODY")"
    PR_NUMBER="$(gh pr view "$PR_URL" --json number --jq '.number')"

    # Add labels if possible (ignore errors)
    for label in "${PR_LABELS[@]}"; do
      gh pr edit "$PR_NUMBER" --add-label "$label" 2>/dev/null || \
        info "Warning: Could not add label '$label' (may not exist)"
    done
    info "Created PR: $PR_URL"
  else
    PR_URL="$(gh pr view "$PR_NUMBER" --json url --jq '.url')"
    info "PR already exists: $PR_URL"
  fi

  # Enable auto-merge (will merge once required checks pass + approvals rules satisfied)
  info "Enabling auto-merge for PR #$PR_NUMBER..."
  # gh will fail if auto-merge isn't allowed by repo settings / branch protections
  gh pr merge "$PR_NUMBER" --auto $MERGE_METHOD || \
    die "Failed to enable auto-merge. Check repo settings, required approvals, and permissions."

  # Wait until PR is merged (or fails)
  info "Waiting for PR #$PR_NUMBER to be merged (timeout: ${MAX_WAIT_SECONDS}s)..."
  START_TIME=$(date +%s)

  MAIN_CHANGED=false

  while true; do
    ELAPSED=$(($(date +%s) - START_TIME))
    if [[ $ELAPSED -gt $MAX_WAIT_SECONDS ]]; then
      die "Timeout: PR #$PR_NUMBER not merged after ${MAX_WAIT_SECONDS}s"
    fi

    # Check PR state and status checks (more robust with .merged bool)
    IS_MERGED="$(gh pr view "$PR_NUMBER" --json merged --jq '.merged')"
    IS_CLOSED="$(gh pr view "$PR_NUMBER" --json closed --jq '.closed')"

    if [[ "$IS_MERGED" == "true" ]]; then
      info "PR merged."
      break
    fi
    if [[ "$IS_CLOSED" == "true" && "$IS_MERGED" != "true" ]]; then
      die "PR was closed without merge."
    fi

    # Check status checks and mergeable (note: mergeable can be UNKNOWN on GitHub, not always reliable)
    MERGEABLE="$(gh pr view "$PR_NUMBER" --json mergeable --jq '.mergeable // "UNKNOWN"')"
    CHECKS_STATUS="$(gh pr view "$PR_NUMBER" --json statusCheckRollup --jq '.statusCheckRollup // [] | map(select(.conclusion != null)) | if length == 0 then "PENDING" else (if all(.conclusion == "SUCCESS") then "SUCCESS" elif any(.conclusion == "FAILURE") then "FAILURE" else "PENDING" end) end')"

    if [[ "$CHECKS_STATUS" == "FAILURE" ]]; then
      die "CI checks failed for PR #$PR_NUMBER. Aborting sync."
    fi

    # Check if main has advanced
    git fetch --prune "$REMOTE" >/dev/null 2>&1
    MAIN_SHA_CURRENT="$(git rev-parse "$REMOTE/$MAIN")"
    if [[ "$MAIN_SHA_CURRENT" != "$MAIN_SHA_START" ]]; then
      if [[ "$STRICT_MAIN_SHA" == "true" ]]; then
        info "Main branch has advanced from $MAIN_SHA_START to $MAIN_SHA_CURRENT during sync."
        info "Auto-merge may still merge the PR after we stop waiting; we are only stopping the wait and retrying."
        MAIN_CHANGED=true
        break
      else
        info "Main branch has advanced from $MAIN_SHA_START to $MAIN_SHA_CURRENT, but STRICT_MAIN_SHA=false. Continuing to wait..."
      fi
    fi

    info "Waiting... (merged: $IS_MERGED, closed: $IS_CLOSED, checks: $CHECKS_STATUS, mergeable: $MERGEABLE, elapsed: ${ELAPSED}s)"
    sleep 10
  done

  # If main changed and we need to retry
  if [[ "$MAIN_CHANGED" == "true" ]]; then
    info "Handling main branch advancement..."

    # Cleanup: delete the sync branch created by this attempt
    info "Cleaning up sync branch '$SYNC_BRANCH' from failed attempt..."

    if [[ "$SYNC_BRANCH" != "${SYNC_BRANCH_PREFIX}/"* ]]; then
      die "Refusing to delete unexpected branch name: $SYNC_BRANCH"
    fi

    # Delete local branch
    if git show-ref --verify --quiet "refs/heads/$SYNC_BRANCH"; then
      git switch "$MAIN" >/dev/null 2>&1 || git switch "$DEV" >/dev/null 2>&1 || true
      git branch -D "$SYNC_BRANCH" 2>/dev/null || info "Warning: Failed to delete local branch '$SYNC_BRANCH'."
    fi

    # Delete remote branch
    if git ls-remote --heads "$REMOTE" "$SYNC_BRANCH" | awk '{print $2}' | grep -qx "refs/heads/$SYNC_BRANCH"; then
      git push "$REMOTE" --delete "$SYNC_BRANCH" 2>/dev/null || info "Warning: Failed to delete remote branch '$SYNC_BRANCH'."
    fi

    # Increment attempt counter
    attempt=$((attempt + 1))

    # Check if we should retry
    if [[ $attempt -le $MAX_RETRIES ]]; then
      info "Will retry with updated main branch..."
      continue
    else
      die "Max retries ($MAX_RETRIES) exceeded. Main branch advanced from $MAIN_SHA_START to $MAIN_SHA_CURRENT. Aborting sync."
    fi
  fi

  # If we reach here, PR was merged successfully
  break
done

# Sync dev local with the updated remote dev
info "Synchronizing local '$DEV' with $REMOTE/$DEV..."
if ! git show-ref --verify --quiet "refs/heads/$DEV"; then
  info "Local branch '$DEV' not found. Creating from $REMOTE/$DEV..."
  git switch -c "$DEV" "$REMOTE/$DEV" >/dev/null
else
  git switch "$DEV" >/dev/null
fi
git pull --ff-only "$REMOTE" "$DEV"

if [[ "$KEEP_SYNC_BRANCH" == "true" ]]; then
  info "KEEP_SYNC_BRANCH=true, leaving sync branch '$SYNC_BRANCH' in place."
else
# Delete the sync branch locally and remotely after the PR is merged
if [[ "$SYNC_BRANCH" != "${SYNC_BRANCH_PREFIX}/"* ]]; then
  die "Refusing to delete unexpected branch name: $SYNC_BRANCH"
fi
info "Deleting local and remote sync branch '$SYNC_BRANCH'..."
git branch -D "$SYNC_BRANCH" || die "Failed to delete local branch '$SYNC_BRANCH'."
git push "$REMOTE" --delete "$SYNC_BRANCH" || die "Failed to delete remote branch '$SYNC_BRANCH'."
fi

info "✅ Sync complete. Both '$MAIN' and '$DEV' are up to date locally. You can continue working."
