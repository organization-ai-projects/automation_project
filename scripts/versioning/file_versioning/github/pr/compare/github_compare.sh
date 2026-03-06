#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# GitHub compare helpers with retry policy.

pr_compare_api_commit_messages() {
  local compare_base="$1"
  local compare_head="$2"
  local compare_base_api="$compare_base"
  local compare_head_api="$compare_head"
  local repo_name_with_owner
  local compare_range
  local compare_err_file
  local compare_err
  local compare_messages
  local attempt
  local max_attempts=3
  local compare_ok=0

  compare_base_api="${compare_base_api#origin/}"
  compare_head_api="${compare_head_api#origin/}"
  repo_name_with_owner="$(pr_get_repo_name_with_owner)"
  compare_range="${compare_base_api}...${compare_head_api}"
  compare_err_file="$(mktemp)"

  if [[ -n "$repo_name_with_owner" ]]; then
    for attempt in $(seq 1 "$max_attempts"); do
      compare_messages="$(gh api "repos/${repo_name_with_owner}/compare/${compare_range}" \
        --jq '.commits[]?.commit.message' 2>"$compare_err_file" || true)"

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
  local compare_base_api="$compare_base"
  local compare_head_api="$compare_head"
  local repo_name_with_owner
  local compare_range
  local compare_headlines

  compare_base_api="${compare_base_api#origin/}"
  compare_head_api="${compare_head_api#origin/}"

  repo_name_with_owner="$(pr_get_repo_name_with_owner)"
  compare_range="${compare_base_api}...${compare_head_api}"
  compare_headlines=""

  if [[ -n "$repo_name_with_owner" ]]; then
    compare_headlines="$(gh api "repos/${repo_name_with_owner}/compare/${compare_range}" \
      --jq '.commits[]?.commit.message | split("\n")[0]' 2>/dev/null || true)"
  fi

  if [[ -z "$compare_headlines" ]]; then
    return 1
  fi

  printf "%s" "$compare_headlines"
}
