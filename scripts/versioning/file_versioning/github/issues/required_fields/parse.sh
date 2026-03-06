#!/usr/bin/env bash
# shellcheck shell=bash

# Issue content parsing helpers.

trim_whitespace() {
  local s="${1:-}"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf "%s" "$s"
}

issue_extract_field_value() {
  local body="${1:-}"
  local field="${2:-}"
  awk -v field="$field" '
    BEGIN {
      field_lc = tolower(field)
    }
    {
      line = $0
      lower_line = tolower($0)
      pattern = "^[[:space:]]*" field_lc "[[:space:]]*:[[:space:]]*"
      if (lower_line ~ pattern) {
        match(lower_line, pattern)
        line = substr(line, RLENGTH + 1)
        print line
        exit
      }
    }
  ' <<<"$body"
}

issue_body_has_section() {
  local body="${1:-}"
  local section="${2:-}"
  awk -v expected="$section" '
    function trim(s) {
      sub(/^[[:space:]]+/, "", s)
      sub(/[[:space:]]+$/, "", s)
      return s
    }
    BEGIN {
      target = tolower(trim(expected))
      found = 0
    }
    {
      current = tolower(trim($0))
      if (current == target) {
        found = 1
        exit
      }
    }
    END { exit(found ? 0 : 1) }
  ' <<<"$body"
}

