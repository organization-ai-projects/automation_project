#!/usr/bin/env bash

cmd_create() {
  local add_default_issue_label=1
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
