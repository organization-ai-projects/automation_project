#!/usr/bin/env bash

neutralize_usage() {
  cat <<'USAGE'
Usage:
  neutralize_non_compliant_closure_refs.sh --pr PR_NUMBER [--repo owner/name]

Notes:
  - Detects closure refs in PR body (Closes/Fixes #...).
  - If the same issue also has `Reopen #...`, closure is neutralized on purpose.
  - If referenced issue is non-compliant with required issue contract, inserts:
      "<keyword> rejected #<issue>"
    to neutralize GitHub auto-close behavior.
  - Posts/updates a deterministic status comment in the PR thread.
USAGE
}

neutralize_require_number() {
  local name="$1"
  local value="${2:-}"
  if [[ ! "$value" =~ ^[0-9]+$ ]]; then
    echo "Error: ${name} must be a positive integer." >&2
    exit 2
  fi
}

neutralize_trim() {
  local s="${1:-}"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf "%s" "$s"
}

neutralize_require_deps() {
  if ! command -v gh >/dev/null 2>&1; then
    echo "Error: gh is required." >&2
    exit 3
  fi
  if ! command -v jq >/dev/null 2>&1; then
    echo "Error: jq is required." >&2
    exit 3
  fi
  if ! command -v perl >/dev/null 2>&1; then
    echo "Error: perl is required." >&2
    exit 3
  fi
}

neutralize_keyword_pattern_from_action() {
  local action="$1"
  case "$action" in
  Closes) echo "closes|fixes" ;;
  *) echo "" ;;
  esac
}
