#!/usr/bin/env bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

gh_resolve_repo_name() {
  if [[ -n "${GH_REPO:-}" ]]; then
    echo "$GH_REPO"
    return 0
  fi
  gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true
}

gh_label_exists() {
  local repo="$1"
  local label="$2"
  gh label list -R "$repo" --limit 1000 --json name --jq '.[].name' 2>/dev/null \
    | grep -Fxq "$label"
}

gh_issue_state() {
  local repo="$1"
  local issue_number="$2"
  gh issue view "$issue_number" -R "$repo" --json state -q '.state // ""' 2>/dev/null || true
}

gh_issue_has_label() {
  local repo="$1"
  local issue_number="$2"
  local label="$3"
  gh issue view "$issue_number" -R "$repo" --json labels --jq '.labels[].name' 2>/dev/null \
    | grep -Fxq "$label"
}
