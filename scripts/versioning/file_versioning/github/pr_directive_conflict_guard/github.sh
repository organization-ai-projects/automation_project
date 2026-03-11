#!/usr/bin/env bash
# shellcheck shell=bash

pr_directive_conflict_guard_resolve_repo_name() {
  local repo_name="${1:-}"
  if [[ -z "$repo_name" && -n "${GH_REPO:-}" ]]; then
    repo_name="$GH_REPO"
  fi
  if [[ -z "$repo_name" ]] && command -v va_exec >/dev/null 2>&1; then
    repo_name="$(va_exec issue repo-name 2>/dev/null || true)"
  fi
  if [[ -z "$repo_name" ]]; then
    repo_name="$(gh_cli_resolve_repo_name)"
  fi
  if [[ -z "$repo_name" ]]; then
    echo "Error: unable to determine repository." >&2
    exit 3
  fi
  printf '%s' "$repo_name"
}

pr_directive_conflict_guard_fetch_pr_details_json() {
  local repo_name="$1"
  local pr_number="$2"
  local pr_details_json=""
  local pr_title pr_body commit_messages

  pr_title="$(github_pr_field "$repo_name" "$pr_number" "title" || true)"
  pr_body="$(github_pr_field "$repo_name" "$pr_number" "body" || true)"
  commit_messages="$(github_pr_field "$repo_name" "$pr_number" "commit-messages" || true)"

  if [[ -n "$pr_title" || -n "$pr_body" || -n "$commit_messages" ]]; then
    pr_details_json="$(
      jq -c -n \
        --argjson number "$pr_number" \
        --arg title "$pr_title" \
        --arg body "$pr_body" \
        --arg commit_messages "$commit_messages" \
        '{number: $number, title: $title, body: $body, commit_messages: $commit_messages}' \
        2>/dev/null || true
    )"
  fi

  printf '%s' "$pr_details_json"
}

pr_directive_conflict_guard_upsert_pr_comment() {
  local repo_name="$1"
  local pr_number="$2"
  local marker="$3"
  local body="$4"
  github_pr_upsert_comment "$repo_name" "$pr_number" "$marker" "$body"
}

pr_directive_conflict_guard_update_pr_body() {
  local repo_name="$1"
  local pr_number="$2"
  local body="$3"
  github_pr_update_body "$repo_name" "$pr_number" "$body"
}
