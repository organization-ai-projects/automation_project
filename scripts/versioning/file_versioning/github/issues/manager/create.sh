#!/usr/bin/env bash

cmd_create() {
  local add_default_issue_label=1
  local create_direct_issue_script="${MANAGER_ISSUES_CREATE_SCRIPT:-${ISSUES_DIR}/create_direct/run.sh}"
  local -a passthrough=()
  local -a labels=()
  local i=1
  local arg next next_index

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
      issue_cli_require_option_value "$arg" "$next" die_usage
      labels+=("$next")
      passthrough+=("$arg" "$next")
      i=$((i + 2))
      continue
    fi

    passthrough+=("$arg")
    i=$((i + 1))
  done

  if [[ ! -x "$create_direct_issue_script" ]]; then
    die_usage "create script is missing or not executable: $create_direct_issue_script"
  fi

  if [[ $add_default_issue_label -eq 1 ]] && ! is_label_present "issue" "${labels[@]}"; then
    passthrough+=(--label "issue")
  fi

  # Prefer Rust CLI even in legacy dispatch path; keep explicit script override for regressions/debugging.
  if [[ -z "${MANAGER_ISSUES_CREATE_SCRIPT:-}" ]]; then
    va_exec issue create "${passthrough[@]}"
    return $?
  fi

  if [[ ! -x "$create_direct_issue_script" ]]; then
    die_usage "create script is missing or not executable: $create_direct_issue_script"
  fi

  "$create_direct_issue_script" "${passthrough[@]}"
}
