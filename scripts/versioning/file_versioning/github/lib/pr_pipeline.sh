#!/usr/bin/env bash

# Pipeline orchestration helpers for generate_pr_description.sh.

pr_pipeline_init_artifacts_and_state() {
  if [[ "$keep_artifacts" == "true" ]]; then
    extracted_prs_file="extracted_prs.txt"
    resolved_issues_file="resolved_issues.txt"
    reopened_issues_file="reopened_issues.txt"
    conflict_issues_file="issue_directive_conflicts.txt"
  else
    extracted_prs_file="$(mktemp)"
    resolved_issues_file="$(mktemp)"
    reopened_issues_file="$(mktemp)"
    conflict_issues_file="$(mktemp)"
  fi

  features_tmp="$(mktemp)"
  bugs_tmp="$(mktemp)"
  refactors_tmp="$(mktemp)"
  sync_tmp="$(mktemp)"
  issues_tmp="$(mktemp)"
  reopen_tmp="$(mktemp)"
  conflict_tmp="$(mktemp)"
  directive_resolution_tmp="$(mktemp)"

  declare -gA pr_title_hint
  online_enrich="false"
  pr_enrich_failed=0
  breaking_detected=0
  ci_status="UNKNOWN"
  breaking_scope_crates=""
  breaking_scope_commits=""
  pr_created_successfully="false"
  dry_compare_commit_messages=""
  dry_compare_commit_headlines=""
}

pr_pipeline_check_dependencies() {
  local need_jq="false"

  has_gh="false"
  if command -v gh >/dev/null 2>&1; then
    has_gh="true"
  fi

  if [[ "$has_gh" != "true" ]]; then
    echo "Error: command 'gh' not found." >&2
    exit "$E_DEPENDENCY"
  fi

  if [[ "$has_gh" == "true" ]]; then
    need_jq="true"
  fi
  if [[ "$dry_run" == "false" || "$create_pr" == "true" ]]; then
    need_jq="true"
  fi
  if [[ "$need_jq" == "true" ]] && ! command -v jq >/dev/null 2>&1; then
    echo "Error: command 'jq' not found." >&2
    exit "$E_DEPENDENCY"
  fi
}

pr_pipeline_resolve_refs_and_modes() {
  if [[ "$dry_run" == "true" ]]; then
    if ! command -v git >/dev/null 2>&1; then
      echo "Error: command 'git' not found." >&2
      exit "$E_GIT"
    fi
    if [[ -z "$head_ref" ]]; then
      current_branch="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || true)"
      head_ref="$current_branch"
    fi
    if [[ -z "$base_ref" ]]; then
      base_ref="dev"
    fi
    base_ref="$(pr_preferred_base_ref_with_origin "$base_ref")"
    if [[ -z "$head_ref" ]]; then
      echo "Error: unable to determine head branch in --dry-run mode." >&2
      exit "$E_GIT"
    fi
  else
    base_ref="$(pr_gh_optional "read base branch for PR #${main_pr_number}" pr view "$main_pr_number" --json baseRefName -q '.baseRefName')"
    head_ref="$(pr_gh_optional "read head branch for PR #${main_pr_number}" pr view "$main_pr_number" --json headRefName -q '.headRefName')"
    if [[ -z "$base_ref" ]]; then
      pr_warn_optional "PR #${main_pr_number} base branch unavailable; defaulting to dev (expected dev base)."
      base_ref="dev"
    fi
    if [[ -z "$head_ref" ]]; then
      pr_warn_optional "PR #${main_pr_number} head branch unavailable; defaulting to dev."
      head_ref="dev"
    fi
  fi

  base_ref_display="$(pr_normalize_branch_display_ref "$base_ref")"
  head_ref_display="$(pr_normalize_branch_display_ref "$head_ref")"
  base_ref_git="$base_ref"
  head_ref_git="$head_ref"

  if ! git rev-parse --verify --quiet "${base_ref_git}^{commit}" >/dev/null 2>&1; then
    base_ref_git="$base_ref_display"
  fi
  if ! git rev-parse --verify --quiet "${head_ref_git}^{commit}" >/dev/null 2>&1; then
    head_ref_git="$head_ref_display"
  fi

  if [[ "$dry_run" == "true" && "$create_pr" == "true" ]]; then
    online_enrich="true"
  fi
}

pr_pipeline_extract_pr_refs() {
  : >"$extracted_prs_file"
  : >"$resolved_issues_file"
  : >"$reopened_issues_file"
  : >"$conflict_issues_file"

  if [[ "$dry_run" == "true" ]]; then
    pr_load_dry_compare_commits_into_globals
    if ! pr_extract_child_prs_dry; then
      echo "Warning: unable to extract PRs from compare ${base_ref_git}...${head_ref_git}." >&2
    fi
  else
    if ! pr_extract_child_prs; then
      echo "Warning: unable to fetch commits for PR #${main_pr_number} (API unavailable or PR not found)." >&2
    fi
  fi
}

pr_pipeline_init_issue_tracking() {
  declare -gA seen_issue
  declare -gA issue_category
  declare -gA issue_action
  declare -gA issue_neutralization_reason
  declare -gA issue_reopen_detected
  declare -gA seen_reopen_issue
  declare -gA reopen_issue_category
  declare -gA issue_directive_decision
  declare -gA issue_inferred_decision
  declare -gA issue_directive_conflict_reason
  declare -gA issue_directive_conflict_action
  declare -gA issue_directive_resolution
  declare -gA issue_directive_final_action
  declare -gA pr_ref_cache
  declare -gA duplicate_targets
  declare -gA issue_non_compliance_reason_cache
  declare -gA issue_non_compliance_skip
  declare -gA issue_non_compliance_action

  pr_count=0
  issue_count=0
  reopen_issue_count=0
  directive_conflict_count=0
  neutralized_issue_count=0
}

pr_pipeline_collect_issues_from_pr_bodies() {
  local pr_ref pr_number pr_view_json pr_labels_raw pr_title pr_body pr_category

  if [[ -s "$extracted_prs_file" ]]; then
    while read -r pr_ref; do
      [[ -z "$pr_ref" ]] && continue
      pr_number="${pr_ref//#/}"
      pr_view_json=""
      pr_labels_raw=""

      if [[ "$dry_run" == "true" && "$online_enrich" != "true" ]]; then
        pr_title="${pr_title_hint[$pr_ref]:-PR #${pr_number}}"
        pr_body=""
      else
        pr_view_json="$(pr_gh_optional "read PR ${pr_ref}" pr view "$pr_number" --json title,body,labels)"
        if [[ -n "$pr_view_json" ]]; then
          pr_title="$(echo "$pr_view_json" | jq -r '.title // ""')"
          pr_body="$(echo "$pr_view_json" | jq -r '.body // ""')"
          pr_labels_raw="$(echo "$pr_view_json" | jq -r '.labels // [] | map(.name) | join("||")')"
          if [[ "$(echo "$pr_labels_raw" | tr '[:upper:]' '[:lower:]')" =~ (^|\|\|)breaking(\|\||$) ]]; then
            breaking_detected=1
          fi
        else
          pr_title=""
          pr_body=""
          if [[ "$online_enrich" == "true" ]]; then
            pr_enrich_failed=$((pr_enrich_failed + 1))
            pr_debug_log "enrich_fallback: failed to read PR ${pr_ref} via gh pr view"
          fi
        fi
      fi

      if [[ -z "$pr_title" ]]; then
        pr_title="${pr_title_hint[$pr_ref]:-PR #${pr_number}}"
      fi
      if pr_text_indicates_breaking "$pr_title"; then
        breaking_detected=1
      fi

      pr_category="$(classify_pr "$pr_ref" "$pr_title")"
      pr_count=$((pr_count + 1))

      if [[ -n "$pr_body" ]]; then
        if pr_text_indicates_breaking "$pr_body"; then
          breaking_detected=1
        fi
        if [[ "$pr_category" != "Synchronization" ]]; then
          pr_pipeline_apply_issue_directives_from_text \
            "$pr_body" \
            "$pr_category" \
            "parse_pr_body_closing_issue_refs_from_text" \
            "pr ${pr_ref}"
        else
          pr_debug_log "skip_issue_directives(pr ${pr_ref}): category=Synchronization"
        fi
      fi
    done <"$extracted_prs_file"
  fi
}

pr_pipeline_apply_issue_directives_from_text() {
  local text="$1"
  local category="$2"
  local closing_parser_fn="$3"
  local debug_context="$4"
  local issue_key action duplicate_issue canonical_issue

  [[ -z "$text" ]] && return

  pr_mark_inferred_decisions_from_text "$text"
  pr_mark_directive_decisions_from_text "$text"
  while IFS='|' read -r _ issue_key; do
    pr_mark_reopen_issue "$issue_key" "$category"
  done < <(parse_reopen_issue_refs_from_text "$text")

  while IFS='|' read -r action issue_key; do
    pr_debug_log "parsed_issue_ref(${debug_context}): ${action}|${issue_key}"
    pr_add_issue_entry "$action" "$issue_key" "$category"
  done < <("$closing_parser_fn" "$text")

  while IFS='|' read -r duplicate_issue canonical_issue; do
    pr_add_duplicate_entry "$duplicate_issue" "$canonical_issue"
  done < <(parse_duplicate_refs_from_text "$text")
}

pr_pipeline_collect_issues_from_commits_and_main_pr() {
  local dry_commit_messages main_pr_body refresh_compare_commit_messages

  if [[ "$dry_run" == "true" ]]; then
    dry_commit_messages="$dry_compare_commit_messages"
    if [[ -n "$dry_commit_messages" ]]; then
      if pr_text_indicates_breaking "$dry_commit_messages"; then
        breaking_detected=1
      fi
      pr_pipeline_apply_issue_directives_from_text \
        "$dry_commit_messages" \
        "Mixed" \
        "parse_closing_issue_refs_from_text" \
        "dry commits"
    fi
  fi

  if [[ "$dry_run" == "false" ]]; then
    main_pr_body="$(pr_gh_optional "read PR #${main_pr_number} body" pr view "$main_pr_number" --json body -q '.body')"
    if [[ -n "$main_pr_body" ]]; then
      if pr_text_indicates_breaking "$main_pr_body"; then
        breaking_detected=1
      fi
      pr_pipeline_apply_issue_directives_from_text \
        "$main_pr_body" \
        "Mixed" \
        "parse_pr_body_closing_issue_refs_from_text" \
        "main pr"
    fi

    if [[ -n "$auto_edit_pr_number" ]]; then
      refresh_compare_commit_messages="$(pr_load_compare_commit_messages "$base_ref_git" "$head_ref_git" || true)"
      # Avoid redundant reprocessing if both payloads are identical.
      if [[ -n "$refresh_compare_commit_messages" && "$refresh_compare_commit_messages" != "${main_pr_body:-}" ]]; then
        pr_pipeline_apply_issue_directives_from_text \
          "$refresh_compare_commit_messages" \
          "Mixed" \
          "parse_closing_issue_refs_from_text" \
          "refresh commits"
      fi
    fi
  fi
}

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
