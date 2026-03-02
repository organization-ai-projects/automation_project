#!/usr/bin/env bash
set -euo pipefail

# Public execute orchestrator.
# Monitor CI status of a pull request
# Usage: ./ci_watch_pr.sh [pr-number]
# If no PR number provided, tries to find PR for current branch

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/github/pull_request_lookup.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/github/pull_request_lookup.sh"

require_git_repo
require_cmd gh

POLL_INTERVAL="${POLL_INTERVAL:-10}"  # seconds
MAX_WAIT="${MAX_WAIT:-3600}"  # 1 hour default

# Determine PR number
if [[ "$#" -ge 1 ]]; then
  PR_NUMBER="$1"
else
  CURRENT_BRANCH="$(get_current_branch)"
  info "Finding PR for branch: $CURRENT_BRANCH"
  PR_NUMBER="$(github_find_pr_number_by_branch "$CURRENT_BRANCH" || true)"
  if [[ -z "$PR_NUMBER" ]]; then
    die "No PR found for branch '$CURRENT_BRANCH'."
  fi
fi

info "Watching CI for PR #$PR_NUMBER..."

START_TIME=$(date +%s)

while true; do
  ELAPSED=$(($(date +%s) - START_TIME))

  if [[ $ELAPSED -gt $MAX_WAIT ]]; then
    die "Timeout: CI not complete after ${MAX_WAIT}s"
  fi

  # Get PR status
  PR_DATA=$(vcs_remote_pr_view "$PR_NUMBER" --json state,statusCheckRollup,mergeable)

  STATE=$(echo "$PR_DATA" | jq -r '.state')
  MERGEABLE=$(echo "$PR_DATA" | jq -r '.mergeable // "UNKNOWN"')

  # Parse status checks
  CHECKS=$(echo "$PR_DATA" | jq -r '.statusCheckRollup // []')
  TOTAL=$(echo "$CHECKS" | jq 'length')

  if [[ "$TOTAL" -eq 0 ]]; then
    info "No status checks found. Retrying..."
    sleep "$POLL_INTERVAL"
    continue
  fi

  PENDING=$(echo "$CHECKS" | jq '[.[] | select(.conclusion == null)] | length')
  SUCCESS=$(echo "$CHECKS" | jq '[.[] | select(.conclusion == "SUCCESS")] | length')
  FAILURE=$(echo "$CHECKS" | jq '[.[] | select(.conclusion == "FAILURE")] | length')

  info "[$ELAPSED s] State: $STATE | Mergeable: $MERGEABLE | Checks: $SUCCESS/$TOTAL passed, $FAILURE failed, $PENDING pending"

  if [[ "$FAILURE" -gt 0 ]]; then
    die "CI failed for PR #$PR_NUMBER."
  fi

  if [[ "$PENDING" -eq 0 && "$SUCCESS" -eq "$TOTAL" ]]; then
    info "All checks passed for PR #$PR_NUMBER."
    break
  fi

  sleep "$POLL_INTERVAL"
done
