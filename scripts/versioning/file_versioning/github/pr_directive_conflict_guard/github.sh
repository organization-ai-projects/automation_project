#!/usr/bin/env bash
# shellcheck shell=bash

pr_directive_conflict_guard_resolve_repo_name() {
  local repo_name="${1:-}"
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

  if command -v va_exec >/dev/null 2>&1; then
    pr_details_json="$(
      va_exec pr details \
        --pr "$pr_number" \
        --repo "$repo_name" 2>/dev/null || true
    )"
  fi

  if [[ -z "$pr_details_json" ]]; then
    local pr_json commit_messages
    pr_json="$(gh pr view "$pr_number" -R "$repo_name" --json body,url,number,title 2>/dev/null || true)"
    if [[ -n "$pr_json" ]]; then
      commit_messages="$(gh api "repos/${repo_name}/pulls/${pr_number}/commits" --paginate --jq '.[].commit.message' 2>/dev/null || true)"
      pr_details_json="$(
        jq -c --arg commit_messages "$commit_messages" \
          '. + { commit_messages: $commit_messages }' <<<"$pr_json" 2>/dev/null || true
      )"
    fi
  fi

  printf '%s' "$pr_details_json"
}

pr_directive_conflict_guard_upsert_pr_comment() {
  local repo_name="$1"
  local pr_number="$2"
  local marker="$3"
  local body="$4"
  local comment_id

  if command -v va_exec >/dev/null 2>&1; then
    if va_exec pr upsert-comment \
      --pr "$pr_number" \
      --repo "$repo_name" \
      --marker "$marker" \
      --body "$body" >/dev/null; then
      return 0
    fi
  fi

  comment_id="$(
    gh api "repos/${repo_name}/issues/${pr_number}/comments" --paginate |
      jq -r --arg marker "$marker" '
        map(select((.body // "") | contains($marker)))
        | sort_by(.updated_at)
        | last
        | .id // empty
      ' 2>/dev/null || true
  )"

  if [[ -n "$comment_id" ]]; then
    gh api -X PATCH "repos/${repo_name}/issues/comments/${comment_id}" -f body="$body" >/dev/null
  else
    gh api "repos/${repo_name}/issues/${pr_number}/comments" -f body="$body" >/dev/null
  fi
}

pr_directive_conflict_guard_update_pr_body() {
  local repo_name="$1"
  local pr_number="$2"
  local body="$3"

  if command -v va_exec >/dev/null 2>&1; then
    if va_exec pr update-body --pr "$pr_number" --repo "$repo_name" --body "$body" >/dev/null; then
      return 0
    fi
  fi

  gh pr edit "$pr_number" -R "$repo_name" --body "$body" >/dev/null
}
