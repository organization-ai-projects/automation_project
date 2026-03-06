#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Pipeline issue-outcome rendering helpers.

pr_pipeline_render_issue_outcomes_files() {
  : >"$issues_tmp"
  for issue_key in "${!seen_issue[@]}"; do
    [[ -n "${issue_directive_conflict_reason[$issue_key]:-}" ]] && continue
    [[ -n "${issue_directive_resolution[$issue_key]:-}" ]] && continue
    issue_number="${issue_key//#/}"
    echo "${issue_number}|${issue_category[$issue_key]}|${issue_action[$issue_key]}|${issue_key}" >>"$issues_tmp"
  done

  if [[ -s "$issues_tmp" ]]; then
    pr_render_grouped_by_category "$issues_tmp" "resolved" "$resolved_issues_file"
    issue_count="$(wc -l <"$issues_tmp" | tr -d ' ')"
  fi

  : >"$reopen_tmp"
  for issue_key in "${!seen_reopen_issue[@]}"; do
    [[ -n "${issue_directive_conflict_reason[$issue_key]:-}" ]] && continue
    [[ -n "${issue_directive_resolution[$issue_key]:-}" ]] && continue
    issue_number="${issue_key//#/}"
    echo "${issue_number}|${reopen_issue_category[$issue_key]:-Unknown}|${issue_key}" >>"$reopen_tmp"
  done

  if [[ -s "$reopen_tmp" ]]; then
    pr_render_grouped_by_category "$reopen_tmp" "reopen" "$reopened_issues_file"
    reopen_issue_count="${#seen_reopen_issue[@]}"
  fi

  : >"$conflict_tmp"
  for issue_key in "${!issue_directive_conflict_reason[@]}"; do
    issue_number="${issue_key//#/}"
    echo "${issue_number}|${issue_category[$issue_key]:-Unknown}|${issue_key}|${issue_directive_conflict_reason[$issue_key]}" >>"$conflict_tmp"
  done

  if [[ -s "$conflict_tmp" ]]; then
    pr_render_grouped_by_category "$conflict_tmp" "conflict" "$conflict_issues_file"
    directive_conflict_count="${#issue_directive_conflict_reason[@]}"
  fi

  : >"$directive_resolution_tmp"
  for issue_key in "${!issue_directive_resolution[@]}"; do
    issue_number="${issue_key//#/}"
    directive_action="${issue_directive_final_action[$issue_key]:-}"
    directive_prefix=""
    case "$directive_action" in
    close) directive_prefix="Closes ${issue_key} - " ;;
    reopen) directive_prefix="Reopen ${issue_key} - " ;;
    *) directive_prefix="${issue_key} - " ;;
    esac
    echo "${issue_number}|${issue_category[$issue_key]:-Unknown}|${directive_prefix}${issue_directive_resolution[$issue_key]}" >>"$directive_resolution_tmp"
  done

  if [[ -s "$directive_resolution_tmp" ]]; then
    pr_render_grouped_by_category "$directive_resolution_tmp" "directive" "$directive_resolution_tmp.resolved"
    mv "$directive_resolution_tmp.resolved" "$directive_resolution_tmp"
  fi
}
