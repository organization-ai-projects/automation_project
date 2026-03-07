#!/usr/bin/env bash

auto_link_usage() {
  cat <<USAGE
Usage:
  issues/auto_link/run.sh --issue ISSUE_NUMBER

Notes:
  - Reads "Parent: #<number>" or "Parent: none" from issue body.
  - Attempts to link child -> parent as GitHub sub-issue via GraphQL.
  - On invalid input or API linking failure, posts actionable status comment and labels issue.
USAGE
}

auto_link_trim() {
  local s="${1:-}"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf "%s" "$s"
}

auto_link_graphql_has_errors() {
  local payload="${1:-}"
  jq -e '((.errors // []) | length) > 0' >/dev/null 2>&1 <<<"$payload"
}

auto_link_graphql_error_messages() {
  local payload="${1:-}"
  jq -r '(.errors // []) | map(.message // "unknown GraphQL error") | join("; ")' <<<"$payload" 2>/dev/null || true
}

auto_link_extract_parent_field_value() {
  local body="${1:-}"
  awk '
    BEGIN { IGNORECASE = 1 }
    /^[[:space:]]*Parent[[:space:]]*:/ {
      line = $0
      sub(/^[[:space:]]*Parent[[:space:]]*:[[:space:]]*/, "", line)
      print line
      exit
    }
  ' <<<"$body"
}

auto_link_require_deps() {
  issue_gh_require_gh_and_jq
}
