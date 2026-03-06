#!/usr/bin/env bash
# shellcheck shell=bash

pr_directive_conflict_guard_trim() {
  local s="${1:-}"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf '%s' "$s"
}

pr_directive_conflict_guard_upsert_conflict_block_in_body() {
  local body="$1"
  local block="$2"
  local block_start="$3"
  local block_end="$4"
  local without_block

  without_block="$(
    perl -0777 -pe "s@\n?${block_start}.*?${block_end}\n?@@s" <<<"$body"
  )"

  if [[ -z "$block" ]]; then
    printf '%s' "$without_block"
    return
  fi

  printf '%s\n\n%s\n' "$without_block" "$block"
}

pr_directive_conflict_guard_apply_reopen_rejected_marker() {
  local body="$1"
  local issue_key="$2"

  DIRECTIVE_CONFLICT_ISSUE_KEY="$issue_key" \
    perl -0777 -pe '
      my $ik = $ENV{DIRECTIVE_CONFLICT_ISSUE_KEY} // q{};
      my $ikq = quotemeta($ik);
      s/\b((?:reopen|reopens))\b(\s+)(?!rejected\b)([^\s]*$ikq)\b/$1$2rejected $3/ig;
    ' <<<"$body"
}
