#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CREATE_DIRECT_ISSUE_SCRIPT="${MANAGER_ISSUES_CREATE_SCRIPT:-${SCRIPT_DIR}/create_direct_issue.sh}"

usage() {
  cat <<'USAGE'
Usage:
  manager_issues.sh create [create_direct_issue options...]
    Required passthrough options:
      --title --context --problem --acceptance ...
    Optional:
      --no-default-issue-label

  manager_issues.sh read [--issue <number>] [--repo owner/name] [--json fields] [--jq filter] [--template tpl]
    - With --issue: show a single issue (gh issue view)
    - Without --issue: list issues (gh issue list)
    - Machine-readable: combine --json/--jq/--template as supported by gh

  manager_issues.sh update --issue <number> [--repo owner/name] [edit options...]
    Edit options:
      --title "new title"
      --body "new body"
      --add-label "label"
      --remove-label "label"
      --add-assignee "user"
      --remove-assignee "user"

  manager_issues.sh close --issue <number> [--repo owner/name] [--reason completed|not_planned]
  manager_issues.sh reopen --issue <number> [--repo owner/name]
  manager_issues.sh delete --issue <number> [--repo owner/name]

Notes:
  - create is routed through create_direct_issue.sh contract validation.
  - create applies label "issue" by default (unless --no-default-issue-label is passed).
  - delete performs a soft delete: closes the issue with reason not_planned.
USAGE
}

die_usage() {
  echo "Error: $*" >&2
  usage >&2
  exit 2
}

ensure_number() {
  local name="$1"
  local value="${2:-}"
  if [[ ! "$value" =~ ^[0-9]+$ ]]; then
    die_usage "${name} must be a positive integer."
  fi
}

is_label_present() {
  local needle="$1"
  shift
  local label
  for label in "$@"; do
    if [[ "${label,,}" == "${needle,,}" ]]; then
      return 0
    fi
  done
  return 1
}

cmd_create() {
  local add_default_issue_label=1
  local -a passthrough=()
  local -a labels=()
  local i=1
  local arg next

  while [[ $i -le $# ]]; do
    arg="${!i}"
    if [[ "$arg" == "--no-default-issue-label" ]]; then
      add_default_issue_label=0
      i=$((i + 1))
      continue
    fi

    if [[ "$arg" == "--label" ]]; then
      next_index=$((i + 1))
      next="${!next_index:-}"
      [[ -n "$next" ]] || die_usage "--label requires a value."
      labels+=("$next")
      passthrough+=("$arg" "$next")
      i=$((i + 2))
      continue
    fi

    passthrough+=("$arg")
    i=$((i + 1))
  done

  if [[ ! -x "$CREATE_DIRECT_ISSUE_SCRIPT" ]]; then
    die_usage "create script is missing or not executable: $CREATE_DIRECT_ISSUE_SCRIPT"
  fi

  if [[ $add_default_issue_label -eq 1 ]] && ! is_label_present "issue" "${labels[@]}"; then
    passthrough+=(--label "issue")
  fi

  "$CREATE_DIRECT_ISSUE_SCRIPT" "${passthrough[@]}"
}

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
      -h|--help)
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

cmd_update() {
  local issue_number=""
  local repo=""
  local -a edit_args=()

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
      --title|--body|--add-label|--remove-label|--add-assignee|--remove-assignee)
        [[ -n "${2:-}" ]] || die_usage "$1 requires a value."
        edit_args+=("$1" "${2:-}")
        shift 2
        ;;
      -h|--help)
        usage
        exit 0
        ;;
      *)
        die_usage "Unknown option for update: $1"
        ;;
    esac
  done

  ensure_number "--issue" "$issue_number"
  if [[ ${#edit_args[@]} -eq 0 ]]; then
    die_usage "update requires at least one edit option."
  fi

  local -a cmd=(gh issue edit "$issue_number")
  if [[ -n "$repo" ]]; then
    cmd+=(-R "$repo")
  fi
  cmd+=("${edit_args[@]}")
  "${cmd[@]}" >/dev/null
  echo "Issue #${issue_number} updated."
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
      -h|--help)
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
      -h|--help)
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
      -h|--help)
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

main() {
  local subcommand="${1:-}"
  if [[ -z "$subcommand" ]]; then
    usage
    exit 2
  fi
  shift || true

  case "$subcommand" in
    create) cmd_create "$@" ;;
    read) cmd_read "$@" ;;
    update) cmd_update "$@" ;;
    close) cmd_close "$@" ;;
    reopen) cmd_reopen "$@" ;;
    delete) cmd_delete "$@" ;;
    -h|--help) usage ;;
    *) die_usage "Unknown subcommand: $subcommand" ;;
  esac
}

main "$@"
