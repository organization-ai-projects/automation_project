#!/usr/bin/env bash

issue_gh_require_cmd() {
  local cmd="$1"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Error: command '${cmd}' is required." >&2
    exit 3
  fi
}

issue_gh_require_gh_and_jq() {
  issue_gh_require_cmd gh
  issue_gh_require_cmd jq
}

issue_gh_resolve_repo_name() {
  if [[ -n "${GH_REPO:-}" ]]; then
    echo "$GH_REPO"
    return 0
  fi
  gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true
}

issue_gh_resolve_repo_name_or_exit() {
  local repo_name="${1:-}"
  local context="${2:-repository}"

  if [[ -z "$repo_name" ]]; then
    repo_name="$(issue_gh_resolve_repo_name)"
  fi
  if [[ -z "$repo_name" ]]; then
    echo "Error: unable to resolve ${context} name." >&2
    exit 3
  fi
  echo "$repo_name"
}

issue_gh_label_exists() {
  local repo="$1"
  local label="$2"
  gh label list -R "$repo" --limit 1000 --json name --jq '.[].name' 2>/dev/null |
    grep -Fxq "$label"
}

issue_gh_issue_state() {
  local repo="$1"
  local issue_number="$2"
  gh issue view "$issue_number" -R "$repo" --json state -q '.state // ""' 2>/dev/null || true
}

issue_gh_pr_state() {
  local repo="$1"
  local pr_number="$2"
  local pr_state=""

  if command -v va_exec >/dev/null 2>&1; then
    pr_state="$(
      va_exec pr pr-state \
        --pr "$pr_number" \
        --repo "$repo" 2>/dev/null || true
    )"
  fi

  if [[ -z "$pr_state" ]]; then
    pr_state="$(gh pr view "$pr_number" -R "$repo" --json state -q '.state // ""' 2>/dev/null || true)"
  fi

  echo "$pr_state"
}

issue_gh_pr_details_json() {
  local repo="$1"
  local pr_number="$2"
  local pr_json=""

  if command -v va_exec >/dev/null 2>&1; then
    pr_json="$(
      va_exec pr details \
        --pr "$pr_number" \
        --repo "$repo" 2>/dev/null || true
    )"
  fi

  if [[ -z "$pr_json" ]]; then
    pr_json="$(gh pr view "$pr_number" -R "$repo" --json number,url,title,body 2>/dev/null || true)"
    if [[ -n "$pr_json" ]]; then
      local commit_messages
      commit_messages="$(gh api "repos/${repo}/pulls/${pr_number}/commits" --paginate --jq '.[].commit.message' 2>/dev/null || true)"
      pr_json="$(
        jq -c --arg commit_messages "$commit_messages" \
          '. + { commit_messages: $commit_messages }' <<<"$pr_json" 2>/dev/null || true
      )"
    fi
  fi

  echo "$pr_json"
}

issue_gh_issue_has_label() {
  local repo="$1"
  local issue_number="$2"
  local label="$3"
  gh issue view "$issue_number" -R "$repo" --json labels --jq '.labels[].name' 2>/dev/null |
    grep -Fxq "$label"
}

issue_gh_collect_pr_text_payload() {
  local repo="$1"
  local pr_number="$2"
  local va_payload=""
  local pr_title
  local pr_body
  local commit_messages

  if command -v va_exec >/dev/null 2>&1; then
    va_payload="$(
      va_exec pr text-payload \
        --pr "$pr_number" \
        --repo "$repo" 2>/dev/null || true
    )"
  fi

  if [[ -n "$va_payload" ]]; then
    printf '%s\n' "$va_payload"
    return 0
  fi

  pr_title="$(gh pr view "$pr_number" -R "$repo" --json title -q '.title // ""' 2>/dev/null || true)"
  pr_body="$(gh pr view "$pr_number" -R "$repo" --json body -q '.body // ""' 2>/dev/null || true)"
  commit_messages="$(gh api "repos/${repo}/pulls/${pr_number}/commits" --paginate --jq '.[].commit.message' 2>/dev/null || true)"

  {
    printf '%s\n' "$pr_title"
    printf '%s\n' "$pr_body"
    printf '%s\n' "$commit_messages"
  }
}
