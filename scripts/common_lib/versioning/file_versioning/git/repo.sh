#!/bin/bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# shellcheck source=scripts/common_lib/versioning/file_versioning/git/commands.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/commands.sh"

# Functions related to Git repositories

# Verify if inside a Git repository
require_git_repo() {
  vcs_local_rev_parse --is-inside-work-tree >/dev/null 2>&1 || die "Not a git repository."
}
