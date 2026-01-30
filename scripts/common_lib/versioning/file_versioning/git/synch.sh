#!/bin/bash

# Functions related to synchronization

# Fetch with prune
git_fetch_prune() {
  local remote="${1:-origin}"
  info "Fetching from $remote with prune..."
  git fetch --prune "$remote"
}