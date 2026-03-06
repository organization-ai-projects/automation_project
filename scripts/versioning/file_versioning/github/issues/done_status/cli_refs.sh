#!/usr/bin/env bash

done_status_usage() {
  cat <<EOF_USAGE
Usage:
  issue_done_in_dev_status.sh --on-dev-merge --pr PR_NUMBER [--label LABEL]
  issue_done_in_dev_status.sh --on-issue-closed --issue ISSUE_NUMBER [--label LABEL]
EOF_USAGE
}

done_status_require_number() {
  local name="$1"
  local value="$2"
  if [[ ! "$value" =~ ^[0-9]+$ ]]; then
    echo "Error: ${name} must be a positive integer." >&2
    exit 2
  fi
}

done_status_require_cmd() {
  issue_gh_require_cmd "$1"
}

done_status_resolve_repo_name() {
  issue_gh_resolve_repo_name
}

done_status_label_exists() {
  issue_gh_label_exists "$1" "$2"
}

done_status_issue_state() {
  issue_gh_issue_state "$1" "$2"
}

done_status_issue_has_label() {
  issue_gh_issue_has_label "$1" "$2" "$3"
}

done_status_extract_closing_issue_numbers() {
  local text="$1"
  parse_closing_issue_refs_from_text "$text" |
    cut -d'|' -f2 |
    sed -E 's/^#([0-9]+)$/\1/' |
    grep -E '^[0-9]+$' |
    sort -u
}

done_status_collect_pr_text_payload() {
  issue_gh_collect_pr_text_payload "$1" "$2"
}
