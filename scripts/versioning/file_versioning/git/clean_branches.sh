#!/usr/bin/env bash
set -euo pipefail

# Clean up obsolete local and remote branches
# Removes local branches marked as [gone] and optionally stale remote branches

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
DRY_RUN=false

if [[ "$#" -gt 1 ]]; then
  die "Usage: $0 [--dry-run]"
fi

if [[ "$#" -eq 1 ]]; then
  if [[ "$1" == "--dry-run" ]]; then
    DRY_RUN=true
  else
    die "Unknown argument: $1 (expected: --dry-run)"
  fi
fi

info "Cleaning up stale branches..."
if [[ "$DRY_RUN" == true ]]; then
  warn "DRY-RUN mode enabled: no branch will be deleted."
fi

# Fetch with prune to update remote tracking
git_fetch_prune "$REMOTE"

# Find branches marked as [gone]
mapfile -t GONE_BRANCHES < <(
  git branch -vv | awk '$0 ~ /\[.*: gone\]/ {
    branch=$1
    if (branch == "*") {
      branch=$2
    }
    print branch
  }'
)

deleted_count=0
skipped_count=0
failed_count=0

if [[ "${#GONE_BRANCHES[@]}" -eq 0 ]]; then
  info "✓ No local branches with gone remotes."
else
  info "Found local branches with gone remotes:"
  printf '%s\n' "${GONE_BRANCHES[@]}" | sed 's/^/  - /'

  # Delete each gone branch (avoid pipe subshell to preserve shell semantics)
  while read -r branch; do
    [[ -z "$branch" || "$branch" == "*" ]] && continue

    # Skip if it's a protected branch
    if is_protected_branch "$branch"; then
      warn "Skipping protected branch: $branch"
      skipped_count=$((skipped_count + 1))
      continue
    fi

    if [[ "$DRY_RUN" == true ]]; then
      info "[DRY-RUN] Would delete local branch: $branch"
      continue
    fi

    info "Deleting local branch: $branch"
    if git branch -d "$branch" 2>/dev/null; then
      info "✓ Deleted $branch (safe)"
      deleted_count=$((deleted_count + 1))
    elif git branch -D "$branch" 2>/dev/null; then
      warn "⚠ Deleted $branch (forced)"
      deleted_count=$((deleted_count + 1))
    else
      warn "⚠ Failed to delete $branch"
      failed_count=$((failed_count + 1))
    fi
  done <<< "$(printf '%s\n' "${GONE_BRANCHES[@]}")"
fi

# Optional: List merged branches that could be cleaned up
info "Checking for fully merged local branches..."
mapfile -t MERGED_BRANCHES < <(
  git branch --merged "${BASE_BRANCH:-dev}" | while read -r line; do
    [[ -z "$line" ]] && continue
    [[ "$line" == \** ]] && continue

    branch="${line#"${line%%[![:space:]]*}"}"
    if [[ -n "$branch" ]] && ! is_protected_branch "$branch"; then
      printf '%s\n' "$branch"
    fi
  done
)

if [[ "${#MERGED_BRANCHES[@]}" -gt 0 ]]; then
  info "Local branches fully merged into ${BASE_BRANCH:-dev}:"
  printf '%s\n' "${MERGED_BRANCHES[@]}" | sed 's/^/  - /'
  info "To delete these, run: git branch -d <branch-name>"
else
  info "✓ No additional merged branches to clean up."
fi

if [[ "$DRY_RUN" == true ]]; then
  info "✅ Branch cleanup dry-run complete."
else
  info "✅ Branch cleanup complete (deleted=$deleted_count, skipped=$skipped_count, failed=$failed_count)."
fi
