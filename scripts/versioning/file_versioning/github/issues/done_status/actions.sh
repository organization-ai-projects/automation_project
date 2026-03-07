#!/usr/bin/env bash

done_status_on_dev_merge() {
  local repo_name="$1"
  local pr_number="$2"
  local label_name="$3"
  local label_available="$4"

  issue_cli_require_positive_number "--pr" "$pr_number"

  local pr_state
  pr_state="$(gh pr view "$pr_number" -R "$repo_name" --json state -q '.state // ""' 2>/dev/null || true)"
  if [[ "$pr_state" != "MERGED" ]]; then
    echo "PR #${pr_number} is not merged; nothing to do."
    return 0
  fi

  local payload
  payload="$(issue_gh_collect_pr_text_payload "$repo_name" "$pr_number")"
  local -a closing_issue_numbers
  mapfile -t closing_issue_numbers < <(issue_refs_extract_closing_numbers "$payload")

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
    state="$(issue_gh_issue_state "$repo_name" "$n")"
    if [[ -z "$state" ]]; then
      echo "Issue #${n}: unreadable; skipping."
      continue
    fi
    if [[ "$state" != "OPEN" ]]; then
      echo "Issue #${n}: state=${state}; skipping done-in-dev label."
      continue
    fi
    if issue_gh_issue_has_label "$repo_name" "$n" "$label_name"; then
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

  issue_cli_require_positive_number "--issue" "$issue_number"

  if [[ "$label_available" != "true" ]]; then
    echo "Warning: label '${label_name}' does not exist in ${repo_name}; skipping."
    return 0
  fi

  if issue_gh_issue_has_label "$repo_name" "$issue_number" "$label_name"; then
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
  local arg value

  while [[ $# -gt 0 ]]; do
    arg="$1"
    case "$arg" in
    --on-dev-merge)
      mode="dev-merge"
      shift
      ;;
    --on-issue-closed)
      mode="issue-closed"
      shift
      ;;
    --pr)
      value="${2:-}"
      issue_cli_require_option_value_or_usage "$arg" "$value" done_status_usage || exit 2
      pr_number="$value"
      shift 2
      ;;
    --issue)
      value="${2:-}"
      issue_cli_require_option_value_or_usage "$arg" "$value" done_status_usage || exit 2
      issue_number="$value"
      shift 2
      ;;
    --label)
      value="${2:-}"
      issue_cli_require_option_value_or_usage "$arg" "$value" done_status_usage || exit 2
      label_name="$value"
      shift 2
      ;;
    -h | --help)
      done_status_usage
      exit 0
      ;;
    *)
      issue_cli_unknown_option_with_usage "$1" done_status_usage
      exit 2
      ;;
    esac
  done

  if [[ "$mode" != "dev-merge" && "$mode" != "issue-closed" ]]; then
    echo "Error: one mode is required (--on-dev-merge or --on-issue-closed)." >&2
    done_status_usage >&2
    exit 2
  fi

  issue_gh_require_cmd gh
  issue_gh_require_cmd jq

  local repo_name
  repo_name="$(issue_gh_resolve_repo_name)"
  if [[ -z "$repo_name" ]]; then
    echo "Error: unable to resolve repository name." >&2
    exit 3
  fi

  local label_available="false"
  if issue_gh_label_exists "$repo_name" "$label_name"; then
    label_available="true"
  fi

  if [[ "$mode" == "dev-merge" ]]; then
    done_status_on_dev_merge "$repo_name" "$pr_number" "$label_name" "$label_available"
  else
    done_status_on_issue_closed "$repo_name" "$issue_number" "$label_name" "$label_available"
  fi
}
