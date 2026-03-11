#!/usr/bin/env bash
# shellcheck shell=bash

closure_hygiene_build_status_comment() {
  local parent_number="$1"
  local total="$2"
  local closed_count="$3"
  local open_count="$4"
  local open_lines="$5"
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
    echo "All required child issues are closed. Parent is closure-ready."
  else
    echo "Some required child issues are still open:"
    echo "$open_lines"
  fi
}

closure_hygiene_close_issue_with_comment() {
  local repo_name="$1"
  local issue_number="$2"
  local comment="$3"
  github_issue_close_completed_with_comment "$repo_name" "$issue_number" "$comment"
}

closure_hygiene_evaluate_parent() {
  local parent_number="$1"
  local parent_state
  local parent_body
  local total
  local closed_count=0
  local open_count=0
  local open_lines=""
  local child_refs child_ref child_number child_state child_title

  parent_state="$(github_issue_state "$REPO_NAME" "$parent_number" || true)"
  parent_body="$(github_issue_field "$REPO_NAME" "$parent_number" "body" || true)"
  if [[ -z "$parent_state" && -z "$parent_body" ]]; then
    return 0
  fi

  mapfile -t child_refs < <(github_issue_extract_subissue_refs "$REPO_OWNER" "$REPO_SHORT_NAME" "$parent_number")
  if [[ ${#child_refs[@]} -eq 0 ]]; then
    mapfile -t child_refs < <(github_issue_extract_tasklist_refs "$parent_body")
  fi
  if [[ ${#child_refs[@]} -eq 0 ]]; then
    return 0
  fi

  total="${#child_refs[@]}"
  for child_ref in "${child_refs[@]}"; do
    child_number="${child_ref//#/}"
    child_state="$(github_issue_state "$REPO_NAME" "$child_number" || true)"
    child_title="$(github_issue_field "$REPO_NAME" "$child_number" "title" || true)"
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

  github_issue_upsert_marker_comment \
    "$REPO_NAME" \
    "$parent_number" \
    "<!-- parent-issue-status:${parent_number} -->" \
    "$(closure_hygiene_build_status_comment "$parent_number" "$total" "$closed_count" "$open_count" "$open_lines")"

  if [[ "$parent_state" == "OPEN" && "$open_count" -eq 0 ]]; then
    closure_hygiene_close_issue_with_comment \
      "$REPO_NAME" \
      "$parent_number" \
      "All required child issues are closed. Auto-closed by closure hygiene workflow after merge into main."
    echo "Closed parent issue #${parent_number}."
  fi
}

closure_hygiene_scan_open_parents() {
  local issue_number
  local open_issue_numbers

  mapfile -t open_issue_numbers < <(github_issue_list_open_numbers "$REPO_NAME")

  for issue_number in "${open_issue_numbers[@]}"; do
    closure_hygiene_evaluate_parent "$issue_number"
  done
}
