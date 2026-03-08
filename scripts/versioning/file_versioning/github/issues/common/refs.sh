#!/usr/bin/env bash

issue_cli_require_positive_number() {
  local name="$1"
  local value="$2"
  if [[ ! "$value" =~ ^[0-9]+$ ]]; then
    echo "Error: ${name} must be a positive integer." >&2
    exit 2
  fi
}

issue_refs_extract_closing_numbers() {
  local text="$1"
  parse_closing_issue_refs_from_text "$text" |
    cut -d'|' -f2 |
    sed -E 's/^#([0-9]+)$/\1/' |
    grep -E '^[0-9]+$' |
    sort -u
}

issue_refs_extract_reopen_numbers() {
  local text="$1"
  parse_reopen_issue_refs_from_text "$text" |
    cut -d'|' -f2 |
    sed -E 's/^#([0-9]+)$/\1/' |
    grep -E '^[0-9]+$' |
    sort -u
}
