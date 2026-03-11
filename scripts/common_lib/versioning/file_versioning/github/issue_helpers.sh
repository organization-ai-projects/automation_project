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

issue_helpers_normalize_json_fields() {
  local json_fields="${1:-}"

  printf '%s' "$json_fields" |
    tr -d '[:space:]' |
    tr ',' '\n' |
    sed '/^$/d' |
    sort -u |
    paste -sd, -
}

github_issue_repo_name() {
  local va_output=""

  if [[ -n "${GH_REPO:-}" ]]; then
    printf '%s\n' "$GH_REPO"
    return 0
  fi

  if issue_helpers_has_va_issue; then
    va_output="$(issue_helpers_va_exec issue repo-name 2>/dev/null || true)"
  fi

  if [[ -n "$va_output" ]]; then
    printf '%s\n' "$va_output"
    return 0
  fi

  gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true
}

github_issue_label_exists() {
  local repo_name="${1:-}"
  local label_name="${2:-}"
  local va_output=""

  if [[ -z "$repo_name" || -z "$label_name" ]]; then
    return 1
  fi

  if issue_helpers_has_va_issue; then
    va_output="$(
      issue_helpers_va_exec issue label-exists \
        --repo "$repo_name" \
        --label "$label_name" 2>/dev/null || true
    )"
  fi

  if [[ "$va_output" == "true" ]]; then
    return 0
  fi
  if [[ "$va_output" == "false" ]]; then
    return 1
  fi

  gh label list -R "$repo_name" --limit 1000 --json name --jq '.[].name' 2>/dev/null |
    grep -Fxq "$label_name"
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

  echo "$body" |
    awk '
      /-[[:space:]]*\[[xX ]\]/ {
        line = $0
        while (match(line, /#[0-9]+/)) {
          ref = substr(line, RSTART, RLENGTH)
          print ref
          line = substr(line, RSTART + RLENGTH)
        }
      }
    ' |
    sort -u
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

  if issue_helpers_has_va_issue; then
    local -a va_cmd=(issue list-by-label --label "$label_name")
    if [[ -n "$repo_name" ]]; then
      va_cmd+=(--repo "$repo_name")
    fi
    va_output="$(issue_helpers_va_exec "${va_cmd[@]}" 2>/dev/null || true)"
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

github_issue_view_title_labels() {
  local issue_number="${1:-}"
  local repo_name="${2:-}"
  local va_output=""
  local title=""
  local labels_raw=""

  if [[ -z "$issue_number" ]]; then
    return 1
  fi

  if issue_helpers_has_va_issue; then
    title="$(github_issue_field "$repo_name" "$issue_number" "title" || true)"
    labels_raw="$(github_issue_field "$repo_name" "$issue_number" "labels-raw" || true)"
    if [[ -n "$title" || -n "$labels_raw" ]]; then
      va_output="$(
        jq -c -n \
          --arg title "$title" \
          --arg labels_raw "$labels_raw" \
          '{title: $title, labels: ($labels_raw | split("||") | map(select(length > 0)) | map({name: .}))}' \
          2>/dev/null || true
      )"
    fi
  fi

  if [[ -n "$va_output" ]]; then
    printf '%s\n' "$va_output"
    return 0
  fi

  local -a gh_cmd=(gh issue view "$issue_number" --json title,labels)
  if [[ -n "$repo_name" ]]; then
    gh_cmd+=(-R "$repo_name")
  fi
  "${gh_cmd[@]}" 2>/dev/null || true
}

github_issue_read() {
  local issue_number="${1:-}"
  local repo_name="${2:-}"
  local json_fields="${3:-}"
  local jq_filter="${4:-}"
  local template="${5:-}"
  local va_output=""
  local normalized_fields=""

  normalized_fields="$(issue_helpers_normalize_json_fields "$json_fields")"
  if issue_helpers_has_va_issue &&
    [[ -n "$issue_number" ]] &&
    [[ -z "$jq_filter" ]] &&
    [[ -z "$template" ]] &&
    ([[ "$normalized_fields" == "labels,title" ]] || [[ "$normalized_fields" == "body,labels,title" ]]); then
    local title="" body="" labels_raw=""
    title="$(github_issue_field "$repo_name" "$issue_number" "title" || true)"
    labels_raw="$(github_issue_field "$repo_name" "$issue_number" "labels-raw" || true)"
    if [[ "$normalized_fields" == "body,labels,title" ]]; then
      body="$(github_issue_field "$repo_name" "$issue_number" "body" || true)"
    fi
    if [[ -n "$title" || -n "$labels_raw" || -n "$body" ]]; then
      if [[ "$normalized_fields" == "body,labels,title" ]]; then
        va_output="$(
          jq -c -n \
            --arg title "$title" \
            --arg body "$body" \
            --arg labels_raw "$labels_raw" \
            '{title: $title, body: $body, labels: ($labels_raw | split("||") | map(select(length > 0)) | map({name: .}))}' \
            2>/dev/null || true
        )"
      else
        va_output="$(
          jq -c -n \
            --arg title "$title" \
            --arg labels_raw "$labels_raw" \
            '{title: $title, labels: ($labels_raw | split("||") | map(select(length > 0)) | map({name: .}))}' \
            2>/dev/null || true
        )"
      fi
    fi
  fi

  if [[ -z "$va_output" ]] && issue_helpers_has_va_issue; then
    local -a va_cmd=(issue read)
    if [[ -n "$issue_number" ]]; then
      va_cmd+=(--issue "$issue_number")
    fi
    if [[ -n "$repo_name" ]]; then
      va_cmd+=(--repo "$repo_name")
    fi
    if [[ -n "$json_fields" ]]; then
      va_cmd+=(--json "$json_fields")
    fi
    if [[ -n "$jq_filter" ]]; then
      va_cmd+=(--jq "$jq_filter")
    fi
    if [[ -n "$template" ]]; then
      va_cmd+=(--template "$template")
    fi
    va_output="$(issue_helpers_va_exec "${va_cmd[@]}" 2>/dev/null || true)"
  fi

  if [[ -n "$va_output" ]]; then
    printf '%s\n' "$va_output"
    return 0
  fi

  local -a gh_cmd
  if [[ -n "$issue_number" ]]; then
    gh_cmd=(gh issue view "$issue_number")
  else
    gh_cmd=(gh issue list)
  fi
  if [[ -n "$repo_name" ]]; then
    gh_cmd+=(-R "$repo_name")
  fi
  if [[ -n "$json_fields" ]]; then
    gh_cmd+=(--json "$json_fields")
  fi
  if [[ -n "$jq_filter" ]]; then
    gh_cmd+=(--jq "$jq_filter")
  fi
  if [[ -n "$template" ]]; then
    gh_cmd+=(--template "$template")
  fi

  "${gh_cmd[@]}"
}

github_issue_field() {
  local repo_name="${1:-}"
  local issue_number="${2:-}"
  local field_name="${3:-}"
  local va_output=""

  if [[ -z "$issue_number" || -z "$field_name" ]]; then
    return 1
  fi

  if issue_helpers_has_va_issue; then
    local -a va_cmd=(issue field --issue "$issue_number" --name "$field_name")
    if [[ -n "$repo_name" ]]; then
      va_cmd+=(--repo "$repo_name")
    fi
    va_output="$(issue_helpers_va_exec "${va_cmd[@]}" 2>/dev/null || true)"
  fi

  if [[ -n "$va_output" ]]; then
    printf '%s\n' "$va_output"
    return 0
  fi

  local -a gh_cmd=(gh issue view "$issue_number" --json title,body,labels)
  if [[ -n "$repo_name" ]]; then
    gh_cmd+=(-R "$repo_name")
  fi
  local issue_json=""
  issue_json="$("${gh_cmd[@]}" 2>/dev/null || true)"
  if [[ -z "$issue_json" ]]; then
    gh_cmd=(gh issue view "$issue_number" --json labels,title,body)
    if [[ -n "$repo_name" ]]; then
      gh_cmd+=(-R "$repo_name")
    fi
    issue_json="$("${gh_cmd[@]}" 2>/dev/null || true)"
  fi
  if [[ -z "$issue_json" ]]; then
    return 0
  fi

  case "$field_name" in
  title)
    echo "$issue_json" | jq -r '.title // ""' 2>/dev/null || true
    ;;
  body)
    echo "$issue_json" | jq -r '.body // ""' 2>/dev/null || true
    ;;
  labels-raw)
    echo "$issue_json" | jq -r '(.labels // []) | map(.name) | join("||")' 2>/dev/null || true
    ;;
  *)
    return 1
    ;;
  esac
}

github_issue_reopen() {
  local repo_name="${1:-}"
  local issue_number="${2:-}"

  if [[ -z "$issue_number" ]]; then
    return 1
  fi

  if issue_helpers_has_va_issue; then
    local -a va_cmd=(issue reopen --issue "$issue_number")
    if [[ -n "$repo_name" ]]; then
      va_cmd+=(--repo "$repo_name")
    fi
    if issue_helpers_va_exec "${va_cmd[@]}" >/dev/null 2>&1; then
      return 0
    fi
  fi

  local -a gh_cmd=(gh issue reopen "$issue_number")
  if [[ -n "$repo_name" ]]; then
    gh_cmd+=(-R "$repo_name")
  fi
  "${gh_cmd[@]}" >/dev/null
}

github_issue_update() {
  local repo_name="${1:-}"
  local issue_number="${2:-}"
  shift 2 || true

  if [[ -z "$issue_number" ]]; then
    return 1
  fi

  if issue_helpers_has_va_issue; then
    local -a va_cmd=(issue update --issue "$issue_number")
    if [[ -n "$repo_name" ]]; then
      va_cmd+=(--repo "$repo_name")
    fi
    va_cmd+=("$@")
    if issue_helpers_va_exec "${va_cmd[@]}" >/dev/null 2>&1; then
      return 0
    fi
  fi

  local -a gh_cmd=(gh issue edit "$issue_number")
  if [[ -n "$repo_name" ]]; then
    gh_cmd+=(-R "$repo_name")
  fi
  gh_cmd+=("$@")
  "${gh_cmd[@]}" >/dev/null
}

github_issue_close() {
  local repo_name="${1:-}"
  local issue_number="${2:-}"
  local reason="${3:-completed}"
  local comment="${4:-}"

  if [[ -z "$issue_number" ]]; then
    return 1
  fi

  if issue_helpers_has_va_issue; then
    local -a va_cmd=(issue close --issue "$issue_number" --reason "$reason")
    if [[ -n "$repo_name" ]]; then
      va_cmd+=(--repo "$repo_name")
    fi
    if [[ -n "$comment" ]]; then
      va_cmd+=(--comment "$comment")
    fi
    if issue_helpers_va_exec "${va_cmd[@]}" >/dev/null 2>&1; then
      return 0
    fi
  fi

  local -a gh_cmd=(gh issue close "$issue_number" --reason "$reason")
  if [[ -n "$repo_name" ]]; then
    gh_cmd+=(-R "$repo_name")
  fi
  if [[ -n "$comment" ]]; then
    gh_cmd+=(--comment "$comment")
  fi
  "${gh_cmd[@]}" >/dev/null
}

github_issue_close_completed_with_comment() {
  local repo_name="${1:-}"
  local issue_number="${2:-}"
  local comment="${3:-}"

  if [[ -z "$issue_number" ]]; then
    return 1
  fi

  github_issue_close "$repo_name" "$issue_number" "completed" "$comment"
}

github_issue_list_open_numbers() {
  local repo_name="${1:-}"
  local va_output=""

  if issue_helpers_has_va_issue; then
    local -a va_cmd=(issue open-numbers)
    if [[ -n "$repo_name" ]]; then
      va_cmd+=(--repo "$repo_name")
    fi
    va_output="$(issue_helpers_va_exec "${va_cmd[@]}" 2>/dev/null || true)"
  fi

  if [[ -n "$va_output" ]]; then
    printf '%s\n' "$va_output"
    return 0
  fi

  local -a gh_cmd=(gh issue list --state open --limit 300 --json number --jq '.[].number')
  if [[ -n "$repo_name" ]]; then
    gh_cmd+=(-R "$repo_name")
  fi
  "${gh_cmd[@]}" 2>/dev/null || true
}

github_issue_assignee_logins() {
  local repo_name="${1:-}"
  local issue_number="${2:-}"
  local va_output=""

  if [[ -z "$issue_number" ]]; then
    return 1
  fi

  if issue_helpers_has_va_issue; then
    local -a va_cmd=(issue assignee-logins --issue "$issue_number")
    if [[ -n "$repo_name" ]]; then
      va_cmd+=(--repo "$repo_name")
    fi
    va_output="$(issue_helpers_va_exec "${va_cmd[@]}" 2>/dev/null || true)"
  fi

  if [[ -n "$va_output" ]]; then
    printf '%s\n' "$va_output"
    return 0
  fi

  local -a gh_cmd=(gh issue view "$issue_number" --json assignees --jq '.assignees[].login')
  if [[ -n "$repo_name" ]]; then
    gh_cmd+=(-R "$repo_name")
  fi
  "${gh_cmd[@]}" 2>/dev/null || true
}

github_issue_state() {
  local repo_name="${1:-}"
  local issue_number="${2:-}"
  local va_output=""
  local gh_output=""

  if [[ -z "$issue_number" ]]; then
    return 1
  fi

  if issue_helpers_has_va_issue; then
    local -a va_cmd=(issue state --issue "$issue_number")
    if [[ -n "$repo_name" ]]; then
      va_cmd+=(--repo "$repo_name")
    fi
    va_output="$(issue_helpers_va_exec "${va_cmd[@]}" 2>/dev/null || true)"
  fi

  if [[ -n "$va_output" ]]; then
    printf '%s\n' "$va_output"
    return 0
  fi

  local -a gh_cmd=(gh issue view "$issue_number" --json state --jq '.state // ""')
  if [[ -n "$repo_name" ]]; then
    gh_cmd+=(-R "$repo_name")
  fi
  gh_output="$("${gh_cmd[@]}" 2>/dev/null || true)"
  if [[ "$gh_output" == \{* ]]; then
    printf '%s\n' "$gh_output" | jq -r '.state // ""' 2>/dev/null || true
    return 0
  fi
  printf '%s\n' "$gh_output"
}

github_issue_has_label() {
  local repo_name="${1:-}"
  local issue_number="${2:-}"
  local label="${3:-}"
  local va_output=""

  if [[ -z "$issue_number" || -z "$label" ]]; then
    return 1
  fi

  if issue_helpers_has_va_issue; then
    local -a va_cmd=(issue has-label --issue "$issue_number" --label "$label")
    if [[ -n "$repo_name" ]]; then
      va_cmd+=(--repo "$repo_name")
    fi
    va_output="$(issue_helpers_va_exec "${va_cmd[@]}" 2>/dev/null || true)"
  fi

  if [[ "$va_output" == "true" ]]; then
    return 0
  fi
  if [[ "$va_output" == "false" ]]; then
    return 1
  fi

  local -a gh_cmd=(gh issue view "$issue_number" --json labels --jq '.labels[].name')
  if [[ -n "$repo_name" ]]; then
    gh_cmd+=(-R "$repo_name")
  fi
  "${gh_cmd[@]}" 2>/dev/null | grep -Fxq "$label"
}
