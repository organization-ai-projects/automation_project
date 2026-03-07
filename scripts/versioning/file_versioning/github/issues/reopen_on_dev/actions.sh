#!/usr/bin/env bash

reopen_on_dev_run() {
  local pr_number=""
  local label_name="${DONE_IN_DEV_LABEL:-done-in-dev}"
  local arg value

  while [[ $# -gt 0 ]]; do
    arg="$1"
    case "$arg" in
    --pr)
      value="${2:-}"
      issue_cli_require_option_value_or_usage "$arg" "$value" reopen_on_dev_usage || exit 2
      pr_number="$value"
      shift 2
      ;;
    --label)
      value="${2:-}"
      issue_cli_require_option_value_or_usage "$arg" "$value" reopen_on_dev_usage || exit 2
      label_name="$value"
      shift 2
      ;;
    -h | --help)
      reopen_on_dev_usage
      exit 0
      ;;
    *)
      issue_cli_unknown_option_with_usage "$1" reopen_on_dev_usage
      exit 2
      ;;
    esac
  done

  issue_cli_require_positive_number "--pr" "$pr_number"
  issue_gh_require_cmd gh
  issue_gh_require_cmd jq

  local repo_name
  repo_name="$(issue_gh_resolve_repo_name)"
  if [[ -z "$repo_name" ]]; then
    echo "Error: unable to resolve repository name." >&2
    exit 3
  fi

  local pr_state
  pr_state="$(gh pr view "$pr_number" -R "$repo_name" --json state -q '.state // ""' 2>/dev/null || true)"
  if [[ "$pr_state" != "MERGED" ]]; then
    echo "PR #${pr_number} is not merged; nothing to do."
    exit 0
  fi

  local payload
  payload="$(issue_gh_collect_pr_text_payload "$repo_name" "$pr_number")"
  local -a reopen_issue_numbers
  mapfile -t reopen_issue_numbers < <(issue_refs_extract_reopen_numbers "$payload")
  if [[ ${#reopen_issue_numbers[@]} -eq 0 ]]; then
    echo "No reopen issue refs found for PR #${pr_number}."
    exit 0
  fi

  local label_available="false"
  if issue_gh_label_exists "$repo_name" "$label_name"; then
    label_available="true"
  fi

  local n state
  for n in "${reopen_issue_numbers[@]}"; do
    state="$(issue_gh_issue_state "$repo_name" "$n")"
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

    if [[ "$label_available" == "true" ]] && issue_gh_issue_has_label "$repo_name" "$n" "$label_name"; then
      gh issue edit "$n" -R "$repo_name" --remove-label "$label_name" >/dev/null
      echo "Issue #${n}: removed label '${label_name}' due to Reopen ref."
    fi

    reopen_on_dev_sync_issue_project_status "$repo_name" "$n"
  done
}
