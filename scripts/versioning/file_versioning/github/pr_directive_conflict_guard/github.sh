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

pr_directive_conflict_guard_fetch_pr_json() {
  local repo_name="$1"
  local pr_number="$2"
  gh pr view "$pr_number" -R "$repo_name" --json body,url,number 2>/dev/null || true
}

pr_directive_conflict_guard_fetch_commit_messages() {
  local repo_name="$1"
  local pr_number="$2"
  gh api "repos/${repo_name}/pulls/${pr_number}/commits" --paginate --jq '.[].commit.message' 2>/dev/null || true
}

pr_directive_conflict_guard_upsert_pr_comment() {
  local repo_name="$1"
  local pr_number="$2"
  local marker="$3"
  local body="$4"
  local comment_id

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
