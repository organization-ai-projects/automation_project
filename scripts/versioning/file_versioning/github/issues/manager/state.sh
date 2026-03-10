#!/usr/bin/env bash

manager_parse_issue_repo_args() {
  local command_name="$1"
  local issue_var_name="$2"
  local repo_var_name="$3"
  shift 3
  printf -v "$issue_var_name" '%s' ""
  printf -v "$repo_var_name" '%s' ""

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --issue)
      issue_cli_assign_value_or_error "$1" "${2:-}" "$issue_var_name" die_usage
      shift 2
      ;;
    --repo)
      issue_cli_assign_value_or_error "$1" "${2:-}" "$repo_var_name" die_usage
      shift 2
      ;;
    -h | --help)
      usage
      exit 0
      ;;
    *)
      die_usage "Unknown option for ${command_name}: $1"
      ;;
    esac
  done
}

cmd_close() {
  local issue_number=""
  local repo=""
  local reason="completed"

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --issue)
      issue_cli_assign_value_or_error "$1" "${2:-}" issue_number die_usage
      shift 2
      ;;
    --repo)
      issue_cli_assign_value_or_error "$1" "${2:-}" repo die_usage
      shift 2
      ;;
    --reason)
      issue_cli_assign_value_or_error "$1" "${2:-}" reason die_usage
      shift 2
      ;;
    -h | --help)
      usage
      exit 0
      ;;
    *)
      die_usage "Unknown option for close: $1"
      ;;
    esac
  done

  issue_cli_require_positive_number "--issue" "$issue_number"
  if [[ "$reason" != "completed" && "$reason" != "not_planned" ]]; then
    die_usage "--reason must be 'completed' or 'not_planned'."
  fi

  issue_gh_issue_close "$repo" "$issue_number" "$reason"
  echo "Issue #${issue_number} closed (reason: ${reason})."
}

cmd_reopen() {
  local issue_number=""
  local repo=""

  manager_parse_issue_repo_args "reopen" issue_number repo "$@"

  issue_cli_require_positive_number "--issue" "$issue_number"
  issue_gh_issue_reopen "$repo" "$issue_number"
  echo "Issue #${issue_number} reopened."
}

cmd_delete() {
  local issue_number=""
  local repo=""

  manager_parse_issue_repo_args "delete" issue_number repo "$@"

  issue_cli_require_positive_number "--issue" "$issue_number"
  issue_gh_issue_close "$repo" "$issue_number" "not_planned"
  echo "Issue #${issue_number} soft-deleted (closed with reason: not_planned)."
}
