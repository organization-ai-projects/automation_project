#!/usr/bin/env bash
# shellcheck disable=SC2034,SC2178

neutralize_upsert_pr_comment() {
  local repo_name="$1"
  local pr_number="$2"
  local marker="$3"
  local body="$4"
  local comment_id

  comment_id="$({
    gh api "repos/${repo_name}/issues/${pr_number}/comments" --paginate
  } | jq -r --arg marker "$marker" '
      map(select((.body // "") | contains($marker)))
      | sort_by(.updated_at)
      | last
      | .id // empty
    ' 2>/dev/null || true)"

  if [[ -n "$comment_id" ]]; then
    gh api -X PATCH "repos/${repo_name}/issues/comments/${comment_id}" \
      -f body="$body" >/dev/null
  else
    gh api "repos/${repo_name}/issues/${pr_number}/comments" \
      -f body="$body" >/dev/null
  fi
}

neutralize_update_pr_body() {
  local repo_name="$1"
  local pr_number="$2"
  local body="$3"

  if command -v va_exec >/dev/null 2>&1; then
    if va_exec pr update-body --pr "$pr_number" --repo "$repo_name" --body "$body" >/dev/null; then
      return 0
    fi
  fi

  gh pr edit "$pr_number" -R "$repo_name" --body "$body" >/dev/null
}

neutralize_reason_for_issue_cached() {
  local issue_number="$1"
  local repo_name="$2"
  local cache_var_name="$3"
  local -n cache_ref="$cache_var_name"
  local cache_key="#${issue_number}"

  if [[ -v "cache_ref[$cache_key]" ]]; then
    echo "${cache_ref[$cache_key]}"
    return
  fi

  local reason
  reason="$(issue_fetch_non_compliance_reason "$issue_number" "$repo_name")"
  cache_ref["$cache_key"]="$reason"
  echo "$reason"
}

neutralize_valid_keyword_pattern() {
  local keyword_pattern="$1"
  [[ -n "$keyword_pattern" && "$keyword_pattern" =~ ^[a-z|]+$ ]]
}

neutralize_apply_rejected_marker() {
  local body="$1"
  local keyword_pattern="$2"
  local issue_key="$3"
  local transformed

  if command -v va_exec >/dev/null 2>&1; then
    transformed="$(
      printf '%s' "$body" | va_exec pr closure-marker --stdin \
        --keyword-pattern "$keyword_pattern" \
        --issue "$issue_key" \
        --mode apply 2>/dev/null || true
    )"
    if [[ -n "$transformed" ]]; then
      printf '%s' "$transformed"
      return
    fi
  fi

  NEUTRALIZE_KEYWORDS="$keyword_pattern" \
    NEUTRALIZE_ISSUE_KEY="$issue_key" \
    perl -0777 -pe '
      my $kw = $ENV{NEUTRALIZE_KEYWORDS} // q{};
      my $ik = $ENV{NEUTRALIZE_ISSUE_KEY} // q{};
      my $ikq = quotemeta($ik);
      s/\b((?:$kw))\b(\s+)(?!rejected\b)([^\s]*$ikq)\b/$1$2rejected $3/ig;
    ' <<<"$body"
}

neutralize_remove_rejected_marker() {
  local body="$1"
  local keyword_pattern="$2"
  local issue_key="$3"
  local transformed

  if command -v va_exec >/dev/null 2>&1; then
    transformed="$(
      printf '%s' "$body" | va_exec pr closure-marker --stdin \
        --keyword-pattern "$keyword_pattern" \
        --issue "$issue_key" \
        --mode remove 2>/dev/null || true
    )"
    if [[ -n "$transformed" ]]; then
      printf '%s' "$transformed"
      return
    fi
  fi

  NEUTRALIZE_KEYWORDS="$keyword_pattern" \
    NEUTRALIZE_ISSUE_KEY="$issue_key" \
    perl -0777 -pe '
      my $kw = $ENV{NEUTRALIZE_KEYWORDS} // q{};
      my $ik = $ENV{NEUTRALIZE_ISSUE_KEY} // q{};
      my $ikq = quotemeta($ik);
      s/\b((?:$kw))\b(\s+)rejected\s+([^\s]*$ikq)\b/$1$2$3/ig;
    ' <<<"$body"
}

neutralize_register_non_compliant_ref() {
  local issue_key="$1"
  local action="$2"
  local reason="$3"
  local neutralized_reason_var_name="$4"
  local neutralized_action_var_name="$5"
  local neutralized_count_var_name="$6"
  local -n neutralized_reason_ref="$neutralized_reason_var_name"
  local -n neutralized_action_ref="$neutralized_action_var_name"
  local -n neutralized_count_ref="$neutralized_count_var_name"

  neutralized_reason_ref["$issue_key"]="$reason"
  neutralized_action_ref["$issue_key"]="$action"
  neutralized_count_ref=$((neutralized_count_ref + 1))
}

neutralize_collect_refs_from_body() {
  local body="$1"
  local _out_closing_refs_var="$2"
  local _out_pre_neutralized_refs_var="$3"
  local -n _out_closing_refs_ref="$_out_closing_refs_var"
  local -n _out_pre_neutralized_refs_ref="$_out_pre_neutralized_refs_var"
  local va_records=""
  local record=""
  local record_type=""
  local action=""
  local issue_key=""
  local collected_closing_refs=""
  local collected_pre_neutralized_refs=""

  if command -v va_exec >/dev/null 2>&1; then
    va_records="$(printf '%s' "$body" | va_exec pr directives --stdin 2>/dev/null || true)"
    if [[ -n "$va_records" ]]; then
      while IFS= read -r record; do
        [[ -z "$record" ]] && continue
        IFS='|' read -r record_type action issue_key <<<"$record"
        [[ "$record_type" == "EV" ]] || continue
        [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue

        if [[ "$action" == "Closes" ]]; then
          collected_closing_refs+="${action}|${issue_key}"$'\n'
        elif [[ "$action" == "Closes rejected" ]]; then
          collected_pre_neutralized_refs+="Closes|${issue_key}"$'\n'
        fi
      done <<<"$va_records"

      _out_closing_refs_ref="${collected_closing_refs%$'\n'}"
      _out_pre_neutralized_refs_ref="${collected_pre_neutralized_refs%$'\n'}"
      return
    fi
  fi

  _out_closing_refs_ref="$(parse_closing_issue_refs_from_text "$body")"
  _out_pre_neutralized_refs_ref="$(parse_neutralized_closing_issue_refs_from_text "$body")"
}

neutralize_normalize_ref_line() {
  local action_raw="$1"
  local issue_key_raw="$2"
  local _out_action_var="$3"
  local _out_issue_key_var="$4"
  local _out_issue_number_var="$5"
  local normalized_action normalized_issue_key normalized_issue_number
  local -n _out_action_ref="$_out_action_var"
  local -n _out_issue_key_ref="$_out_issue_key_var"
  local -n _out_issue_number_ref="$_out_issue_number_var"

  normalized_action="$(neutralize_trim "$action_raw")"
  normalized_issue_key="$(neutralize_trim "$issue_key_raw")"
  [[ "$normalized_issue_key" =~ ^#[0-9]+$ ]] || return 1

  normalized_issue_number="${normalized_issue_key//#/}"
  _out_action_ref="$normalized_action"
  _out_issue_key_ref="$normalized_issue_key"
  _out_issue_number_ref="$normalized_issue_number"
  return 0
}

neutralize_mark_seen_ref_or_skip() {
  local action="$1"
  local issue_key="$2"
  local seen_ref_var_name="$3"
  local dedupe_key
  local -n seen_ref_ref="$seen_ref_var_name"

  dedupe_key="${action}|${issue_key}"
  if [[ -n "${seen_ref_ref[$dedupe_key]:-}" ]]; then
    return 1
  fi
  seen_ref_ref["$dedupe_key"]=1
  return 0
}

neutralize_process_closing_refs() {
  local refs="$1"
  local repo_name="$2"
  local updated_body_var_name="$3"
  local seen_ref_var_name="$4"
  local neutralized_reason_var_name="$5"
  local neutralized_action_var_name="$6"
  local neutralized_count_var_name="$7"
  local reason_cache_var_name="$8"

  local -n updated_body_ref="$updated_body_var_name"
  local -n seen_ref_ref="$seen_ref_var_name"

  local action issue_key issue_number reason keyword_pattern

  while IFS='|' read -r action issue_key; do
    neutralize_normalize_ref_line "$action" "$issue_key" action issue_key issue_number || continue
    neutralize_mark_seen_ref_or_skip "$action" "$issue_key" "$seen_ref_var_name" || continue

    reason="$(neutralize_reason_for_issue_cached "$issue_number" "$repo_name" "$reason_cache_var_name")"
    [[ -n "$reason" ]] || continue

    keyword_pattern="$(neutralize_keyword_pattern_from_action "$action")"
    neutralize_valid_keyword_pattern "$keyword_pattern" || continue

    updated_body_ref="$(neutralize_apply_rejected_marker "$updated_body_ref" "$keyword_pattern" "$issue_key")"

    neutralize_register_non_compliant_ref \
      "$issue_key" "$action" "$reason" \
      "$neutralized_reason_var_name" "$neutralized_action_var_name" "$neutralized_count_var_name"
  done <<<"$refs"
}

neutralize_process_pre_neutralized_refs() {
  local refs="$1"
  local repo_name="$2"
  local updated_body_var_name="$3"
  local seen_ref_var_name="$4"
  local neutralized_reason_var_name="$5"
  local neutralized_action_var_name="$6"
  local neutralized_count_var_name="$7"
  local reason_cache_var_name="$8"

  local -n updated_body_ref="$updated_body_var_name"
  local -n seen_ref_ref="$seen_ref_var_name"

  local action issue_key issue_number reason keyword_pattern

  while IFS='|' read -r action issue_key; do
    neutralize_normalize_ref_line "$action" "$issue_key" action issue_key issue_number || continue
    neutralize_mark_seen_ref_or_skip "$action" "$issue_key" "$seen_ref_var_name" || continue

    reason="$(neutralize_reason_for_issue_cached "$issue_number" "$repo_name" "$reason_cache_var_name")"

    keyword_pattern="$(neutralize_keyword_pattern_from_action "$action")"
    neutralize_valid_keyword_pattern "$keyword_pattern" || continue

    if [[ -n "$reason" ]]; then
      # Idempotent re-apply to preserve neutralization if body edits removed/reordered markers.
      updated_body_ref="$(neutralize_apply_rejected_marker "$updated_body_ref" "$keyword_pattern" "$issue_key")"
      neutralize_register_non_compliant_ref \
        "$issue_key" "$action" "$reason" \
        "$neutralized_reason_var_name" "$neutralized_action_var_name" "$neutralized_count_var_name"
    else
      updated_body_ref="$(neutralize_remove_rejected_marker "$updated_body_ref" "$keyword_pattern" "$issue_key")"
    fi
  done <<<"$refs"
}

neutralize_build_comment_body() {
  local marker="$1"
  local neutralized_count="$2"
  local neutralized_reason_var_name="$3"
  local neutralized_action_var_name="$4"
  local -n neutralized_reason_ref="$neutralized_reason_var_name"
  local -n neutralized_action_ref="$neutralized_action_var_name"

  local comment_body
  if [[ "$neutralized_count" -gt 0 ]]; then
    comment_body="$marker
### Closure Neutralization Status

⚠️ Non-compliant issue references were neutralized to prevent incorrect auto-close.

"

    local issue_key
    while IFS= read -r issue_key; do
      [[ -z "$issue_key" ]] && continue
      comment_body+="- ${neutralized_action_ref[$issue_key]} rejected ${issue_key}: ${neutralized_reason_ref[$issue_key]}"$'\n'
    done < <(printf '%s\n' "${!neutralized_reason_ref[@]}" | sort -V)

    comment_body+=$'\n'"How to restore standard auto-close:"$'\n'
    comment_body+="- Fix issue required fields/title contract (if applicable)."$'\n'
    comment_body+="- Remove or adjust \`Reopen #...\` for issues that should close now."$'\n'
    comment_body+="- Remove \`rejected\` from closure lines in PR body."
  else
    comment_body="$marker
### Closure Neutralization Status

✅ No non-compliant closure refs detected. No neutralization applied."
  fi

  echo "$comment_body"
}

neutralize_parse_args() {
  local pr_var_name="$1"
  local repo_var_name="$2"
  shift 2

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --pr)
      issue_cli_assign_value_or_usage "$1" "${2:-}" "$pr_var_name" neutralize_usage || exit 2
      shift 2
      ;;
    --repo)
      issue_cli_assign_value_or_usage "$1" "${2:-}" "$repo_var_name" neutralize_usage || exit 2
      shift 2
      ;;
    -h | --help)
      neutralize_usage
      exit 0
      ;;
    *)
      issue_cli_unknown_option_with_usage "$1" neutralize_usage
      exit 2
      ;;
    esac
  done
}

neutralize_run() {
  local pr_number=""
  local repo_name="${GH_REPO:-}"

  neutralize_parse_args pr_number repo_name "$@"

  [[ -n "$pr_number" ]] || {
    echo "Error: --pr is required." >&2
    neutralize_usage >&2
    exit 2
  }
  issue_cli_require_positive_number "--pr" "$pr_number"
  neutralize_require_deps

  repo_name="$(issue_gh_resolve_repo_name_or_exit "$repo_name" "repository")"

  local marker="<!-- closure-neutralizer:${pr_number} -->"

  local pr_json
  pr_json="$(issue_gh_pr_details_json "$repo_name" "$pr_number")"
  if [[ -z "$pr_json" ]]; then
    echo "Error: unable to read PR #${pr_number}." >&2
    exit 4
  fi

  local original_body updated_body
  local closing_refs="" pre_neutralized_refs=""
  original_body="$(echo "$pr_json" | jq -r '.body // ""')"
  updated_body="$original_body"
  neutralize_collect_refs_from_body "$original_body" closing_refs pre_neutralized_refs

  declare -A seen_ref
  declare -A neutralized_reason
  declare -A neutralized_action
  declare -A reason_cache
  local neutralized_count=0

  neutralize_process_closing_refs \
    "$closing_refs" "$repo_name" \
    updated_body seen_ref neutralized_reason neutralized_action neutralized_count reason_cache

  neutralize_process_pre_neutralized_refs \
    "$pre_neutralized_refs" "$repo_name" \
    updated_body seen_ref neutralized_reason neutralized_action neutralized_count reason_cache

  if [[ "$updated_body" != "$original_body" ]]; then
    neutralize_update_pr_body "$repo_name" "$pr_number" "$updated_body"
  fi

  local comment_body
  comment_body="$(neutralize_build_comment_body "$marker" "$neutralized_count" neutralized_reason neutralized_action)"
  neutralize_upsert_pr_comment "$repo_name" "$pr_number" "$marker" "$comment_body"

  echo "Closure neutralization evaluated for PR #${pr_number}."
}
