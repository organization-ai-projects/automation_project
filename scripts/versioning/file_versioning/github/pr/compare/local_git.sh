#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Local git compare helpers.

pr_compare_local_commit_messages() {
  local compare_base="$1"
  local compare_head="$2"
  git log --format=%B "${compare_base}..${compare_head}" 2>/dev/null || true
}

pr_compare_local_commit_headlines() {
  local compare_base="$1"
  local compare_head="$2"
  git log --format=%s "${compare_base}..${compare_head}" 2>/dev/null || true
}
