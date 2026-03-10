#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091,SC2034,SC2154

# Issue-view helpers shared across PR modules.

# shellcheck source=scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh
source "${PR_COMMON_DIR}/../../../../../common_lib/versioning/file_versioning/github/issue_helpers.sh"

pr_issue_view_full_json() {
  local issue_number="$1"
  local issue_key="#${issue_number}"
  local repo_name_with_owner
  local issue_json

  if [[ -n "${issue_view_full_json_cache[$issue_key]:-}" ]]; then
    echo "${issue_view_full_json_cache[$issue_key]}"
    return
  fi

  if [[ "$has_gh" != "true" ]]; then
    issue_view_full_json_cache["$issue_key"]=""
    echo ""
    return
  fi

  repo_name_with_owner="$(pr_get_repo_name_with_owner)"
  issue_json="$(github_issue_read_json "$repo_name_with_owner" "$issue_number" "title,body,labels" || true)"

  issue_view_full_json_cache["$issue_key"]="$issue_json"
  echo "$issue_json"
}
