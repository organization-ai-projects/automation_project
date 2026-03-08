#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Commit compare loaders (public API for PR pipeline).

pr_load_compare_commit_messages() {
  local compare_base="$1"
  local compare_head="$2"
  local compare_messages

  # Deterministic-first: use local git history when available.
  compare_messages="$(pr_compare_local_commit_messages "$compare_base" "$compare_head")"
  if [[ -n "$compare_messages" ]]; then
    printf "%s" "$compare_messages"
    return 0
  fi

  compare_messages="$(pr_compare_api_commit_messages "$compare_base" "$compare_head" || true)"
  if [[ -n "$compare_messages" ]]; then
    printf "%s" "$compare_messages"
    return 0
  fi

  echo "Error: unable to retrieve commit messages for ${compare_base}..${compare_head}." >&2
  return 1
}

pr_load_compare_commit_headlines() {
  local compare_base="$1"
  local compare_head="$2"
  local compare_headlines

  # Deterministic-first: use local git history when available.
  compare_headlines="$(pr_compare_local_commit_headlines "$compare_base" "$compare_head")"
  if [[ -n "$compare_headlines" ]]; then
    printf "%s" "$compare_headlines"
    return 0
  fi

  compare_headlines="$(pr_compare_api_commit_headlines "$compare_base" "$compare_head" || true)"
  if [[ -z "$compare_headlines" ]]; then
    return 1
  fi

  printf "%s" "$compare_headlines"
}

pr_load_dry_compare_commits() {
  local compare_base="$1"
  local compare_head="$2"
  local compare_messages
  local compare_headlines

  compare_messages="$(pr_load_compare_commit_messages "$compare_base" "$compare_head" || true)"
  if [[ -z "$compare_messages" ]]; then
    echo "Error: unable to determine commit messages for --dry-run compare ${compare_base}...${compare_head}." >&2
    return 1
  fi

  compare_headlines="$(pr_load_compare_commit_headlines "$compare_base" "$compare_head" || true)"
  if [[ -z "$compare_headlines" ]]; then
    echo "Error: unable to determine commit headlines for --dry-run compare ${compare_base}...${compare_head}." >&2
    return 1
  fi

  printf "%s\x1f%s" "$compare_messages" "$compare_headlines"
}
