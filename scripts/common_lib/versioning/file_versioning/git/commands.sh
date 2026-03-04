#!/usr/bin/env bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# Single entrypoint for git CLI calls.
# Higher-level scripts should call these wrappers instead of invoking `git` directly.

vcs_local_run() {
  git "$@"
}

vcs_local_add() { vcs_local_run add "$@"; }
vcs_local_branch() { vcs_local_run branch "$@"; }
vcs_local_checkout() { vcs_local_run checkout "$@"; }
vcs_local_commit() { vcs_local_run commit "$@"; }
vcs_local_diff() { vcs_local_run diff "$@"; }
vcs_local_fetch() { vcs_local_run fetch "$@"; }
vcs_local_for_each_ref() { vcs_local_run for-each-ref "$@"; }
vcs_local_log() { vcs_local_run log "$@"; }
vcs_local_ls_files() { vcs_local_run ls-files "$@"; }
vcs_local_ls_remote() { vcs_local_run ls-remote "$@"; }
vcs_local_merge_base() { vcs_local_run merge-base "$@"; }
vcs_local_pull() { vcs_local_run pull "$@"; }
vcs_local_push() { vcs_local_run push "$@"; }
vcs_local_reset() { vcs_local_run reset "$@"; }
vcs_local_rev_list() { vcs_local_run rev-list "$@"; }
vcs_local_rev_parse() { vcs_local_run rev-parse "$@"; }
vcs_local_show_ref() { vcs_local_run show-ref "$@"; }
vcs_local_status() { vcs_local_run status "$@"; }
vcs_local_switch() { vcs_local_run switch "$@"; }
