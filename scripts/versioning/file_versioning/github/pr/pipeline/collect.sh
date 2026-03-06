#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Pipeline issue collection helpers.

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
