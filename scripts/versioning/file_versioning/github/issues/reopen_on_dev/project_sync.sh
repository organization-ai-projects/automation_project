#!/usr/bin/env bash
# shellcheck disable=SC2016

reopen_on_dev_sync_issue_project_status() {
  local repo="$1"
  local issue_number="$2"
  local target_status="${PROJECT_STATUS_REOPEN_NAME:-Todo}"
  local owner
  local name
  local issue_json
  local issue_id
  local items_tsv
  local project_json
  local status_field_id
  local status_option_id

  if va_exec issue sync-project-status \
    --repo "$repo" \
    --issue "$issue_number" \
    --status "$target_status" >/dev/null 2>&1; then
    return 0
  fi

  owner="${repo%/*}"
  name="${repo#*/}"
  if [[ -z "$owner" || -z "$name" || "$owner" == "$name" ]]; then
    return 0
  fi

  issue_json="$(gh api graphql -f query='
    query($owner:String!, $name:String!, $number:Int!) {
      repository(owner:$owner, name:$name) {
        issue(number:$number) {
          id
          projectItems(first:50) {
            nodes {
              id
              project { id title }
            }
          }
        }
      }
    }' -F owner="$owner" -F name="$name" -F number="$issue_number" 2>/dev/null || true)"

  issue_id="$(echo "$issue_json" | jq -r '.data.repository.issue.id // empty' 2>/dev/null || true)"
  [[ -n "$issue_id" ]] || return 0

  items_tsv="$(echo "$issue_json" | jq -r '.data.repository.issue.projectItems.nodes[]? | [.id, .project.id, (.project.title // "")] | @tsv' 2>/dev/null || true)"
  [[ -n "$items_tsv" ]] || return 0

  while IFS=$'\t' read -r item_id project_id project_title; do
    [[ -n "$item_id" && -n "$project_id" ]] || continue

    project_json="$(gh api graphql -f query='
      query($projectId:ID!) {
        node(id:$projectId) {
          ... on ProjectV2 {
            fields(first:100) {
              nodes {
                ... on ProjectV2SingleSelectField {
                  id
                  name
                  options { id name }
                }
              }
            }
          }
        }
      }' -F projectId="$project_id" 2>/dev/null || true)"

    status_field_id="$(echo "$project_json" | jq -r '.data.node.fields.nodes[]? | select(.name == "Status") | .id' 2>/dev/null | head -n1)"
    [[ -n "$status_field_id" ]] || continue

    status_option_id="$(echo "$project_json" | jq -r --arg target "$target_status" '.data.node.fields.nodes[]? | select(.name == "Status") | .options[]? | select((.name | ascii_downcase) == ($target | ascii_downcase)) | .id' 2>/dev/null | head -n1)"
    [[ -n "$status_option_id" ]] || continue

    gh api graphql -f query='
      mutation($project:ID!, $item:ID!, $field:ID!, $option: String!) {
        updateProjectV2ItemFieldValue(input: {
          projectId: $project
          itemId: $item
          fieldId: $field
          value: { singleSelectOptionId: $option }
        }) { projectV2Item { id } }
      }' \
      -F project="$project_id" \
      -F item="$item_id" \
      -F field="$status_field_id" \
      -F option="$status_option_id" >/dev/null 2>&1 || true

    echo "Issue #${issue_number}: synced project '${project_title}' status to '${target_status}'."
  done <<<"$items_tsv"
}
