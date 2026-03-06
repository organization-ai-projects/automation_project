#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154


# PR body output/publication helpers.

pr_body_emit_generated_output() {
  if [[ "$auto_mode" != "true" && -z "$auto_edit_pr_number" && "$write_body_to_file" == "true" ]]; then
    printf "%s\n" "$body_content" >"$output_file"
    echo "Generated file: $output_file"
  else
    if [[ "$auto_mode" == "true" ]]; then
      echo "PR description generated in memory (--auto mode)."
    elif [[ -n "$auto_edit_pr_number" ]]; then
      echo "PR description generated in memory (--auto-edit mode)."
    else
      echo "PR description generated in memory (--dry-run default output)."
    fi
  fi

  if [[ "$keep_artifacts" == "true" ]]; then
    echo "Extracted PRs: $extracted_prs_file"
    echo "Resolved issues: $resolved_issues_file"
    echo "Reopened issues: $reopened_issues_file"
    echo "Directive conflicts: $conflict_issues_file"
  fi
}

pr_body_handle_create_pr() {
  [[ "$create_pr" != "true" ]] && return

  if [[ "$online_enrich" == "true" && "$pr_enrich_failed" -gt 0 && "$allow_partial_create" != "true" ]]; then
    echo "Error: partial GitHub enrichment (${pr_enrich_failed} PRs unread)." >&2
    echo "The body may be incomplete. Fix network/auth and retry, or use --allow-partial-create." >&2
    exit "$E_PARTIAL"
  fi

  default_title="$(build_dynamic_pr_title)"

  if pr_body_should_show_create_summary; then
    echo
    echo "Dry-run complete."
    echo "Base: ${base_ref_display}"
    echo "Head: ${head_ref_display}"
    if [[ "$auto_mode" != "true" ]]; then
      echo "Body file: ${output_file}"
    else
      echo "Body: in-memory"
    fi
  fi

  if pr_body_confirm_with_policy "Create PR now with generated body? [y/N] " "auto-yes"; then
    if [[ "$auto_mode" == "true" ]]; then
      pr_url="$(gh pr create --base "$base_ref_display" --head "$head_ref_display" --title "$default_title" --body "$body_content" --label "pull-request")"
    else
      pr_url="$(gh pr create --base "$base_ref_display" --head "$head_ref_display" --title "$default_title" --body-file "$output_file" --label "pull-request")"
    fi
    pr_created_successfully="true"
    echo "PR created: $pr_url"
  else
    echo "PR creation skipped."
  fi
}

pr_body_handle_auto_edit_pr() {
  [[ -z "$auto_edit_pr_number" ]] && return

  if [[ "$assume_yes" != "true" ]] && pr_is_human_interactive_terminal; then
    echo
    echo "Body generated for update."
  fi

  if pr_body_confirm_with_policy \
    "Update PR #${auto_edit_pr_number} now with generated body? [y/N] " \
    "require-yes" \
    "--auto-edit requires --yes in non-interactive context."; then
    repo_name_with_owner="$(pr_get_repo_name_with_owner)"
    if [[ -z "$repo_name_with_owner" ]]; then
      echo "Error: unable to determine GitHub repository for --auto-edit." >&2
      exit "$E_DEPENDENCY"
    fi

    current_pr_body="$(pr_gh_optional "read current PR #${auto_edit_pr_number} body before update" pr view "$auto_edit_pr_number" --json body -q '.body // ""')"
    if [[ "$validation_only" != "true" ]]; then
      body_content="$(pr_build_sectional_auto_edit_body "$current_pr_body" "$body_content")"
    fi

    if [[ "$current_pr_body" == "$body_content" ]]; then
      echo "PR unchanged: #${auto_edit_pr_number}"
      return
    fi

    gh api -X PATCH "repos/${repo_name_with_owner}/pulls/${auto_edit_pr_number}" \
      --raw-field body="$body_content" >/dev/null
    echo "PR updated: #${auto_edit_pr_number}"
  else
    echo "PR update skipped."
  fi
}

pr_body_finalize_exit_status() {
  [[ "$dry_run" != "true" ]] && return
  [[ "$create_pr" != "true" ]] && return
  [[ "$pr_created_successfully" == "true" ]] && return
  [[ -s "$extracted_prs_file" ]] && return
  exit "$E_NO_DATA"
}
