#!/usr/bin/env bash

cmd_read() {
  local issue_number=""
  local repo=""
  local json_fields=""
  local jq_filter=""
  local template=""

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
    --json)
      json_fields="${2:-}"
      shift 2
      ;;
    --jq)
      jq_filter="${2:-}"
      shift 2
      ;;
    --template)
      template="${2:-}"
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

  local -a cmd
  if [[ -n "$issue_number" ]]; then
    ensure_number "--issue" "$issue_number"
    cmd=(gh issue view "$issue_number")
  else
    cmd=(gh issue list)
  fi

  if [[ -n "$repo" ]]; then
    cmd+=(-R "$repo")
  fi
  if [[ -n "$json_fields" ]]; then
    cmd+=(--json "$json_fields")
  fi
  if [[ -n "$jq_filter" ]]; then
    cmd+=(--jq "$jq_filter")
  fi
  if [[ -n "$template" ]]; then
    cmd+=(--template "$template")
  fi

  "${cmd[@]}"
}
