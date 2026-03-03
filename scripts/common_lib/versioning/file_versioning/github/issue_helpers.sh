#!/usr/bin/env bash

# Shared GitHub issue helpers for shell automation scripts.

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

  gh api graphql \
    -f query='query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){issue(number:$number){subIssues(first:100){nodes{number}}}}}' \
    -f owner="$repo_owner" \
    -f name="$repo_short_name" \
    -F number="$parent_number" \
    --jq '.data.repository.issue.subIssues.nodes[]?.number | "#"+tostring' 2>/dev/null || true
}

github_issue_upsert_marker_comment() {
  local repo_name="${1:-}"
  local issue_number="${2:-}"
  local marker="${3:-}"
  local body="${4:-}"
  local announce="${5:-false}"

  local comment_id
  comment_id="$({
    gh api "repos/${repo_name}/issues/${issue_number}/comments" --paginate
  } | jq -r --arg marker "$marker" '
      map(select((.body // "") | contains($marker)))
      | sort_by(.updated_at)
      | last
      | .id // empty
    ' 2>/dev/null || true)"

  if [[ -n "$comment_id" ]]; then
    gh api -X PATCH "repos/${repo_name}/issues/comments/${comment_id}" \
      -f body="$body" >/dev/null
    if [[ "$announce" == "true" ]]; then
      echo "Updated parent status comment on #${issue_number}."
    fi
  else
    gh api "repos/${repo_name}/issues/${issue_number}/comments" \
      -f body="$body" >/dev/null
    if [[ "$announce" == "true" ]]; then
      echo "Posted parent status comment on #${issue_number}."
    fi
  fi
}
