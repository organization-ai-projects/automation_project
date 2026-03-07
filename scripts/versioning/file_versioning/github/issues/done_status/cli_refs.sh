#!/usr/bin/env bash

done_status_usage() {
  cat <<EOF_USAGE
Usage:
  issues/done_status/run.sh --on-dev-merge --pr PR_NUMBER [--label LABEL]
  issues/done_status/run.sh --on-issue-closed --issue ISSUE_NUMBER [--label LABEL]
EOF_USAGE
}

done_status_require_number() {
  issue_cli_require_positive_number "$1" "$2"
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
  issue_refs_extract_closing_numbers "$1"
}

done_status_collect_pr_text_payload() {
  issue_gh_collect_pr_text_payload "$1" "$2"
}
