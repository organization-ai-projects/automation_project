#!/usr/bin/env bash

manager_parse_issue_repo_args() {
  local command_name="$1"
  local issue_var_name="$2"
  local repo_var_name="$3"
  shift 3

  local -n issue_ref="$issue_var_name"
  local -n repo_ref="$repo_var_name"

  issue_ref=""
  repo_ref=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --issue)
      issue_ref="${2:-}"
      shift 2
      ;;
    --repo)
      repo_ref="${2:-}"
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

manager_append_repo_arg() {
  local -n cmd_ref="$1"
  local repo="$2"
  if [[ -n "$repo" ]]; then
    cmd_ref+=(-R "$repo")
  fi
}

cmd_close() {
  local issue_number=""
  local repo=""
  local reason="completed"

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --issue)
      issue_number="${2:-}"
      shift 2
      ;;
    --repo)
      repo="${2:-}"
      shift 2
      ;;
    --reason)
      reason="${2:-}"
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

  ensure_number "--issue" "$issue_number"
  if [[ "$reason" != "completed" && "$reason" != "not_planned" ]]; then
    die_usage "--reason must be 'completed' or 'not_planned'."
  fi

  local -a cmd=(gh issue close "$issue_number" --reason "$reason")
  manager_append_repo_arg cmd "$repo"
  "${cmd[@]}" >/dev/null
  echo "Issue #${issue_number} closed (reason: ${reason})."
}

cmd_reopen() {
  local issue_number=""
  local repo=""

  manager_parse_issue_repo_args "reopen" issue_number repo "$@"

  ensure_number "--issue" "$issue_number"
  local -a cmd=(gh issue reopen "$issue_number")
  manager_append_repo_arg cmd "$repo"
  "${cmd[@]}" >/dev/null
  echo "Issue #${issue_number} reopened."
}

cmd_delete() {
  local issue_number=""
  local repo=""

  manager_parse_issue_repo_args "delete" issue_number repo "$@"

  ensure_number "--issue" "$issue_number"
  local -a cmd=(gh issue close "$issue_number" --reason not_planned)
  manager_append_repo_arg cmd "$repo"
  "${cmd[@]}" >/dev/null
  echo "Issue #${issue_number} soft-deleted (closed with reason: not_planned)."
}
