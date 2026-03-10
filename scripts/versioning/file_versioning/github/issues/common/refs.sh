#!/usr/bin/env bash

issue_cli_require_positive_number() {
  local name="$1"
  local value="$2"
  if [[ ! "$value" =~ ^[0-9]+$ ]]; then
    echo "Error: ${name} must be a positive integer." >&2
    exit 2
  fi
}

issue_refs_extract_numbers_from_refs() {
  local refs="$1"
  parse_issue_numbers_from_refs "$refs"
}

issue_refs_extract_closing_numbers() {
  local text="$1"
  issue_refs_extract_numbers_from_refs "$(parse_closing_issue_refs_from_text "$text")"
}

issue_refs_extract_all_closing_numbers() {
  local text="$1"
  issue_refs_extract_numbers_from_refs "$(parse_all_closing_issue_refs_from_text "$text")"
}

issue_refs_extract_reopen_numbers() {
  local text="$1"
  issue_refs_extract_numbers_from_refs "$(parse_reopen_issue_refs_from_text "$text")"
}
