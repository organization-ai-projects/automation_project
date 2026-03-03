#!/usr/bin/env bash
set -euo pipefail

# Remove local branches whose remote counterparts have disappeared
# Usage: ./clean_local_gone.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../.." && pwd)"

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

info "Cleaning local branches with gone remotes..."

# Fetch with prune to update remote tracking
git_fetch_prune "$REMOTE"

# Find branches marked as [gone]
GONE_BRANCHES=$(git branch -vv | awk '/: gone]/{print $1}' || true)

if [[ -z "$GONE_BRANCHES" ]]; then
  info "✓ No local branches with gone remotes."
  exit 0
fi

info "Found branches to delete:"
echo "$GONE_BRANCHES" | sed 's/^/  - /'

# Delete each gone branch
echo "$GONE_BRANCHES" | while read -r branch; do
  # Skip if it's a protected branch
  if is_protected_branch "$branch"; then
    warn "Skipping protected branch: $branch"
    continue
  fi

  info "Deleting local branch: $branch"
  if git branch -d "$branch" 2>/dev/null; then
    info "✓ Deleted $branch (safe)"
  elif git branch -D "$branch" 2>/dev/null; then
    warn "⚠ Deleted $branch (forced)"
  else
    warn "⚠ Failed to delete $branch"
  fi
done

info "✅ Cleanup complete."
