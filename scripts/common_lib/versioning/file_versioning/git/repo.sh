#!/bin/bash

# Functions related to Git repositories

# Verify if inside a Git repository
require_git_repo() {
  git rev-parse --is-inside-work-tree >/dev/null 2>&1 || die "Not a git repository."
}