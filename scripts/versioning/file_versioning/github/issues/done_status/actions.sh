#!/usr/bin/env bash

done_status_on_dev_merge() {
  local repo_name="$1"
  local pr_number="$2"
  local label_name="$3"
  local label_available="$4"

  done_status_require_number "--pr" "$pr_number"

  local pr_state
  pr_state="$(gh pr view "$pr_number" -R "$repo_name" --json state -q '.state // ""' 2>/dev/null || true)"
  if [[ "$pr_state" != "MERGED" ]]; then
    echo "PR #${pr_number} is not merged; nothing to do."
    return 0
  fi

  local payload
  payload="$(done_status_collect_pr_text_payload "$repo_name" "$pr_number")"
  local -a closing_issue_numbers
  mapfile -t closing_issue_numbers < <(done_status_extract_closing_issue_numbers "$payload")

  if [[ ${#closing_issue_numbers[@]} -eq 0 ]]; then
    echo "No closing issue refs found for PR #${pr_number}."
    return 0
  fi

  if [[ "$label_available" != "true" ]]; then
    echo "Warning: label '${label_name}' does not exist in ${repo_name}; skipping done-in-dev labeling."
    return 0
  fi

  local n state
  for n in "${closing_issue_numbers[@]}"; do
    state="$(done_status_issue_state "$repo_name" "$n")"
    if [[ -z "$state" ]]; then
      echo "Issue #${n}: unreadable; skipping."
      continue
    fi
    if [[ "$state" != "OPEN" ]]; then
      echo "Issue #${n}: state=${state}; skipping done-in-dev label."
      continue
    fi
    if done_status_issue_has_label "$repo_name" "$n" "$label_name"; then
      echo "Issue #${n}: label '${label_name}' already present."
      continue
    fi

    gh issue edit "$n" -R "$repo_name" --add-label "$label_name" >/dev/null
    echo "Issue #${n}: added label '${label_name}'."
  done
}

done_status_on_issue_closed() {
  local repo_name="$1"
  local issue_number="$2"
  local label_name="$3"
  local label_available="$4"

  done_status_require_number "--issue" "$issue_number"

  if [[ "$label_available" != "true" ]]; then
    echo "Warning: label '${label_name}' does not exist in ${repo_name}; skipping."
    return 0
  fi

  if done_status_issue_has_label "$repo_name" "$issue_number" "$label_name"; then
    gh issue edit "$issue_number" -R "$repo_name" --remove-label "$label_name" >/dev/null
    echo "Issue #${issue_number}: removed label '${label_name}'."
  else
    echo "Issue #${issue_number}: label '${label_name}' not present."
  fi
}

done_status_run() {
  local mode=""
  local pr_number=""
  local issue_number=""
  local label_name="${DONE_IN_DEV_LABEL:-done-in-dev}"

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --on-dev-merge)
      mode="dev-merge"
      shift
      ;;
    --on-issue-closed)
      mode="issue-closed"
      shift
      ;;
    --pr)
      pr_number="${2:-}"
      shift 2
      ;;
    --issue)
      issue_number="${2:-}"
      shift 2
      ;;
    --label)
      label_name="${2:-}"
      shift 2
      ;;
    -h | --help)
      done_status_usage
      exit 0
      ;;
    *)
      echo "Error: unknown argument '$1'." >&2
      done_status_usage >&2
      exit 2
      ;;
    esac
  done

  if [[ "$mode" != "dev-merge" && "$mode" != "issue-closed" ]]; then
    echo "Error: one mode is required (--on-dev-merge or --on-issue-closed)." >&2
    done_status_usage >&2
    exit 2
  fi

  done_status_require_cmd gh
  done_status_require_cmd jq

  local repo_name
  repo_name="$(done_status_resolve_repo_name)"
  if [[ -z "$repo_name" ]]; then
    echo "Error: unable to resolve repository name." >&2
    exit 3
  fi

  local label_available="false"
  if done_status_label_exists "$repo_name" "$label_name"; then
    label_available="true"
  fi

  if [[ "$mode" == "dev-merge" ]]; then
    done_status_on_dev_merge "$repo_name" "$pr_number" "$label_name" "$label_available"
  else
    done_status_on_issue_closed "$repo_name" "$issue_number" "$label_name" "$label_available"
  fi
}
