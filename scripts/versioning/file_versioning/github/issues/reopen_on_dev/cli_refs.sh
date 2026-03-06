#!/usr/bin/env bash

reopen_on_dev_usage() {
  cat <<EOF_USAGE
Usage:
  issue_reopen_on_dev_merge.sh --pr PR_NUMBER [--label LABEL]
EOF_USAGE
}

reopen_on_dev_require_number() {
  issue_cli_require_positive_number "$1" "$2"
}

reopen_on_dev_require_cmd() {
  issue_gh_require_cmd "$1"
}

reopen_on_dev_resolve_repo_name() {
  issue_gh_resolve_repo_name
}

reopen_on_dev_label_exists() {
  issue_gh_label_exists "$1" "$2"
}

reopen_on_dev_issue_state() {
  issue_gh_issue_state "$1" "$2"
}

reopen_on_dev_issue_has_label() {
  issue_gh_issue_has_label "$1" "$2" "$3"
}

reopen_on_dev_extract_issue_numbers() {
  issue_refs_extract_reopen_numbers "$1"
}

reopen_on_dev_collect_pr_text_payload() {
  issue_gh_collect_pr_text_payload "$1" "$2"
}
