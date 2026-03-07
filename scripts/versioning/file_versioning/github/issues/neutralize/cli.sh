#!/usr/bin/env bash

neutralize_usage() {
  cat <<'USAGE'
Usage:
  issues/neutralize/run.sh --pr PR_NUMBER [--repo owner/name]

Notes:
  - Detects closure refs in PR body (Closes/Fixes #...).
  - If the same issue also has `Reopen #...`, closure is neutralized on purpose.
  - If referenced issue is non-compliant with required issue contract, inserts:
      "<keyword> rejected #<issue>"
    to neutralize GitHub auto-close behavior.
  - Posts/updates a deterministic status comment in the PR thread.
USAGE
}

neutralize_trim() {
  local s="${1:-}"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf "%s" "$s"
}

neutralize_require_deps() {
  issue_gh_require_cmd gh
  issue_gh_require_cmd jq
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
