#!/usr/bin/env bash

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
  if [[ -n "$repo" ]]; then
    cmd+=(-R "$repo")
  fi
  "${cmd[@]}" >/dev/null
  echo "Issue #${issue_number} closed (reason: ${reason})."
}

cmd_reopen() {
  local issue_number=""
  local repo=""

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
    -h | --help)
      usage
      exit 0
      ;;
    *)
      die_usage "Unknown option for reopen: $1"
      ;;
    esac
  done

  ensure_number "--issue" "$issue_number"
  local -a cmd=(gh issue reopen "$issue_number")
  if [[ -n "$repo" ]]; then
    cmd+=(-R "$repo")
  fi
  "${cmd[@]}" >/dev/null
  echo "Issue #${issue_number} reopened."
}

cmd_delete() {
  local issue_number=""
  local repo=""

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
    -h | --help)
      usage
      exit 0
      ;;
    *)
      die_usage "Unknown option for delete: $1"
      ;;
    esac
  done

  ensure_number "--issue" "$issue_number"
  local -a cmd=(gh issue close "$issue_number" --reason not_planned)
  if [[ -n "$repo" ]]; then
    cmd+=(-R "$repo")
  fi
  "${cmd[@]}" >/dev/null
  echo "Issue #${issue_number} soft-deleted (closed with reason: not_planned)."
}
