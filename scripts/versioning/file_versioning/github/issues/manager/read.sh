#!/usr/bin/env bash

manager_parse_read_args() {
  local issue_var_name="$1"
  local repo_var_name="$2"
  local json_var_name="$3"
  local jq_var_name="$4"
  local template_var_name="$5"
  shift 5

  printf -v "$issue_var_name" '%s' ""
  printf -v "$repo_var_name" '%s' ""
  printf -v "$json_var_name" '%s' ""
  printf -v "$jq_var_name" '%s' ""
  printf -v "$template_var_name" '%s' ""

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
    --json)
      issue_cli_assign_value_or_error "$1" "${2:-}" "$json_var_name" die_usage
      shift 2
      ;;
    --jq)
      issue_cli_assign_value_or_error "$1" "${2:-}" "$jq_var_name" die_usage
      shift 2
      ;;
    --template)
      issue_cli_assign_value_or_error "$1" "${2:-}" "$template_var_name" die_usage
      shift 2
      ;;
    -h | --help)
      usage
      exit 0
      ;;
    *)
      die_usage "Unknown option for read: $1"
      ;;
    esac
  done
}

cmd_read() {
  local issue_number=""
  local repo=""
  local json_fields=""
  local jq_filter=""
  local template=""

  manager_parse_read_args issue_number repo json_fields jq_filter template "$@"

  if [[ -n "$issue_number" ]]; then
    issue_cli_require_positive_number "--issue" "$issue_number"
  fi

  if va_exec issue read "$@"; then
    return 0
  fi
  echo "Warning: Rust issue read path failed; falling back to shell read path." >&2

  issue_gh_issue_read "$issue_number" "$repo" "$json_fields" "$jq_filter" "$template"
}
