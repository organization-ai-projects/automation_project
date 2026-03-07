#!/usr/bin/env bash
# shellcheck shell=bash

closure_hygiene_close_completed_milestones() {
  local milestone
  local milestone_number
  local milestone_title
  local open_issues
  local milestones

  mapfile -t milestones < <(
    gh api "repos/${REPO_NAME}/milestones?state=open" --paginate |
      jq -r '.[] | @base64'
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
