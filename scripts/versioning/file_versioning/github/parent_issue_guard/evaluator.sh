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
  github_issue_read_json "$repo_name" "$issue_number" "$json_fields"
}

parent_guard_reopen_issue() {
  local repo_name="$1"
  local issue_number="$2"
  github_issue_reopen "$repo_name" "$issue_number"
}

parent_guard_close_issue_with_comment() {
  local repo_name="$1"
  local issue_number="$2"
  local comment="$3"
  github_issue_close_completed_with_comment "$repo_name" "$issue_number" "$comment"
}

parent_guard_evaluate_parent_issue() {
  local strict_guard="$1"
  local repo_name="$2"
  local repo_owner="$3"
  local repo_short_name="$4"
  local parent_number="$5"

  local parent_state
  parent_state="$(github_issue_state "$repo_name" "$parent_number" || true)"

  local body
  body="$(github_issue_field "$repo_name" "$parent_number" "body" || true)"

  if [[ -z "$parent_state" && -z "$body" ]]; then
    return 0
  fi

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
    local child_state child_title
    child_state="$(github_issue_state "$repo_name" "$child_number" || true)"
    child_title="$(github_issue_field "$repo_name" "$child_number" "title" || true)"
    if [[ -z "$child_state" && -z "$child_title" ]]; then
      open_count=$((open_count + 1))
      open_lines+="- ${child_ref} (unreadable or missing)"$'\n'
      continue
    fi

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
    parent_guard_close_issue_with_comment \
      "$repo_name" \
      "$parent_number" \
      "All required child issues are closed. Auto-closed by parent-issue-guard."
    echo "Closed parent issue #${parent_number} because all required children are closed."
  fi

  if [[ "$strict_guard" == "true" && "$parent_state" == "CLOSED" && "$open_count" -gt 0 ]]; then
    parent_guard_reopen_issue "$repo_name" "$parent_number"
    echo "Reopened parent issue #${parent_number} due to open required children."
  fi
}
