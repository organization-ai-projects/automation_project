#!/usr/bin/env bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# shellcheck source=scripts/common_lib/versioning/file_versioning/github/commands.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/commands.sh"

# Shared GitHub issue helpers for shell automation scripts.

gh_resolve_repo_name() {
  if [[ -n "${GH_REPO:-}" ]]; then
    echo "$GH_REPO"
    return 0
  fi
  vcs_remote_repo_view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true
}

gh_label_exists() {
  local repo="$1"
  local label="$2"
  vcs_remote_label_list -R "$repo" --limit 1000 --json name --jq '.[].name' 2>/dev/null \
    | grep -Fxq "$label"
}

github_issue_view_field() {
  local repo="$1"
  local issue_number="$2"
  local json_field="$3"
  local jq_query="$4"
  vcs_remote_issue_view "$issue_number" -R "$repo" --json "$json_field" --jq "$jq_query" 2>/dev/null || true
}

gh_issue_state() {
  local repo="$1"
  local issue_number="$2"
  github_issue_view_field "$repo" "$issue_number" "state" '.state // ""'
}

gh_issue_has_label() {
  local repo="$1"
  local issue_number="$2"
  local label="$3"
  github_issue_view_field "$repo" "$issue_number" "labels" '.labels[].name' | grep -Fxq "$label"
}

github_issue_extract_tasklist_refs() {
  local body="${1:-}"
  echo "$body" \
    | awk '
      /-[[:space:]]*\[[xX ]\]/ {
        line = $0
        while (match(line, /#[0-9]+/)) {
          ref = substr(line, RSTART, RLENGTH)
          print ref
          line = substr(line, RSTART + RLENGTH)
        }
      }
    ' \
    | sort -u
}

github_issue_extract_subissue_refs() {
  local repo_owner="${1:-}"
  local repo_short_name="${2:-}"
  local parent_number="${3:-}"

  vcs_remote_api graphql \
    -f query='query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){issue(number:$number){subIssues(first:100){nodes{number}}}}}' \
    -f owner="$repo_owner" \
    -f name="$repo_short_name" \
    -F number="$parent_number" \
    --jq '.data.repository.issue.subIssues.nodes[]?.number | "#"+tostring' 2>/dev/null || true
}

github_issue_list_open_by_label() {
  local label="${1:-}"
  local repo_name="${2:-${GH_REPO:-}}"
  local cmd=(vcs_remote_issue_list --label "$label" --state open --json number,title,url --jq '.[] | "\(.number)|\(.title)|\(.url)"')

  [[ -n "$label" ]] || return 0

  if [[ -n "$repo_name" ]]; then
    cmd=(vcs_remote_issue_list -R "$repo_name" --label "$label" --state open --json number,title,url --jq '.[] | "\(.number)|\(.title)|\(.url)"')
    "${cmd[@]}" 2>/dev/null || true
    return 0
  fi

  "${cmd[@]}" 2>/dev/null || true
}

github_issue_upsert_marker_comment() {
  local repo_name="${1:-}"
  local issue_number="${2:-}"
  local marker="${3:-}"
  local body="${4:-}"
  local announce="${5:-false}"

  local comment_id
  local action_verb
  comment_id="$(github_issue_find_latest_marker_comment_id "$repo_name" "$issue_number" "$marker")"

  if [[ -n "$comment_id" ]]; then
    vcs_remote_api -X PATCH "repos/${repo_name}/issues/comments/${comment_id}" \
      -f body="$body" >/dev/null
    action_verb="Updated"
  else
    vcs_remote_api "repos/${repo_name}/issues/${issue_number}/comments" \
      -f body="$body" >/dev/null
    action_verb="Posted"
  fi

  if [[ "$announce" == "true" ]]; then
    echo "${action_verb} parent status comment on #${issue_number}."
  fi
}

github_issue_find_latest_marker_comment_id() {
  local repo_name="${1:-}"
  local issue_number="${2:-}"
  local marker="${3:-}"
  {
    vcs_remote_api "repos/${repo_name}/issues/${issue_number}/comments" --paginate
  } | jq -r --arg marker "$marker" '
      map(select((.body // "") | contains($marker)))
      | sort_by(.updated_at)
      | last
      | .id // empty
    ' 2>/dev/null || true
}
