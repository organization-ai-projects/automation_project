#!/usr/bin/env bash
# shellcheck shell=bash

# GitHub issue fetch helpers for compliance evaluation.

issue_fetch_non_compliance_reason() {
  local issue_number="${1:-}"
  local repo_name="${2:-}"
  local labels_raw
  local title
  local body
  local va_reason=""

  if [[ -n "$repo_name" ]]; then
    if va_reason="$(
      va_exec issue fetch-non-compliance-reason \
        --issue "$issue_number" \
        --repo "$repo_name" 2>/dev/null
    )"; then
      echo "$va_reason"
      return
    fi
  else
    if va_reason="$(
      va_exec issue fetch-non-compliance-reason \
        --issue "$issue_number" 2>/dev/null
    )"; then
      echo "$va_reason"
      return
    fi
  fi

  title="$(github_issue_field "$repo_name" "$issue_number" "title" || true)"
  body="$(github_issue_field "$repo_name" "$issue_number" "body" || true)"
  labels_raw="$(github_issue_field "$repo_name" "$issue_number" "labels-raw" || true)"

  if [[ -z "$title" && -z "$body" && -z "$labels_raw" ]]; then
    echo ""
    return
  fi

  issue_non_compliance_reason_from_content "$title" "$body" "$labels_raw"
}
