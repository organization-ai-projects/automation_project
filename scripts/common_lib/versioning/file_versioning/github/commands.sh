#!/usr/bin/env bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# Single entrypoint for remote VCS provider commands (GitHub CLI today).

vcs_remote_run() {
  gh "$@"
}

vcs_remote_api() { vcs_remote_run api "$@"; }
vcs_remote_repo_view() { vcs_remote_run repo view "$@"; }

vcs_remote_pr_list() { vcs_remote_run pr list "$@"; }
vcs_remote_pr_view() { vcs_remote_run pr view "$@"; }
vcs_remote_pr_edit() { vcs_remote_run pr edit "$@"; }
vcs_remote_pr_create() { vcs_remote_run pr create "$@"; }
vcs_remote_pr_merge() { vcs_remote_run pr merge "$@"; }

vcs_remote_issue_list() { vcs_remote_run issue list "$@"; }
vcs_remote_issue_view() { vcs_remote_run issue view "$@"; }
vcs_remote_issue_edit() { vcs_remote_run issue edit "$@"; }
vcs_remote_issue_create() { vcs_remote_run issue create "$@"; }
vcs_remote_issue_close() { vcs_remote_run issue close "$@"; }
vcs_remote_issue_reopen() { vcs_remote_run issue reopen "$@"; }

vcs_remote_label_list() { vcs_remote_run label list "$@"; }
vcs_remote_label_edit() { vcs_remote_run label edit "$@"; }
vcs_remote_label_create() { vcs_remote_run label create "$@"; }
vcs_remote_label_delete() { vcs_remote_run label delete "$@"; }
