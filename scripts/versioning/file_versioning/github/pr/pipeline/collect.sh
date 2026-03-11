#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Pipeline issue collection helpers.

pr_pipeline_mark_breaking_from_text() {
  local text="$1"

  if [[ -z "$text" ]]; then
    return
  fi

  if pr_text_indicates_breaking "$text"; then
    breaking_detected=1
  fi
}

pr_pipeline_is_breaking_from_labels() {
  local labels_raw="${1:-}"
  pr_labels_indicate_breaking "$labels_raw"
}

pr_pipeline_parse_body_context_payload() {
  local payload="$1"
  local out_title_var="$2"
  local out_body_var="$3"
  local out_labels_var="$4"
  local payload_tail
  local -n out_title_ref="$out_title_var"
  local -n out_body_ref="$out_body_var"
  local -n out_labels_ref="$out_labels_var"

  out_title_ref="${payload%%$'\x1f'*}"
  payload_tail="${payload#*$'\x1f'}"
  out_body_ref="${payload_tail%%$'\x1f'*}"
  out_labels_ref="${payload_tail#*$'\x1f'}"
}

pr_pipeline_load_pr_body_context() {
  local pr_ref="$1"
  local pr_number="$2"
  local pr_view_json
  local context_payload
  local pr_labels_raw
  local pr_title
  local pr_body

  if [[ "$dry_run" == "true" && "$online_enrich" != "true" ]]; then
    pr_title="${pr_title_hint[$pr_ref]:-PR #${pr_number}}"
    pr_body=""
    printf "%s\x1f%s" "$pr_title" "$pr_body"
    return
  fi

  context_payload="$(github_pr_body_context "" "$pr_number" || true)"
  if [[ "$context_payload" == *$'\x1f'* ]]; then
    pr_pipeline_parse_body_context_payload "$context_payload" pr_title pr_body pr_labels_raw
  else
    pr_title="$(github_pr_field "" "$pr_number" "title" 2>/dev/null || true)"
    pr_body="$(github_pr_field "" "$pr_number" "body" 2>/dev/null || true)"

    # Keep optional-context gh reads for debug visibility in pipeline logs.
    if [[ -z "$pr_title" ]]; then
      pr_title="$(pr_gh_optional "read PR ${pr_ref} title" pr view "$pr_number" --json title -q '.title // ""')"
    fi
    if [[ -z "$pr_body" ]]; then
      pr_body="$(pr_gh_optional "read PR ${pr_ref} body" pr view "$pr_number" --json body -q '.body // ""')"
    fi
    pr_labels_raw="$(pr_gh_optional "read PR ${pr_ref} labels" pr view "$pr_number" --json labels -q '.labels // [] | map(.name) | join("||")')"

    # Keep legacy combined read as compatibility fallback for older gh mocks/wrappers.
    if [[ -z "$pr_title" && -z "$pr_body" && -z "$pr_labels_raw" ]]; then
      pr_view_json="$(pr_gh_optional "read PR ${pr_ref}" pr view "$pr_number" --json title,body,labels)"
    fi

    if [[ -n "$pr_view_json" ]]; then
      [[ -n "$pr_title" ]] || pr_title="$(echo "$pr_view_json" | jq -r '.title // ""')"
      [[ -n "$pr_body" ]] || pr_body="$(echo "$pr_view_json" | jq -r '.body // ""')"
      [[ -n "$pr_labels_raw" ]] || pr_labels_raw="$(echo "$pr_view_json" | jq -r '.labels // [] | map(.name) | join("||")')"
    fi

    if [[ -z "$pr_title" && -z "$pr_body" ]]; then
      if [[ "$online_enrich" == "true" ]]; then
        pr_enrich_failed=$((pr_enrich_failed + 1))
        pr_debug_log "enrich_fallback: failed to read PR ${pr_ref} title/body"
      fi
      printf "%s\x1f%s" "" ""
      return
    fi
  fi
  if pr_pipeline_is_breaking_from_labels "$pr_labels_raw"; then
    breaking_detected=1
  fi

  printf "%s\x1f%s" "$pr_title" "$pr_body"
}

pr_pipeline_collect_from_single_pr_body() {
  local pr_ref="$1"
  local pr_number="$2"
  local pr_title="$3"
  local pr_body="$4"
  local pr_category

  if [[ -z "$pr_title" ]]; then
    pr_title="${pr_title_hint[$pr_ref]:-PR #${pr_number}}"
  fi
  pr_pipeline_mark_breaking_from_text "$pr_title"

  pr_category="$(classify_pr "$pr_ref" "$pr_title")"
  pr_count=$((pr_count + 1))

  if [[ -z "$pr_body" ]]; then
    return
  fi

  pr_pipeline_mark_breaking_from_text "$pr_body"
  if [[ "$pr_category" == "Synchronization" ]]; then
    pr_debug_log "skip_issue_directives(pr ${pr_ref}): category=Synchronization"
    return
  fi

  pr_pipeline_apply_issue_directives_from_text \
    "$pr_body" \
    "$pr_category" \
    "pr ${pr_ref}"
}

pr_pipeline_collect_issues_from_pr_bodies() {
  local pr_ref pr_number
  local pr_payload
  local pr_title
  local pr_body

  if [[ -s "$extracted_prs_file" ]]; then
    while read -r pr_ref; do
      [[ -z "$pr_ref" ]] && continue
      pr_number="${pr_ref//#/}"
      pr_payload="$(pr_pipeline_load_pr_body_context "$pr_ref" "$pr_number")"
      pr_title="${pr_payload%%$'\x1f'*}"
      pr_body="${pr_payload#*$'\x1f'}"

      pr_pipeline_collect_from_single_pr_body "$pr_ref" "$pr_number" "$pr_title" "$pr_body"
    done <"$extracted_prs_file"
  fi
}

pr_pipeline_apply_explicit_directive_decisions() {
  local records_var_name="$1"
  local -n records_ref="$records_var_name"
  local record record_type field_a field_b
  local -A seen_decisions=()

  for record in "${records_ref[@]}"; do
    IFS='|' read -r record_type field_a field_b <<<"$record"
    [[ "$record_type" == "DEC" ]] || continue
    [[ -z "$field_a" || -z "$field_b" ]] && continue
    [[ "$field_b" == "close" || "$field_b" == "reopen" ]] || continue

    if [[ -n "${seen_decisions[${field_a} | ${field_b}]:-}" ]]; then
      continue
    fi
    seen_decisions["${field_a}|${field_b}"]=1
    issue_directive_decision["$field_a"]="$field_b"
  done
}

pr_pipeline_apply_inferred_directive_decisions() {
  local records_var_name="$1"
  local -n records_ref="$records_var_name"
  local record record_type field_a field_b decision

  for record in "${records_ref[@]}"; do
    IFS='|' read -r record_type field_a field_b <<<"$record"
    [[ "$record_type" == "EV" ]] || continue
    [[ -z "$field_a" || -z "$field_b" ]] && continue

    case "$field_a" in
    Closes) decision="close" ;;
    Reopen) decision="reopen" ;;
    *) continue ;;
    esac

    if [[ -z "${issue_directive_decision[$field_b]:-}" ]]; then
      if [[ -n "${issue_inferred_decision[$field_b]:-}" && "${issue_inferred_decision[$field_b]}" != "$decision" ]]; then
        if [[ "$decision" == "reopen" || "${issue_inferred_decision[$field_b]}" == "reopen" ]]; then
          issue_inferred_decision["$field_b"]="reopen"
        else
          issue_inferred_decision["$field_b"]="conflict"
        fi
      elif [[ -z "${issue_inferred_decision[$field_b]:-}" ]]; then
        issue_inferred_decision["$field_b"]="$decision"
      fi
    fi
  done
}

pr_pipeline_apply_issue_and_duplicate_entries() {
  local records_var_name="$1"
  local category="$2"
  local debug_context="$3"
  local -n records_ref="$records_var_name"
  local record record_type field_a field_b
  local -A seen_reopen_refs=()
  local -A seen_close_refs=()
  local -A seen_duplicates=()

  for record in "${records_ref[@]}"; do
    IFS='|' read -r record_type field_a field_b <<<"$record"
    [[ -z "$record_type" ]] && continue

    case "$record_type" in
    EV)
      [[ -z "$field_a" || -z "$field_b" ]] && continue

      # Preserve source chronology for inferred directive conflict resolution.
      if [[ "$field_a" == "Closes" || "$field_a" == "Reopen" ]]; then
        if [[ "$field_a" == "Closes" ]]; then
          if [[ -z "${seen_close_refs[$field_b]:-}" ]]; then
            seen_close_refs["$field_b"]=1
            pr_debug_log "parsed_issue_ref(${debug_context}): ${field_a}|${field_b}"
            pr_add_issue_entry "$field_a" "$field_b" "$category"
          fi
        else
          if [[ -z "${seen_reopen_refs[$field_b]:-}" ]]; then
            seen_reopen_refs["$field_b"]=1
            pr_mark_reopen_issue "$field_b" "$category"
          fi
        fi
      fi
      ;;
    DUP)
      [[ -z "$field_a" || -z "$field_b" ]] && continue
      if [[ -n "${seen_duplicates[${field_a} | ${field_b}]:-}" ]]; then
        continue
      fi
      seen_duplicates["${field_a}|${field_b}"]=1
      pr_add_duplicate_entry "$field_a" "$field_b"
      ;;
    esac
  done
}

pr_pipeline_apply_issue_directives_via_va() {
  local text="$1"
  local category="$2"
  local debug_context="$3"
  local va_output
  local record_type field_a field_b

  if ! command -v va_exec >/dev/null 2>&1; then
    return 1
  fi

  va_output="$(printf '%s' "$text" | va_exec pr directives-apply --stdin 2>/dev/null)" || {
    return 1
  }

  while IFS= read -r record; do
    [[ -z "$record" ]] && continue
    IFS='|' read -r record_type field_a field_b <<<"$record"
    [[ -z "$record_type" ]] && continue
    case "$record_type" in
    SET_DEC)
      [[ -z "$field_a" || -z "$field_b" ]] && continue
      [[ "$field_b" == "close" || "$field_b" == "reopen" ]] || continue
      issue_directive_decision["$field_a"]="$field_b"
      ;;
    SET_INF)
      [[ -z "$field_a" || -z "$field_b" ]] && continue
      issue_inferred_decision["$field_a"]="$field_b"
      ;;
    ADD_CLOSE)
      [[ -z "$field_a" ]] && continue
      pr_debug_log "parsed_issue_ref(${debug_context}): Closes|${field_a}"
      pr_add_issue_entry "Closes" "$field_a" "$category"
      ;;
    ADD_REOPEN)
      [[ -z "$field_a" ]] && continue
      pr_mark_reopen_issue "$field_a" "$category"
      ;;
    ADD_DUP)
      [[ -z "$field_a" || -z "$field_b" ]] && continue
      pr_add_duplicate_entry "$field_a" "$field_b"
      ;;
    esac
  done <<<"$va_output"

  return 0
}

pr_pipeline_apply_issue_directives_from_text() {
  local text="$1"
  local category="$2"
  local debug_context="$3"
  local -a records=()

  [[ -z "$text" ]] && return

  if pr_pipeline_apply_issue_directives_via_va "$text" "$category" "$debug_context"; then
    return
  fi

  mapfile -t records < <(parse_issue_directive_records_from_text "$text")

  # Pass 1: explicit directive decisions.
  pr_pipeline_apply_explicit_directive_decisions records

  # Pass 2: inferred decisions (chronological EV stream), honoring explicit decisions.
  pr_pipeline_apply_inferred_directive_decisions records

  # Pass 3: apply deduped issue actions and duplicate links.
  pr_pipeline_apply_issue_and_duplicate_entries records "$category" "$debug_context"
}

pr_pipeline_collect_issues_from_commits_and_main_pr() {
  local dry_commit_messages main_pr_body refresh_compare_commit_messages

  if [[ "$dry_run" == "true" ]]; then
    dry_commit_messages="$dry_compare_commit_messages"
    if [[ -n "$dry_commit_messages" ]]; then
      pr_pipeline_mark_breaking_from_text "$dry_commit_messages"
      pr_pipeline_apply_issue_directives_from_text \
        "$dry_commit_messages" \
        "Mixed" \
        "dry commits"
    fi
  fi

  if [[ "$dry_run" == "false" ]]; then
    local main_pr_body_payload
    main_pr_body_payload="$(github_pr_body_context "" "$main_pr_number" || true)"
    if [[ "$main_pr_body_payload" == *$'\x1f'* ]]; then
      local _main_tail
      _main_tail="${main_pr_body_payload#*$'\x1f'}"
      main_pr_body="${_main_tail%%$'\x1f'*}"
    else
      main_pr_body="$(pr_get_pr_body "$main_pr_number" "read PR #${main_pr_number} body")"
    fi
    if [[ -n "$main_pr_body" ]]; then
      pr_pipeline_mark_breaking_from_text "$main_pr_body"
      pr_pipeline_apply_issue_directives_from_text \
        "$main_pr_body" \
        "Mixed" \
        "main pr"
    fi

    if [[ -n "$auto_edit_pr_number" ]]; then
      refresh_compare_commit_messages="$(pr_load_compare_commit_messages "$base_ref_git" "$head_ref_git" || true)"
      # Avoid redundant reprocessing if both payloads are identical.
      if [[ -n "$refresh_compare_commit_messages" && "$refresh_compare_commit_messages" != "${main_pr_body:-}" ]]; then
        pr_pipeline_apply_issue_directives_from_text \
          "$refresh_compare_commit_messages" \
          "Mixed" \
          "refresh commits"
      fi
    fi
  fi
}
