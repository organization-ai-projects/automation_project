#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# GitHub compare helpers with retry policy.

pr_compare_api_context() {
  local compare_base="$1"
  local compare_head="$2"
  local compare_base_api="${compare_base#origin/}"
  local compare_head_api="${compare_head#origin/}"
  local repo_name_with_owner
  local compare_range

  repo_name_with_owner="$(pr_get_repo_name_with_owner)"
  compare_range="${compare_base_api}...${compare_head_api}"
  printf "%s|%s" "$repo_name_with_owner" "$compare_range"
}

pr_compare_api_call() {
  local repo_name_with_owner="$1"
  local compare_range="$2"
  local jq_filter="$3"
  local err_file="${4:-}"

  if [[ -n "$err_file" ]]; then
    pr_repo_api_call "$repo_name_with_owner" "compare/${compare_range}" --jq "$jq_filter" 2>"$err_file" || true
    return
  fi
  pr_repo_api_call "$repo_name_with_owner" "compare/${compare_range}" --jq "$jq_filter" 2>/dev/null || true
}

pr_compare_api_commit_messages() {
  local compare_base="$1"
  local compare_head="$2"
  local context
  local repo_name_with_owner
  local compare_range
  local compare_err_file
  local compare_err
  local compare_messages
  local attempt
  local max_attempts=3
  local compare_ok=0

  context="$(pr_compare_api_context "$compare_base" "$compare_head")"
  repo_name_with_owner="${context%%|*}"
  compare_range="${context#*|}"
  compare_err_file="$(mktemp)"

  if [[ -n "$repo_name_with_owner" ]]; then
    for attempt in $(seq 1 "$max_attempts"); do
      compare_messages="$(pr_compare_api_call "$repo_name_with_owner" "$compare_range" '.commits[]?.commit.message' "$compare_err_file")"

      if [[ -n "$compare_messages" ]]; then
        compare_ok=1
        break
      fi

      compare_err="$(cat "$compare_err_file" 2>/dev/null || true)"
      if echo "$compare_err" | grep -qiE 'error connecting to api.github.com|timeout|temporarily unavailable|EOF|reset by peer'; then
        sleep "$attempt"
        continue
      fi
      break
    done
  fi

  if [[ $compare_ok -ne 1 ]]; then
    compare_err="$(cat "$compare_err_file" 2>/dev/null || true)"
    echo "Warning: GitHub compare failed (${compare_range}). Falling back to local git history." >&2
    if [[ -n "$compare_err" ]]; then
      echo "Detail: ${compare_err}" >&2
    fi
    rm -f "$compare_err_file"
    return 1
  fi

  rm -f "$compare_err_file"
  printf "%s" "$compare_messages"
}

pr_compare_api_commit_headlines() {
  local compare_base="$1"
  local compare_head="$2"
  local context
  local repo_name_with_owner
  local compare_range
  local compare_headlines

  context="$(pr_compare_api_context "$compare_base" "$compare_head")"
  repo_name_with_owner="${context%%|*}"
  compare_range="${context#*|}"
  compare_headlines=""

  if [[ -n "$repo_name_with_owner" ]]; then
    compare_headlines="$(pr_compare_api_call "$repo_name_with_owner" "$compare_range" '.commits[]?.commit.message | split("\n")[0]')"
  fi

  if [[ -z "$compare_headlines" ]]; then
    return 1
  fi

  printf "%s" "$compare_headlines"
}
