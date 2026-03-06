#!/usr/bin/env bash

manager_parse_read_args() {
  local issue_var_name="$1"
  local repo_var_name="$2"
  local json_var_name="$3"
  local jq_var_name="$4"
  local template_var_name="$5"
  shift 5

  local -n issue_ref="$issue_var_name"
  local -n repo_ref="$repo_var_name"
  local -n json_ref="$json_var_name"
  local -n jq_ref="$jq_var_name"
  local -n template_ref="$template_var_name"

  issue_ref=""
  repo_ref=""
  json_ref=""
  jq_ref=""
  template_ref=""

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
    --json)
      json_ref="${2:-}"
      shift 2
      ;;
    --jq)
      jq_ref="${2:-}"
      shift 2
      ;;
    --template)
      template_ref="${2:-}"
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

manager_append_read_optional_args() {
  local -n cmd_ref="$1"
  local repo="$2"
  local json_fields="$3"
  local jq_filter="$4"
  local template="$5"

  if [[ -n "$repo" ]]; then
    cmd_ref+=(-R "$repo")
  fi
  if [[ -n "$json_fields" ]]; then
    cmd_ref+=(--json "$json_fields")
  fi
  if [[ -n "$jq_filter" ]]; then
    cmd_ref+=(--jq "$jq_filter")
  fi
  if [[ -n "$template" ]]; then
    cmd_ref+=(--template "$template")
  fi
}

cmd_read() {
  local issue_number=""
  local repo=""
  local json_fields=""
  local jq_filter=""
  local template=""

  manager_parse_read_args issue_number repo json_fields jq_filter template "$@"

  local -a cmd
  if [[ -n "$issue_number" ]]; then
    ensure_number "--issue" "$issue_number"
    cmd=(gh issue view "$issue_number")
  else
    cmd=(gh issue list)
  fi

  manager_append_read_optional_args cmd "$repo" "$json_fields" "$jq_filter" "$template"

  "${cmd[@]}"
}
