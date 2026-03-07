#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Runtime cache/state helpers.

pr_cleanup_tmp_files() {
  rm -f "$features_tmp" "$bugs_tmp" "$refactors_tmp" "$sync_tmp" "$issues_tmp" "$reopen_tmp" "$conflict_tmp" "$directive_resolution_tmp"
  if [[ "$keep_artifacts" != "true" ]]; then
    rm -f "$extracted_prs_file" "$resolved_issues_file" "$reopened_issues_file" "$conflict_issues_file"
  fi
}

pr_get_repo_name_with_owner() {
  if [[ "$has_gh" != "true" ]]; then
    echo ""
    return
  fi

  if [[ -n "$repo_name_with_owner_cache" ]]; then
    echo "$repo_name_with_owner_cache"
    return
  fi

  if [[ -n "${GH_REPO:-}" ]]; then
    repo_name_with_owner_cache="$GH_REPO"
    echo "$repo_name_with_owner_cache"
    return
  fi

  repo_name_with_owner_cache="$(pr_gh_optional "resolve repository name" repo view --json nameWithOwner -q '.nameWithOwner')"
  echo "$repo_name_with_owner_cache"
}

pr_seed_pr_ref_cache() {
  local pr_ref

  if [[ -n "$main_pr_number" ]]; then
    pr_ref_cache["#${main_pr_number}"]="1"
  fi

  if [[ -s "$extracted_prs_file" ]]; then
    while read -r pr_ref; do
      [[ -z "$pr_ref" ]] && continue
      pr_ref_cache["$pr_ref"]="1"
    done <"$extracted_prs_file"
  fi
}
