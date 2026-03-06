#!/usr/bin/env bash

# PR body/build/publish helpers for generate_pr_description.sh.

pr_body_confirm_with_policy() {
  local prompt="$1"
  local noninteractive_policy="$2" # auto-yes | require-yes
  local noninteractive_error="${3:-}"
  local answer

  if [[ "$assume_yes" == "true" ]]; then
    return 0
  fi

  if ! pr_is_human_interactive_terminal; then
    if [[ "$noninteractive_policy" == "auto-yes" ]]; then
      return 0
    fi
    pr_usage_error "$noninteractive_error"
  fi

  read -r -p "$prompt" answer
  case "$answer" in
  y | Y | yes | YES) return 0 ;;
  *) return 1 ;;
  esac
}

pr_body_compute_validation_state() {
  pr_compute_ci_status
  pr_compute_breaking_scope

  ci_status_with_symbol="$ci_status"
  case "$ci_status" in
  PASS) ci_status_with_symbol="PASS ✅" ;;
  FAIL) ci_status_with_symbol="FAIL ❌" ;;
  RUNNING) ci_status_with_symbol="RUNNING ⏳" ;;
  *) ci_status_with_symbol="UNKNOWN ⚪" ;;
  esac
}

pr_body_build_description_section() {
  echo "### Description"
  echo ""
  echo "This pull request merges the \`${head_ref_display}\` branch into \`${base_ref_display}\` and summarizes merged pull requests and resolved issues."
  echo ""
}

pr_body_build_validation_section() {
  echo "### Validation Gate"
  echo ""
  echo "- CI: ${ci_status_with_symbol}"
  if [[ "$breaking_detected" -eq 1 ]]; then
    echo "- Breaking change"
  else
    echo "- No breaking change"
  fi

  if [[ "$breaking_detected" -eq 1 ]]; then
    echo "- Breaking scope:"
    if [[ -n "$breaking_scope_crates" ]]; then
      echo "  - crate(s): ${breaking_scope_crates}"
    else
      echo "  - crate(s): unknown"
    fi
    if [[ -n "$breaking_scope_commits" ]]; then
      echo "  - source commit(s): ${breaking_scope_commits}"
    else
      echo "  - source commit(s): unknown"
    fi
  fi
  echo ""
}

pr_body_build_issue_outcomes_section() {
  echo "### Issue Outcomes"
  echo ""

  if [[ ! -s "$resolved_issues_file" && ! -s "$reopened_issues_file" && ! -s "$directive_resolution_tmp" && ! -s "$conflict_issues_file" ]]; then
    echo "- No issues processed in this PR."
    echo ""
    return
  fi

  echo "#### Category 1: Issues Without Conflicts"
  echo ""
  echo "##### Closes/Fixes"
  echo ""
  if [[ -s "$resolved_issues_file" ]]; then
    cat "$resolved_issues_file"
  else
    echo "- No resolved issues detected via GitHub references or PR body keywords."
  fi
  echo ""

  echo "##### Reopened"
  echo ""
  if [[ -s "$reopened_issues_file" ]]; then
    cat "$reopened_issues_file"
  else
    echo "- No reopened issues detected."
  fi
  echo ""

  echo "#### Category 2: Issues With Conflicts"
  echo ""
  echo "##### Auto-resolved"
  echo ""
  if [[ -s "$directive_resolution_tmp" ]]; then
    cat "$directive_resolution_tmp"
  else
    echo "- No auto-resolved directive conflicts."
  fi
  echo ""

  echo "##### Not resolved"
  echo ""
  if [[ -s "$conflict_issues_file" ]]; then
    cat "$conflict_issues_file"
  else
    echo "- No unresolved directive conflicts."
  fi
  echo ""
}

pr_body_build_key_changes_section() {
  local key_changes_found=0

  echo "### Key Changes"
  echo ""

  if [[ -s "$sync_tmp" ]]; then
    key_changes_found=1
    echo "#### Synchronization"
    echo ""
    write_section_from_file "$sync_tmp"
    echo ""
  fi
  if [[ -s "$features_tmp" ]]; then
    key_changes_found=1
    echo "#### Features"
    echo ""
    write_section_from_file "$features_tmp"
    echo ""
  fi
  if [[ -s "$bugs_tmp" ]]; then
    key_changes_found=1
    echo "#### Bug Fixes"
    echo ""
    write_section_from_file "$bugs_tmp"
    echo ""
  fi
  if [[ -s "$refactors_tmp" ]]; then
    key_changes_found=1
    echo "#### Refactoring"
    echo ""
    write_section_from_file "$refactors_tmp"
    echo ""
  fi
  if [[ "$key_changes_found" -eq 0 ]]; then
    echo "- No significant items detected."
    echo ""
  fi

  echo "#### Change Footprint"
  echo ""
  pr_emit_change_footprint "${base_ref_git}..${head_ref_git}"
  echo ""
}

pr_body_build_content() {
  pr_body_compute_validation_state

  body_content="$({
    pr_body_build_description_section
    pr_body_build_validation_section
    pr_body_build_issue_outcomes_section
    pr_body_build_key_changes_section
  })"
}

pr_extract_top_level_section_from_body() {
  local body="$1"
  local heading="$2"

  awk -v heading="$heading" '
    BEGIN {
      in_section = 0
      found = 0
    }
    {
      if (!in_section && $0 == heading) {
        in_section = 1
        found = 1
      }

      if (in_section) {
        if ($0 ~ /^### / && $0 != heading) {
          exit
        }
        print $0
      }
    }
    END {
      if (!found) {
        exit 1
      }
    }
  ' <<<"$body"
}

pr_body_has_top_level_heading() {
  local body="$1"
  local heading="$2"
  grep -Fxq "$heading" <<<"$body"
}

pr_remove_top_level_section_from_body() {
  local original_body="$1"
  local heading="$2"

  awk -v heading="$heading" '
    BEGIN {
      in_section = 0
    }
    {
      if ($0 == heading) {
        in_section = 1
        next
      }

      if (in_section) {
        if ($0 ~ /^### /) {
          in_section = 0
          print $0
        }
        next
      }

      print $0
    }
  ' <<<"$original_body"
}

pr_replace_top_level_section_in_body() {
  local original_body="$1"
  local heading="$2"
  local replacement="$3"
  local repl_file
  repl_file="$(mktemp)"
  printf "%s\n" "$replacement" >"$repl_file"

  awk -v heading="$heading" -v repl_file="$repl_file" '
    function print_replacement() {
      while ((getline line < repl_file) > 0) {
        print line
      }
      close(repl_file)
    }
    BEGIN {
      in_section = 0
      replaced = 0
    }
    {
      if ($0 == heading) {
        print_replacement()
        replaced = 1
        in_section = 1
        next
      }

      if (in_section) {
        if ($0 ~ /^### /) {
          in_section = 0
          print ""
          print $0
        }
        next
      }

      print $0
    }
    END {
      if (!replaced) {
        if (NR > 0) {
          print ""
        }
        print_replacement()
      }
    }
  ' <<<"$original_body"

  rm -f "$repl_file"
}

pr_build_sectional_auto_edit_body() {
  local current_body="$1"
  local generated_body="$2"
  local merged_body heading section target_heading

  merged_body="$current_body"
  for heading in "### Description" "### Validation Gate" "### Issue Outcomes" "### Key Changes"; do
    section="$(pr_extract_top_level_section_from_body "$generated_body" "$heading" || true)"
    [[ -z "$section" ]] && continue
    target_heading="$heading"
    if [[ "$heading" == "### Validation Gate" ]] \
      && ! pr_body_has_top_level_heading "$merged_body" "### Validation Gate" \
      && pr_body_has_top_level_heading "$merged_body" "### Validation Status"; then
      target_heading="### Validation Status"
    fi
    merged_body="$(pr_replace_top_level_section_in_body "$merged_body" "$target_heading" "$section")"
  done

  if pr_body_has_top_level_heading "$merged_body" "### Validation Gate" \
    && pr_body_has_top_level_heading "$merged_body" "### Validation Status"; then
    merged_body="$(pr_remove_top_level_section_from_body "$merged_body" "### Validation Status")"
  fi

  echo "$merged_body"
}

pr_body_apply_validation_only_if_needed() {
  [[ "$validation_only" != "true" || -z "$auto_edit_pr_number" ]] && return

  # Safety: allow validation-only refresh to run even if called before full body build.
  if [[ -z "${ci_status_with_symbol:-}" ]]; then
    pr_body_compute_validation_state
  fi

  current_pr_body="$(pr_gh_optional "read PR #${auto_edit_pr_number} body for validation-only update" pr view "$auto_edit_pr_number" --json body -q '.body')"
  if [[ -z "$current_pr_body" ]]; then
    echo "Error: unable to read current PR body for validation-only update." >&2
    exit "$E_PARTIAL"
  fi

  validation_gate_section="$(pr_build_validation_gate_section "$ci_status_with_symbol" "$breaking_detected" "$breaking_scope_crates" "$breaking_scope_commits")"
  body_content="$(pr_replace_validation_gate_in_body "$current_pr_body" "$validation_gate_section")"
}

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

pr_body_should_show_create_summary() {
  if [[ "$assume_yes" == "true" ]]; then
    return 1
  fi

  if [[ "$auto_mode" == "true" ]] && ! pr_is_human_interactive_terminal; then
    return 1
  fi

  return 0
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
