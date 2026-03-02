#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/versioning/file_versioning/github/lib/issue_refs.sh
source "${SCRIPT_DIR}/lib/issue_refs.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/github/directive_resolution.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/versioning/file_versioning/github/directive_resolution.sh"

usage() {
  cat <<EOF
Usage:
  $0 --pr PR_NUMBER [--label LABEL]
EOF
}

require_number() {
  local name="$1"
  local value="$2"
  if [[ ! "$value" =~ ^[0-9]+$ ]]; then
    echo "Error: ${name} must be a positive integer." >&2
    exit 2
  fi
}

require_cmd() {
  local cmd="$1"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Error: command '${cmd}' is required." >&2
    exit 3
  fi
}

sync_issue_project_status_on_reopen() {
  local repo="$1"
  local issue_number="$2"
  local target_status="${PROJECT_STATUS_REOPEN_NAME:-Todo}"
  local owner
  local name
  local issue_json
  local issue_id
  local items_tsv

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
  done <<< "$items_tsv"
}

collect_pr_context() {
  local repo="$1"
  local pr_number="$2"

  PR_TITLE="$(gh pr view "$pr_number" -R "$repo" --json title -q '.title // ""' 2>/dev/null || true)"
  PR_BODY="$(gh pr view "$pr_number" -R "$repo" --json body -q '.body // ""' 2>/dev/null || true)"
  PR_COMMITS="$(gh api "repos/${repo}/pulls/${pr_number}/commits" --paginate --jq '.[].commit.message' 2>/dev/null || true)"
}

pr_number=""
label_name="${DONE_IN_DEV_LABEL:-done-in-dev}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --pr)
      pr_number="${2:-}"
      shift 2
      ;;
    --label)
      label_name="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Error: unknown argument '$1'." >&2
      usage >&2
      exit 2
      ;;
  esac
done

require_number "--pr" "$pr_number"
require_cmd gh
require_cmd jq

repo_name="$(gh_resolve_repo_name)"
if [[ -z "$repo_name" ]]; then
  echo "Error: unable to resolve repository name." >&2
  exit 3
fi

pr_state="$(gh pr view "$pr_number" -R "$repo_name" --json state -q '.state // ""' 2>/dev/null || true)"
if [[ "$pr_state" != "MERGED" ]]; then
  echo "PR #${pr_number} is not merged; nothing to do."
  exit 0
fi

collect_pr_context "$repo_name" "$pr_number"
payload="${PR_TITLE}"$'\n'"${PR_BODY}"$'\n'"${PR_COMMITS}"

declare -a reopen_issue_numbers=()
while IFS='|' read -r issue_key close_flag reopen_flag decision source reason; do
  [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue
  [[ "$reopen_flag" == "1" ]] || continue

  if [[ "$source" == "unresolved" ]]; then
    echo "Issue ${issue_key}: reopen skipped (${reason})."
    continue
  fi

  if [[ "$decision" == "reopen" ]]; then
    reopen_issue_numbers+=("${issue_key//#/}")
    continue
  fi

  if [[ "$close_flag" == "1" ]]; then
    echo "Issue ${issue_key}: reopen skipped (resolved decision=${decision:-close})."
  fi
done < <(resolve_issue_directives "$payload" "$PR_BODY" "$PR_COMMITS")

if [[ ${#reopen_issue_numbers[@]} -eq 0 ]]; then
  echo "No reopen issue refs found for PR #${pr_number}."
  exit 0
fi

label_available="false"
if gh_label_exists "$repo_name" "$label_name"; then
  label_available="true"
fi

for n in "${reopen_issue_numbers[@]}"; do
  state="$(gh_issue_state "$repo_name" "$n")"
  if [[ -z "$state" ]]; then
    echo "Issue #${n}: unreadable; skipping reopen sync."
    continue
  fi

  if [[ "$state" == "CLOSED" ]]; then
    gh issue reopen "$n" -R "$repo_name" >/dev/null
    echo "Issue #${n}: reopened from Reopen ref."
  else
    echo "Issue #${n}: state=${state}; no reopen needed."
  fi

  if [[ "$label_available" == "true" ]] && gh_issue_has_label "$repo_name" "$n" "$label_name"; then
    gh issue edit "$n" -R "$repo_name" --remove-label "$label_name" >/dev/null
    echo "Issue #${n}: removed label '${label_name}' due to Reopen ref."
  fi

  sync_issue_project_status_on_reopen "$repo_name" "$n"
done
