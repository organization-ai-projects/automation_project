#!/usr/bin/env bash
# shellcheck shell=bash

# GitHub issue fetch helpers for compliance evaluation.

issue_fetch_non_compliance_reason() {
  local issue_number="${1:-}"
  local repo_name="${2:-}"
  local issue_json
  local labels_raw
  local title
  local body

  if [[ -n "$repo_name" ]]; then
    issue_json="$(gh issue view "$issue_number" -R "$repo_name" --json labels,title,body 2>/dev/null || true)"
  else
    issue_json="$(gh issue view "$issue_number" --json labels,title,body 2>/dev/null || true)"
  fi
  if [[ -z "$issue_json" ]]; then
    echo ""
    return
  fi

  labels_raw="$(echo "$issue_json" | jq -r '.labels | map(.name) | join("||")')"
  title="$(echo "$issue_json" | jq -r '.title // ""')"
  body="$(echo "$issue_json" | jq -r '.body // ""')"

  issue_non_compliance_reason_from_content "$title" "$body" "$labels_raw"
}
