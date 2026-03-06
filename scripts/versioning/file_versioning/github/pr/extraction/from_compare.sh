#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Compare-payload extraction helpers (used when dry-run routing is active).

pr_load_dry_compare_commits_into_globals() {
  local compare_payload
  compare_payload="$(pr_load_dry_compare_commits "$base_ref_git" "$head_ref_git" || true)"
  if [[ -z "$compare_payload" ]]; then
    exit "$E_NO_DATA"
  fi
  dry_compare_commit_messages="${compare_payload%%$'\x1f'*}"
  dry_compare_commit_headlines="${compare_payload#*$'\x1f'}"
}

pr_extract_child_prs_from_compare() {
  local commit_headlines

  commit_headlines="$dry_compare_commit_headlines"
  if [[ -z "$commit_headlines" ]]; then
    pr_debug_log "extract_child_prs(compare ${base_ref_git}...${head_ref_git}) => no commits found"
    return 1
  fi

  pr_seed_pr_title_hints_from_headlines "$commit_headlines"

  pr_extract_pr_refs_from_headlines "$commit_headlines" | sort -u >"$extracted_prs_file"

  pr_debug_log "extract_child_prs(compare ${base_ref_git}...${head_ref_git}) => $(tr '\n' ' ' <"$extracted_prs_file")"
  return 0
}

# Backward-compatible alias used by existing call sites/tests.
pr_extract_child_prs_dry() {
  pr_extract_child_prs_from_compare "$@"
}
