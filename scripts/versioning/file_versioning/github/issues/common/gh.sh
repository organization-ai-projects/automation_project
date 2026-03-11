#!/usr/bin/env bash

# shellcheck source=scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh
source "${ISSUES_DIR}/../../../../common_lib/versioning/file_versioning/github/issue_helpers.sh"

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
  github_issue_repo_name
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
  github_issue_label_exists "$repo" "$label"
}

issue_gh_issue_state() {
  local repo="$1"
  local issue_number="$2"
  github_issue_state "$repo" "$issue_number"
}

issue_gh_issue_json() {
  local repo="${1:-}"
  local issue_number="$2"
  local json_fields="$3"
  github_issue_read_json "$repo" "$issue_number" "$json_fields"
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
    if [[ -z "$pr_json" ]]; then
      local pr_title pr_body
      pr_title="$(gh pr view "$pr_number" -R "$repo" --json title -q '.title // ""' 2>/dev/null || true)"
      pr_body="$(gh pr view "$pr_number" -R "$repo" --json body -q '.body // ""' 2>/dev/null || true)"
      if [[ -n "$pr_title" || -n "$pr_body" ]]; then
        pr_json="$(jq -c -n --arg title "$pr_title" --arg body "$pr_body" \
          '{number: 0, url: "", title: $title, body: $body}' 2>/dev/null || true)"
      fi
    fi
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
  github_issue_has_label "$repo" "$issue_number" "$label"
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

issue_gh_open_prs_referencing_issue() {
  local repo="$1"
  local issue_number="$2"
  local pr_numbers=""

  if command -v va_exec >/dev/null 2>&1; then
    pr_numbers="$(
      va_exec pr open-referencing-issue \
        --issue "$issue_number" \
        --repo "$repo" 2>/dev/null || true
    )"
  fi
  if [[ -n "$pr_numbers" ]]; then
    printf '%s\n' "$pr_numbers"
    return 0
  fi

  {
    gh api "repos/${repo}/pulls?state=open&per_page=100" --paginate --jq '.[]. | [.number, (.body // "")] | @tsv' 2>/dev/null |
      while IFS=$'\t' read -r pr_num pr_body; do
        [[ -n "$pr_num" ]] || continue
        issue_refs_extract_all_closing_numbers "$pr_body" | grep -qx "$issue_number" || continue
        printf '%s\n' "$pr_num"
      done
  } || true
}

issue_gh_issue_update() {
  local repo="$1"
  local issue_number="$2"
  shift 2
  github_issue_update "$repo" "$issue_number" "$@"
}

issue_gh_issue_reopen() {
  local repo="$1"
  local issue_number="$2"
  github_issue_reopen "$repo" "$issue_number"
}

issue_gh_issue_close() {
  local repo="$1"
  local issue_number="$2"
  local reason="$3"
  github_issue_close "$repo" "$issue_number" "$reason"
}
