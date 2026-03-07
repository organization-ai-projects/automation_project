#!/usr/bin/env bash

done_status_usage() {
  cat <<EOF_USAGE
Usage:
  issues/done_status/run.sh --on-dev-merge --pr PR_NUMBER [--label LABEL]
  issues/done_status/run.sh --on-issue-closed --issue ISSUE_NUMBER [--label LABEL]
EOF_USAGE
}
