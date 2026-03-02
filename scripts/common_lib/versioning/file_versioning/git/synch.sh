#!/bin/bash

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  echo "Error: $(basename "$0") is a library script and must be sourced, not executed directly." >&2
  exit 2
fi

# shellcheck source=scripts/common_lib/versioning/file_versioning/git/commands.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/commands.sh"

# Functions related to synchronization

# Fetch with prune
git_fetch_prune() {
  local remote="${1:-origin}"
  info "Fetching from $remote with prune..."
  vcs_local_fetch --prune "$remote"
}
