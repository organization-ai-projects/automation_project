#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Issue contract loading/profile helpers.

ISSUE_CONTRACT_LOADED="false"
ISSUE_CONTRACT_LOADED_PATH=""

issue_contract_file() {
  local root=""
  root="$(git rev-parse --show-toplevel 2>/dev/null || true)"
  if [[ -n "$root" && -f "$root/.github/issue_required_fields.conf" ]]; then
    echo "$root/.github/issue_required_fields.conf"
    return
  fi
  if [[ -z "$root" ]]; then
    echo "Warning: unable to resolve git repository root; falling back to relative issue contract path." >&2
  fi
  echo ".github/issue_required_fields.conf"
}

issue_contract_load() {
  local contract
  contract="$(issue_contract_file)"
  if [[ "$ISSUE_CONTRACT_LOADED" == "true" && "$ISSUE_CONTRACT_LOADED_PATH" == "$contract" ]]; then
    return 0
  fi
  if [[ ! -f "$contract" ]]; then
    echo "Missing issue contract file: ${contract}" >&2
    return 1
  fi
  # shellcheck disable=SC1090
  source "$contract"
  ISSUE_CONTRACT_LOADED="true"
  ISSUE_CONTRACT_LOADED_PATH="$contract"
}

issue_contract_profile_for_labels() {
  local labels_raw="${1:-}"
  local lower_labels
  lower_labels="$(echo "$labels_raw" | tr '[:upper:]' '[:lower:]')"
  if [[ "$lower_labels" =~ (^|\|\|)review(\|\||$) ]]; then
    echo "review"
    return
  fi
  echo "default"
}

issue_contract_key_for_profile() {
  local profile="${1:-default}"
  local base_key="${2:-}"
  if [[ -z "$base_key" ]]; then
    echo ""
    return
  fi
  if [[ "$profile" == "review" ]]; then
    echo "ISSUE_REVIEW_${base_key}"
    return
  fi
  echo "ISSUE_${base_key}"
}

issue_contract_get() {
  local key="${1:-}"
  if [[ -z "$key" ]]; then
    echo ""
    return
  fi
  # shellcheck disable=SC2154
  echo "${!key-}"
}

