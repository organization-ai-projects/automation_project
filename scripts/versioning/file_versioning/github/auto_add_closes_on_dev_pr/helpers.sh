#!/usr/bin/env bash
# shellcheck shell=bash

auto_add_extract_issue_numbers() {
  local refs="$1"
  printf '%s\n' "$refs" |
    cut -d'|' -f2 |
    sed -nE 's/^#([0-9]+)$/\1/p' |
    sort -u
}

auto_add_strip_managed_block() {
  local body="$1"
  awk '
    BEGIN { in_block = 0 }
    /^<!-- auto-closes:start -->$/ { in_block = 1; next }
    /^<!-- auto-closes:end -->$/ { in_block = 0; next }
    { if (!in_block) print }
  ' <<<"$body"
}
