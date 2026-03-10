#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Issue-view helpers shared across PR modules.

pr_issue_view_full_json() {
  local issue_number="$1"
  local issue_key="#${issue_number}"
  local repo_name_with_owner
  local issue_json

  if [[ -n "${issue_view_full_json_cache[$issue_key]:-}" ]]; then
    echo "${issue_view_full_json_cache[$issue_key]}"
    return
  fi

  if [[ "$has_gh" != "true" ]]; then
    issue_view_full_json_cache["$issue_key"]=""
    echo ""
    return
  fi

  repo_name_with_owner="$(pr_get_repo_name_with_owner)"
  if command -v va_exec >/dev/null 2>&1; then
    if [[ -n "$repo_name_with_owner" ]]; then
      issue_json="$(va_exec pr issue-view --issue "$issue_number" --repo "$repo_name_with_owner" 2>/dev/null || true)"
    else
      issue_json="$(va_exec pr issue-view --issue "$issue_number" 2>/dev/null || true)"
    fi
  fi

  if [[ -z "$issue_json" ]]; then
    if [[ -n "$repo_name_with_owner" ]]; then
      issue_json="$(gh issue view "$issue_number" -R "$repo_name_with_owner" --json title,body,labels 2>/dev/null || true)"
    else
      issue_json="$(gh issue view "$issue_number" --json title,body,labels 2>/dev/null || true)"
    fi
  fi

  issue_view_full_json_cache["$issue_key"]="$issue_json"
  echo "$issue_json"
}
