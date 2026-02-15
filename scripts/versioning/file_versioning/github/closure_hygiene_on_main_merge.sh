#!/usr/bin/env bash

set -euo pipefail

if ! command -v gh >/dev/null 2>&1; then
  echo "Error: gh is required." >&2
  exit 3
fi
if ! command -v jq >/dev/null 2>&1; then
  echo "Error: jq is required." >&2
  exit 3
fi

REPO_NAME="${GH_REPO:-}"
if [[ -z "$REPO_NAME" ]]; then
  REPO_NAME="$(gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true)"
fi
if [[ -z "$REPO_NAME" ]]; then
  echo "Error: unable to resolve repository name." >&2
  exit 3
fi

extract_tasklist_issue_refs() {
  local body="$1"
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

build_status_comment() {
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

upsert_status_comment() {
  local issue_number="$1"
  local body="$2"
  local marker="<!-- parent-issue-status:${issue_number} -->"

  local comment_id
  comment_id="$({
    gh api "repos/${REPO_NAME}/issues/${issue_number}/comments" --paginate
  } | jq -r --arg marker "$marker" '
      map(select((.body // "") | contains($marker)))
      | sort_by(.updated_at)
      | last
      | .id // empty
    ' 2>/dev/null || true)"

  if [[ -n "$comment_id" ]]; then
    gh api -X PATCH "repos/${REPO_NAME}/issues/comments/${comment_id}" \
      -f body="$body" >/dev/null
  else
    gh api "repos/${REPO_NAME}/issues/${issue_number}/comments" \
      -f body="$body" >/dev/null
  fi
}

evaluate_and_close_parent_if_ready() {
  local parent_number="$1"
  local parent_json
  local parent_state
  local parent_body
  local total
  local closed_count=0
  local open_count=0
  local open_lines=""

  parent_json="$(gh issue view "$parent_number" -R "$REPO_NAME" --json number,body,state 2>/dev/null || true)"
  if [[ -z "$parent_json" ]]; then
    return 0
  fi

  parent_state="$(echo "$parent_json" | jq -r '.state')"
  parent_body="$(echo "$parent_json" | jq -r '.body // ""')"

  mapfile -t child_refs < <(extract_tasklist_issue_refs "$parent_body")
  if [[ ${#child_refs[@]} -eq 0 ]]; then
    return 0
  fi

  total="${#child_refs[@]}"

  for child_ref in "${child_refs[@]}"; do
    local child_number="${child_ref//#/}"
    local child_json
    local child_state
    local child_title

    child_json="$(gh issue view "$child_number" -R "$REPO_NAME" --json state,title 2>/dev/null || true)"
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

  upsert_status_comment "$parent_number" "$(build_status_comment "$parent_number" "$total" "$closed_count" "$open_count" "$open_lines")"

  if [[ "$parent_state" == "OPEN" && "$open_count" -eq 0 ]]; then
    gh issue close "$parent_number" -R "$REPO_NAME" \
      --comment "All required child issues are closed. Auto-closed by closure hygiene workflow after merge into main." >/dev/null
    echo "Closed parent issue #${parent_number}."
  fi
}

close_completed_milestones() {
  local milestone
  local milestone_number
  local milestone_title
  local open_issues

  mapfile -t milestones < <(
    gh api "repos/${REPO_NAME}/milestones?state=open" --paginate \
      | jq -r '.[] | @base64'
  )

  for milestone in "${milestones[@]}"; do
    milestone_number="$(echo "$milestone" | base64 -d | jq -r '.number')"
    milestone_title="$(echo "$milestone" | base64 -d | jq -r '.title')"
    open_issues="$(echo "$milestone" | base64 -d | jq -r '.open_issues')"
    if [[ "$open_issues" == "0" ]]; then
      gh api -X PATCH "repos/${REPO_NAME}/milestones/${milestone_number}" \
        -f state=closed >/dev/null
      echo "Closed milestone #${milestone_number} (${milestone_title})."
    fi
  done
}

scan_open_parents() {
  local issue_number

  mapfile -t open_issue_numbers < <(
    gh issue list -R "$REPO_NAME" --state open --limit 300 --json number \
      | jq -r '.[].number'
  )

  for issue_number in "${open_issue_numbers[@]}"; do
    evaluate_and_close_parent_if_ready "$issue_number"
  done
}

scan_open_parents
close_completed_milestones

echo "Closure hygiene completed."
