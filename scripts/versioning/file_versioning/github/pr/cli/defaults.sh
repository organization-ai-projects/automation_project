#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# CLI default-state helpers.

pr_args_init_defaults() {
  main_pr_number=""
  output_file="pr_description.md"
  output_file_explicit="false"
  write_body_to_file="true"
  keep_artifacts="false"
  dry_run="false"
  base_ref=""
  head_ref=""
  create_pr="false"
  allow_partial_create="false"
  assume_yes="false"
  auto_mode="false"
  mode_explicit="false"
  auto_edit_pr_number=""
  refresh_pr_used="false"
  debug_mode="false"
  duplicate_mode=""
  repo_name_with_owner_cache=""
  validation_only="false"
  allow_inferred_directive_conflicts="false"
  positionals=()
}
