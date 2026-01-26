#!/usr/bin/env bash
set -euo pipefail

REMOTE="${REMOTE:-origin}"
MAIN="${MAIN:-main}"
DEV="${DEV:-dev}"
SYNC_BRANCH="${SYNC_BRANCH:-sync/dev-with-main}"

PR_TITLE="chore: sync dev with main"
PR_BODY="Automated sync PR to bring dev up-to-date with main merge commits. No functional changes intended."

# Choose merge method for the PR into dev:
#   --merge  = merge commit
#   --squash = squash merge
#   --rebase = rebase merge (if allowed)
MERGE_METHOD="${MERGE_METHOD:---merge}"

info() { echo "$(date '+%Y-%m-%d %H:%M:%S') INFO: $*"; }
die() { echo "$(date '+%Y-%m-%d %H:%M:%S') ERROR: $*" >&2; exit 1; }

# Validate environment
git rev-parse --is-inside-work-tree >/dev/null 2>&1 || die "Not a git repository."
command -v gh >/dev/null 2>&1 || die "'gh' not found. Install GitHub CLI and run: gh auth login"
command -v jq >/dev/null 2>&1 || die "'jq' not found. Install jq and try again."

# Safety: don't run with dirty tree
if ! git diff --quiet || ! git diff --cached --quiet; then
  die "Working tree is dirty. Commit/stash your changes first."
fi

info "Fetching remote refs..."
git fetch --prune "$REMOTE"

# Ensure local dev exists and is up to date with remote dev
info "Updating local '$DEV' from $REMOTE/$DEV..."
git switch "$DEV" >/dev/null
git pull --ff-only "$REMOTE" "$DEV"

# Create or reset the sync branch from dev
if git show-ref --verify --quiet "refs/heads/$SYNC_BRANCH"; then
  info "Sync branch exists locally. Resetting it to '$DEV'..."
  git switch "$SYNC_BRANCH" >/dev/null
  git reset --hard "$DEV" >/dev/null
else
  info "Creating sync branch '$SYNC_BRANCH' from '$DEV'..."
  git switch -c "$SYNC_BRANCH" "$DEV" >/dev/null
fi

# Merge main into sync branch
info "Merging $REMOTE/$MAIN into '$SYNC_BRANCH'..."
if ! git merge --no-edit "$REMOTE/$MAIN"; then
  die "Merge conflicts while syncing main -> dev. Resolve conflicts manually, then rerun."
fi

# Push sync branch (force-with-lease in case it already existed on remote)
info "Pushing '$SYNC_BRANCH' to $REMOTE..."
git push -u "$REMOTE" "$SYNC_BRANCH" --force-with-lease

# Find existing PR (if any) for this head -> base, else create it
info "Finding existing PR for '$SYNC_BRANCH' -> '$DEV'..."
PR_NUMBER="$(gh pr list --head "$SYNC_BRANCH" --base "$DEV" --json number --jq '.[0].number' || true)"

if [[ -z "${PR_NUMBER:-}" || "${PR_NUMBER:-null}" == "null" ]]; then
  info "No PR found. Creating PR..."
  PR_URL="$(gh pr create \
    --base "$DEV" \
    --head "$SYNC_BRANCH" \
    --title "$PR_TITLE" \
    --body "$PR_BODY" \
    --label "chore" \
    --label "sync_branch")"
  PR_NUMBER="$(gh pr view "$PR_URL" --json number --jq '.number')"
else
  info "PR already exists: #$PR_NUMBER"
fi

# Enable auto-merge (will merge once required checks pass + approvals rules satisfied)
info "Enabling auto-merge for PR #$PR_NUMBER..."
# gh will fail if auto-merge isn't allowed by repo settings / branch protections
gh pr merge "$PR_NUMBER" --auto $MERGE_METHOD || die "Failed to enable auto-merge. Check repo settings / permissions / required approvals."

# Wait until PR is merged (or fails)
info "Waiting for PR #$PR_NUMBER to be merged..."
while true; do
  STATE="$(gh pr view "$PR_NUMBER" --json state --jq '.state')"
  if [[ "$STATE" == "MERGED" ]]; then
    info "PR merged."
    break
  fi
  if [[ "$STATE" == "CLOSED" ]]; then
    die "PR was closed without merge."
  fi
  sleep 5
done

# Update local dev now that remote dev advanced
info "Pulling latest '$DEV'..."
git switch "$DEV" >/dev/null
git pull --ff-only "$REMOTE" "$DEV"

# Delete the sync branch locally and remotely after the PR is merged
info "Deleting local and remote sync branch '$SYNC_BRANCH'..."
git branch -d "$SYNC_BRANCH" || die "Failed to delete local branch '$SYNC_BRANCH'."
git push "$REMOTE" --delete "$SYNC_BRANCH" || die "Failed to delete remote branch '$SYNC_BRANCH'."

info "✅ Sync complete. Your local '$DEV' is up to date. You can continue working."
