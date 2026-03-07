#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Shared extraction helpers for commit-headline parsing.

pr_seed_pr_title_hints_from_headlines() {
  local commit_headlines="$1"
  local line message

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
}

pr_extract_pr_refs_from_headlines() {
  local commit_headlines="$1"
  {
    echo "$commit_headlines" | sed -nE 's/.*Merge pull request #([0-9]+).*/#\1/p'
    echo "$commit_headlines" | sed -nE 's/.*\(#([0-9]+)\)\s*$/#\1/p'
  }
}

pr_extract_pr_refs_from_text() {
  local text="$1"
  {
    echo "$text" | grep -oE '/pull/[0-9]+' | sed -E 's#^/pull/([0-9]+)$#\#\1#'
    echo "$text" | sed -nE 's/.*\bPR[[:space:]]*#([0-9]+).*/#\1/ip'
    echo "$text" | sed -nE 's/.*pull request #([0-9]+).*/#\1/ip'
  }
}
