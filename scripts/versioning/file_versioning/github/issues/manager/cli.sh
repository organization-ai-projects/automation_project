#!/usr/bin/env bash

usage() {
  cat <<'USAGE'
Usage:
  issues/manager/run.sh create [create_direct_issue options...]
    Required passthrough options:
      --title --context --problem --acceptance ...
    Optional:
      --no-default-issue-label

  issues/manager/run.sh read [--issue <number>] [--repo owner/name] [--json fields] [--jq filter] [--template tpl]
    - With --issue: show a single issue (gh issue view)
    - Without --issue: list issues (gh issue list)
    - Machine-readable: combine --json/--jq/--template as supported by gh

  issues/manager/run.sh update --issue <number> [--repo owner/name] [edit options...]
    Edit options:
      --title "new title"
      --body "new body"
      --add-label "label"
      --remove-label "label"
      --add-assignee "user"
      --remove-assignee "user"

  issues/manager/run.sh close --issue <number> [--repo owner/name] [--reason completed|not_planned]
  issues/manager/run.sh reopen --issue <number> [--repo owner/name]
  issues/manager/run.sh delete --issue <number> [--repo owner/name]

Notes:
  - create is routed through issues/create_direct/run.sh contract validation.
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

manager_issues_main() {
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
  -h | --help) usage ;;
  *) die_usage "Unknown subcommand: $subcommand" ;;
  esac
}
