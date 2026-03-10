#!/usr/bin/env bash

# Shared GitHub issue helpers for shell automation scripts.

issue_helpers_va_exec() {
  if command -v va_exec >/dev/null 2>&1; then
    va_exec "$@"
    return $?
  fi
  if command -v va >/dev/null 2>&1; then
    va "$@"
    return $?
  fi
  if command -v versioning_automation >/dev/null 2>&1; then
    versioning_automation "$@"
    return $?
  fi
  return 127
}

issue_helpers_has_va_issue() {
  if [[ "${issue_helpers_va_issue_checked:-0}" != "1" ]]; then
    issue_helpers_va_issue_checked="1"
    if issue_helpers_va_exec issue help >/dev/null 2>&1; then
      issue_helpers_va_issue_available="1"
    else
      issue_helpers_va_issue_available="0"
    fi
  fi

  [[ "${issue_helpers_va_issue_available:-0}" == "1" ]]
}

github_issue_extract_tasklist_refs() {
  local body="${1:-}"
  local va_output=""

  if issue_helpers_has_va_issue; then
    va_output="$(
      issue_helpers_va_exec issue tasklist-refs --body "$body" 2>/dev/null || true
    )"
  fi
  if [[ -n "$va_output" ]]; then
    printf '%s\n' "$va_output"
    return 0
  fi

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
  local va_output=""

  if issue_helpers_has_va_issue; then
    va_output="$(
      issue_helpers_va_exec issue subissue-refs \
        --owner "$repo_owner" \
        --repo "$repo_short_name" \
        --issue "$parent_number" 2>/dev/null || true
    )"
  fi
  if [[ -n "$va_output" ]]; then
    printf '%s\n' "$va_output"
    return 0
  fi

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

  if issue_helpers_has_va_issue; then
    if issue_helpers_va_exec issue upsert-marker-comment \
      --repo "$repo_name" \
      --issue "$issue_number" \
      --marker "$marker" \
      --body "$body" \
      --announce "$announce"; then
      return 0
    fi
  fi

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

github_issue_list_open_by_label() {
  local repo_name="${1:-}"
  local label_name="${2:-}"
  local va_output=""

  if [[ -z "$label_name" ]]; then
    return 0
  fi

  if issue_helpers_has_va_issue && command -v jq >/dev/null 2>&1; then
    local -a va_cmd=(issue read --json number,title,url,labels)
    if [[ -n "$repo_name" ]]; then
      va_cmd+=(--repo "$repo_name")
    fi
    va_output="$(
      issue_helpers_va_exec "${va_cmd[@]}" 2>/dev/null \
        | jq -r --arg label "$label_name" '
            .[]
            | select((.labels // []) | map(.name) | index($label))
            | "\(.number)|\(.title)|\(.url)"
          ' 2>/dev/null || true
    )"
  fi

  if [[ -n "$va_output" ]]; then
    printf '%s\n' "$va_output"
    return 0
  fi

  local -a gh_cmd=(gh issue list --label "$label_name" --state open --json number,title,labels,url --jq '.[] | "\(.number)|\(.title)|\(.url)"')
  if [[ -n "$repo_name" ]]; then
    gh_cmd+=(-R "$repo_name")
  fi
  "${gh_cmd[@]}" 2>/dev/null || true
}
