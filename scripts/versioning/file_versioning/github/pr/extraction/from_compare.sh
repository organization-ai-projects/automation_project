#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Dry-run compare extraction helpers.

pr_load_dry_compare_commits_into_globals() {
  local compare_payload
  compare_payload="$(pr_load_dry_compare_commits "$base_ref_git" "$head_ref_git" || true)"
  if [[ -z "$compare_payload" ]]; then
    exit "$E_NO_DATA"
  fi
  dry_compare_commit_messages="${compare_payload%%$'\x1f'*}"
  dry_compare_commit_headlines="${compare_payload#*$'\x1f'}"
}

pr_extract_child_prs_dry() {
  local commit_headlines
  local message

  commit_headlines="$dry_compare_commit_headlines"
  if [[ -z "$commit_headlines" ]]; then
    pr_debug_log "extract_child_prs_dry(compare ${base_ref_git}...${head_ref_git}) => no commits found"
    return 1
  fi

  while IFS= read -r line; do
    [[ -z "$line" ]] && continue
    if [[ "$line" =~ ^[0-9a-f]{7,40}[[:space:]]+(.+)$ ]]; then
      message="${BASH_REMATCH[1]}"
    else
      message="$line"
    fi
    if [[ "$message" =~ Merge\ pull\ request\ \#([0-9]+) ]]; then
      pr_title_hint["#${BASH_REMATCH[1]}"]="$message"
    elif [[ "$message" =~ \(\#([0-9]+)\)[[:space:]]*$ ]]; then
      pr_title_hint["#${BASH_REMATCH[1]}"]="$message"
    fi
  done <<<"$commit_headlines"

  {
    echo "$commit_headlines" | sed -nE 's/.*Merge pull request #([0-9]+).*/#\1/p'
    echo "$commit_headlines" | sed -nE 's/.*\(#([0-9]+)\)\s*$/#\1/p'
  } | sort -u >"$extracted_prs_file"

  pr_debug_log "extract_child_prs_dry(compare ${base_ref_git}...${head_ref_git}) => $(tr '\n' ' ' <"$extracted_prs_file")"
  return 0
}
