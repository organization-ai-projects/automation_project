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

closure_hygiene_issue_json() {
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

closure_hygiene_close_issue_with_comment() {
  local repo_name="$1"
  local issue_number="$2"
  local comment="$3"

  if command -v va_exec >/dev/null 2>&1; then
    if va_exec issue close \
      --issue "$issue_number" \
      --repo "$repo_name" \
      --reason completed \
      --comment "$comment" >/dev/null 2>&1; then
      return 0
    fi
  fi

  gh issue close "$issue_number" -R "$repo_name" --comment "$comment" >/dev/null
}

closure_hygiene_evaluate_parent() {
  local parent_number="$1"
  local parent_json
  local parent_state
  local parent_body
  local total
  local closed_count=0
  local open_count=0
  local open_lines=""
  local child_refs child_ref child_number child_json child_state child_title

  parent_json="$(closure_hygiene_issue_json "$REPO_NAME" "$parent_number" "number,body,state")"
  if [[ -z "$parent_json" ]]; then
    return 0
  fi

  parent_state="$(echo "$parent_json" | jq -r '.state')"
  parent_body="$(echo "$parent_json" | jq -r '.body // ""')"

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
    child_json="$(closure_hygiene_issue_json "$REPO_NAME" "$child_number" "state,title")"
    if [[ -z "$child_json" ]]; then
      open_count=$((open_count + 1))
      open_lines+="- ${child_ref} (unreadable or missing)"$'\n'
      continue
    fi

    child_state="$(echo "$child_json" | jq -r '.state')"
    child_title="$(echo "$child_json" | jq -r '.title')"
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

  if command -v va_exec >/dev/null 2>&1; then
    mapfile -t open_issue_numbers < <(
      va_exec issue read --repo "$REPO_NAME" --json number --jq '.[].number' 2>/dev/null || true
    )
  fi
  if [[ ${#open_issue_numbers[@]} -eq 0 ]]; then
    mapfile -t open_issue_numbers < <(
      gh issue list -R "$REPO_NAME" --state open --limit 300 --json number |
        jq -r '.[].number'
    )
  fi

  for issue_number in "${open_issue_numbers[@]}"; do
    closure_hygiene_evaluate_parent "$issue_number"
  done
}
