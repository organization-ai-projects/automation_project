#!/usr/bin/env bash

parent_guard_build_status_comment() {
  local strict_guard="$1"
  local parent_number="$2"
  local parent_state="$3"
  local total="$4"
  local closed_count="$5"
  local open_count="$6"
  local open_lines="$7"
  local marker="<!-- parent-issue-status:${parent_number} -->"

  echo "$marker"
  echo "### Parent Issue Status"
  echo "Parent: #${parent_number}"
  echo ""
  echo "- Required children detected: ${total}"
  echo "- Closed: ${closed_count}"
  echo "- Open: ${open_count}"
  echo ""

  if [[ "$open_count" -eq 0 ]]; then
    echo "All required child issues are closed. This parent can be closed."
  else
    echo "Some required child issues are still open:"
    echo "$open_lines"
    if [[ "$parent_state" == "CLOSED" && "$strict_guard" == "true" ]]; then
      echo ""
      echo "Guard action: parent was reopened because required children are still open."
    fi
  fi
}

parent_guard_issue_json() {
  local repo_name="$1"
  local issue_number="$2"
  local json_fields="$3"
  local issue_json=""

  if command -v va_exec >/dev/null 2>&1; then
    issue_json="$(
      va_exec issue read \
        --issue "$issue_number" \
        --repo "$repo_name" \
        --json "$json_fields" 2>/dev/null || true
    )"
  fi

  if [[ -z "$issue_json" ]]; then
    issue_json="$(gh issue view "$issue_number" -R "$repo_name" --json "$json_fields" 2>/dev/null || true)"
  fi

  printf '%s\n' "$issue_json"
}

parent_guard_evaluate_parent_issue() {
  local strict_guard="$1"
  local repo_name="$2"
  local repo_owner="$3"
  local repo_short_name="$4"
  local parent_number="$5"

  local issue_json
  issue_json="$(parent_guard_issue_json "$repo_name" "$parent_number" "number,title,body,state,url")"
  if [[ -z "$issue_json" ]]; then
    return 0
  fi

  local parent_state
  parent_state="$(echo "$issue_json" | jq -r '.state')"

  local body
  body="$(echo "$issue_json" | jq -r '.body // ""')"

  mapfile -t child_refs < <(github_issue_extract_subissue_refs "$repo_owner" "$repo_short_name" "$parent_number")
  if [[ ${#child_refs[@]} -eq 0 ]]; then
    mapfile -t child_refs < <(github_issue_extract_tasklist_refs "$body")
  fi
  if [[ ${#child_refs[@]} -eq 0 ]]; then
    return 0
  fi

  local total="${#child_refs[@]}"
  local closed_count=0
  local open_count=0
  local open_lines=""

  local child_ref
  for child_ref in "${child_refs[@]}"; do
    local child_number="${child_ref//#/}"
    local child_json
    child_json="$(parent_guard_issue_json "$repo_name" "$child_number" "number,title,state,url")"
    if [[ -z "$child_json" ]]; then
      open_count=$((open_count + 1))
      open_lines+="- ${child_ref} (unreadable or missing)"$'\n'
      continue
    fi

    local child_state child_title
    child_state="$(echo "$child_json" | jq -r '.state')"
    child_title="$(echo "$child_json" | jq -r '.title')"

    if [[ "$child_state" == "CLOSED" ]]; then
      closed_count=$((closed_count + 1))
    else
      open_count=$((open_count + 1))
      open_lines+="- ${child_ref} ${child_title}"$'\n'
    fi
  done

  local comment_body
  comment_body="$(parent_guard_build_status_comment "$strict_guard" "$parent_number" "$parent_state" "$total" "$closed_count" "$open_count" "$open_lines")"
  github_issue_upsert_marker_comment \
    "$repo_name" \
    "$parent_number" \
    "<!-- parent-issue-status:${parent_number} -->" \
    "$comment_body" \
    "true"

  if [[ "$open_count" -eq 0 && "$parent_state" == "OPEN" ]]; then
    gh issue close "$parent_number" -R "$repo_name" \
      --comment "All required child issues are closed. Auto-closed by parent-issue-guard." >/dev/null
    echo "Closed parent issue #${parent_number} because all required children are closed."
  fi

  if [[ "$strict_guard" == "true" && "$parent_state" == "CLOSED" && "$open_count" -gt 0 ]]; then
    gh issue reopen "$parent_number" -R "$repo_name" >/dev/null
    echo "Reopened parent issue #${parent_number} due to open required children."
  fi
}
