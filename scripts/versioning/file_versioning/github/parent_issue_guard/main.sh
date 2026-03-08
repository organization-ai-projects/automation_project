#!/usr/bin/env bash

parent_issue_guard_run() {
  local issue_arg=""
  local child_arg=""
  local strict_guard="true"

  parent_guard_parse_args issue_arg child_arg strict_guard "$@"
  parent_guard_validate_args "$issue_arg" "$child_arg" "$strict_guard"

  gh_cli_require_gh_jq

  local repo_name repo_owner repo_short_name
  parent_guard_init_repo_context repo_name repo_owner repo_short_name

  if [[ -n "$issue_arg" ]]; then
    parent_guard_require_number "--issue" "$issue_arg"
    parent_guard_evaluate_parent_issue "$strict_guard" "$repo_name" "$repo_owner" "$repo_short_name" "$issue_arg"
    return 0
  fi

  parent_guard_require_number "--child" "$child_arg"

  local parent_number
  while IFS= read -r parent_number; do
    [[ -z "$parent_number" ]] && continue
    if [[ "$parent_number" == "$child_arg" ]]; then
      continue
    fi
    parent_guard_evaluate_parent_issue "$strict_guard" "$repo_name" "$repo_owner" "$repo_short_name" "$parent_number"
  done < <(parent_guard_collect_parent_candidates "$repo_name" "$repo_owner" "$repo_short_name" "$child_arg")
}
