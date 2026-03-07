#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Pipeline issue-outcome rendering helpers.

pr_pipeline_issue_is_excluded_by_directive() {
  local issue_key="$1"
  [[ -n "${issue_directive_conflict_reason[$issue_key]:-}" || -n "${issue_directive_resolution[$issue_key]:-}" ]]
}

pr_pipeline_directive_prefix_for_issue() {
  local issue_key="$1"
  local directive_action="${issue_directive_final_action[$issue_key]:-}"

  case "$directive_action" in
  close) echo "Closes ${issue_key} - " ;;
  reopen) echo "Reopen ${issue_key} - " ;;
  *) echo "${issue_key} - " ;;
  esac
}

pr_pipeline_sorted_issue_keys_from_assoc() {
  local assoc_var_name="$1"
  local -n assoc_ref="$assoc_var_name"
  printf '%s\n' "${!assoc_ref[@]}" | sort -V
}

pr_pipeline_render_issue_outcomes_files() {
  local issue_key issue_number directive_prefix

  : >"$issues_tmp"
  while IFS= read -r issue_key; do
    [[ -z "$issue_key" ]] && continue
    pr_pipeline_issue_is_excluded_by_directive "$issue_key" && continue
    issue_number="${issue_key//#/}"
    echo "${issue_number}|${issue_category[$issue_key]}|${issue_action[$issue_key]}|${issue_key}" >>"$issues_tmp"
  done < <(pr_pipeline_sorted_issue_keys_from_assoc seen_issue)

  if [[ -s "$issues_tmp" ]]; then
    pr_render_grouped_by_category "$issues_tmp" "resolved" "$resolved_issues_file"
    issue_count="$(wc -l <"$issues_tmp" | tr -d ' ')"
  fi

  : >"$reopen_tmp"
  while IFS= read -r issue_key; do
    [[ -z "$issue_key" ]] && continue
    pr_pipeline_issue_is_excluded_by_directive "$issue_key" && continue
    issue_number="${issue_key//#/}"
    echo "${issue_number}|${reopen_issue_category[$issue_key]:-Unknown}|${issue_key}" >>"$reopen_tmp"
  done < <(pr_pipeline_sorted_issue_keys_from_assoc seen_reopen_issue)

  if [[ -s "$reopen_tmp" ]]; then
    pr_render_grouped_by_category "$reopen_tmp" "reopen" "$reopened_issues_file"
    reopen_issue_count="${#seen_reopen_issue[@]}"
  fi

  : >"$conflict_tmp"
  while IFS= read -r issue_key; do
    [[ -z "$issue_key" ]] && continue
    issue_number="${issue_key//#/}"
    echo "${issue_number}|${issue_category[$issue_key]:-Unknown}|${issue_key}|${issue_directive_conflict_reason[$issue_key]}" >>"$conflict_tmp"
  done < <(pr_pipeline_sorted_issue_keys_from_assoc issue_directive_conflict_reason)

  if [[ -s "$conflict_tmp" ]]; then
    pr_render_grouped_by_category "$conflict_tmp" "conflict" "$conflict_issues_file"
    directive_conflict_count="${#issue_directive_conflict_reason[@]}"
  fi

  : >"$directive_resolution_tmp"
  while IFS= read -r issue_key; do
    [[ -z "$issue_key" ]] && continue
    issue_number="${issue_key//#/}"
    directive_prefix="$(pr_pipeline_directive_prefix_for_issue "$issue_key")"
    echo "${issue_number}|${issue_category[$issue_key]:-Unknown}|${directive_prefix}${issue_directive_resolution[$issue_key]}" >>"$directive_resolution_tmp"
  done < <(pr_pipeline_sorted_issue_keys_from_assoc issue_directive_resolution)

  if [[ -s "$directive_resolution_tmp" ]]; then
    pr_render_grouped_by_category "$directive_resolution_tmp" "directive" "$directive_resolution_tmp.resolved"
    mv "$directive_resolution_tmp.resolved" "$directive_resolution_tmp"
  fi
}
