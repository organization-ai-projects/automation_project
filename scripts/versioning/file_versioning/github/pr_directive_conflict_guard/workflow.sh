#!/usr/bin/env bash
# shellcheck shell=bash

pr_directive_conflict_guard_collect_explicit_directives() {
  local text="$1"
  local _out_closing_requested_var="$2"
  local _out_reopen_requested_var="$3"
  local _out_directive_decision_var="$4"
  local -n _out_closing_requested_ref="$_out_closing_requested_var"
  local -n _out_reopen_requested_ref="$_out_reopen_requested_var"
  local -n _out_directive_decision_ref="$_out_directive_decision_var"
  local action issue_key decision

  while IFS='|' read -r action issue_key; do
    [[ "$action" == "Closes" ]] || continue
    [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue
    _out_closing_requested_ref["$issue_key"]=1
  done < <(parse_closing_issue_refs_from_text "$text")

  while IFS='|' read -r action issue_key; do
    [[ "$action" == "Reopen" ]] || continue
    [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue
    _out_reopen_requested_ref["$issue_key"]=1
  done < <(parse_reopen_issue_refs_from_text "$text")

  while IFS='|' read -r issue_key decision; do
    issue_key="$(pr_directive_conflict_guard_trim "$issue_key")"
    decision="$(pr_directive_conflict_guard_trim "$decision" | tr '[:upper:]' '[:lower:]')"
    [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue
    [[ "$decision" == "close" || "$decision" == "reopen" ]] || continue
    _out_directive_decision_ref["$issue_key"]="$decision"
  done < <(parse_directive_decisions_from_text "$text")
}

pr_directive_conflict_guard_collect_inferred_decisions() {
  local text="$1"
  local _out_inferred_decision_var="$2"
  local -n _out_inferred_decision_ref="$_out_inferred_decision_var"
  local issue_key decision

  while IFS='|' read -r issue_key decision; do
    issue_key="$(pr_directive_conflict_guard_trim "$issue_key")"
    decision="$(pr_directive_conflict_guard_trim "$decision" | tr '[:upper:]' '[:lower:]')"
    [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue
    [[ "$decision" == "close" || "$decision" == "reopen" ]] || continue
    _out_inferred_decision_ref["$issue_key"]="$decision"
  done < <(parse_inferred_directive_decisions_from_text "$text")
}

pr_directive_conflict_guard_collect_conflicts_via_va() {
  local text="$1"
  local source_branch_count="$2"
  local _out_resolved_conflict_var="$3"
  local _out_unresolved_conflict_var="$4"
  local -n _out_resolved_conflict_ref="$_out_resolved_conflict_var"
  local -n _out_unresolved_conflict_ref="$_out_unresolved_conflict_var"
  local output
  local record_type issue_key decision_or_reason origin

  if ! command -v va_exec >/dev/null 2>&1; then
    return 1
  fi

  output="$(printf '%s' "$text" | va_exec pr directive-conflicts --stdin --source-branch-count "$source_branch_count" 2>/dev/null)" || {
    return 1
  }

  while IFS='|' read -r record_type issue_key decision_or_reason origin; do
    [[ -z "$record_type" ]] && continue
    [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue
    case "$record_type" in
    RES)
      [[ "$decision_or_reason" == "close" || "$decision_or_reason" == "reopen" ]] || continue
      _out_resolved_conflict_ref["$issue_key"]="$decision_or_reason ($origin)"
      ;;
    UNRES)
      [[ -n "$decision_or_reason" ]] || continue
      _out_unresolved_conflict_ref["$issue_key"]="$decision_or_reason"
      ;;
    esac
  done <<<"$output"
}

pr_directive_conflict_guard_run() {
  local pr_number=""
  local repo_name="${GH_REPO:-}"
  local pr_details_json original_body updated_body commit_messages directive_payload
  local source_branch_count allow_inferred_resolution
  local marker block_start block_end
  local issue_key action decision comment_body conflict_block
  local unresolved_count=0 resolved_count=0
  local -A closing_requested=()
  local -A reopen_requested=()
  local -A directive_decision=()
  local -A inferred_decision=()
  local -A unresolved_conflict=()
  local -A resolved_conflict=()

  pr_directive_conflict_guard_parse_cli pr_number repo_name "$@"
  gh_cli_require_gh_jq_perl
  repo_name="$(pr_directive_conflict_guard_resolve_repo_name "$repo_name")"

  marker="<!-- directive-conflict-guard:${pr_number} -->"
  block_start="<!-- directive-conflicts:start -->"
  block_end="<!-- directive-conflicts:end -->"

  pr_details_json="$(pr_directive_conflict_guard_fetch_pr_details_json "$repo_name" "$pr_number")"
  if [[ -z "$pr_details_json" ]]; then
    echo "Error: unable to read PR #${pr_number}." >&2
    exit 4
  fi

  original_body="$(echo "$pr_details_json" | jq -r '.body // ""')"
  updated_body="$original_body"

  commit_messages="$(echo "$pr_details_json" | jq -r '.commit_messages // ""')"
  source_branch_count="$(
    printf '%s\n' "$commit_messages" |
      sed -nE 's@.*Merge pull request #[0-9]+ from [^/]+/(.+)@\1@p' |
      sort -u | sed '/^$/d' | wc -l | tr -d ' '
  )"
  allow_inferred_resolution="true"
  if [[ "${source_branch_count:-0}" -gt 1 ]]; then
    allow_inferred_resolution="false"
  fi

  directive_payload="${commit_messages}"$'\n'"${original_body}"
  if ! pr_directive_conflict_guard_collect_conflicts_via_va \
    "$directive_payload" \
    "${source_branch_count:-1}" \
    resolved_conflict \
    unresolved_conflict; then
    pr_directive_conflict_guard_collect_explicit_directives \
      "$original_body" \
      closing_requested \
      reopen_requested \
      directive_decision

    pr_directive_conflict_guard_collect_inferred_decisions \
      "$directive_payload" \
      inferred_decision

    for issue_key in "${!closing_requested[@]}"; do
      if [[ -z "${reopen_requested[$issue_key]:-}" ]]; then
        continue
      fi
      if [[ -n "${directive_decision[$issue_key]:-}" ]]; then
        resolved_conflict["$issue_key"]="${directive_decision[$issue_key]} (explicit)"
      elif [[ "$allow_inferred_resolution" == "true" && -n "${inferred_decision[$issue_key]:-}" ]]; then
        resolved_conflict["$issue_key"]="${inferred_decision[$issue_key]} (inferred from latest directive)"
      else
        if [[ "$allow_inferred_resolution" != "true" ]]; then
          unresolved_conflict["$issue_key"]="Closes + Reopen detected across multiple source branches; explicit decision required."
        else
          unresolved_conflict["$issue_key"]="Closes + Reopen detected without explicit decision."
        fi
      fi
    done
  fi

  resolved_count="${#resolved_conflict[@]}"
  unresolved_count="${#unresolved_conflict[@]}"

  for issue_key in "${!resolved_conflict[@]}"; do
    if [[ "${resolved_conflict[$issue_key]}" != close* ]]; then
      continue
    fi
    updated_body="$(pr_directive_conflict_guard_apply_reopen_rejected_marker "$updated_body" "$issue_key")"
  done

  if [[ "$resolved_count" -gt 0 || "$unresolved_count" -gt 0 ]]; then
    conflict_block="${block_start}
### Issue Directive Decisions
"
    if [[ "$resolved_count" -gt 0 ]]; then
      conflict_block+=$'\n'"Resolved decisions:"$'\n'
      while IFS= read -r issue_key; do
        [[ -z "$issue_key" ]] && continue
        conflict_block+="- ${issue_key} => ${resolved_conflict[$issue_key]}"$'\n'
      done < <(printf '%s\n' "${!resolved_conflict[@]}" | sort -V)
    fi
    if [[ "$unresolved_count" -gt 0 ]]; then
      conflict_block+=$'\n'"❌ Unresolved conflicts (merge blocked):"$'\n'
      while IFS= read -r issue_key; do
        [[ -z "$issue_key" ]] && continue
        conflict_block+="- ${issue_key}: ${unresolved_conflict[$issue_key]}"$'\n'
      done < <(printf '%s\n' "${!unresolved_conflict[@]}" | sort -V)
      conflict_block+=$'\n'"Required decision format:"$'\n'
      conflict_block+="- \`Directive Decision: #<issue> => close\`"$'\n'
      conflict_block+="- \`Directive Decision: #<issue> => reopen\`"$'\n'
    fi
    conflict_block+="${block_end}"
    updated_body="$(
      pr_directive_conflict_guard_upsert_conflict_block_in_body \
        "$updated_body" \
        "$conflict_block" \
        "$block_start" \
        "$block_end"
    )"
  else
    updated_body="$(
      pr_directive_conflict_guard_upsert_conflict_block_in_body \
        "$updated_body" \
        "" \
        "$block_start" \
        "$block_end"
    )"
  fi

  if [[ "$updated_body" != "$original_body" ]]; then
    gh pr edit "$pr_number" -R "$repo_name" --body "$updated_body" >/dev/null
  fi

  if [[ "$unresolved_count" -gt 0 ]]; then
    comment_body="$marker
### Directive Conflict Guard

❌ Unresolved Closes/Reopen conflicts detected. Add explicit directive decisions in PR body."
    pr_directive_conflict_guard_upsert_pr_comment "$repo_name" "$pr_number" "$marker" "$comment_body"
    echo "Unresolved directive conflicts detected for PR #${pr_number}." >&2
    exit 8
  fi

  if [[ "$resolved_count" -gt 0 ]]; then
    comment_body="$marker
### Directive Conflict Guard

✅ Directive conflicts resolved via explicit decisions."
    pr_directive_conflict_guard_upsert_pr_comment "$repo_name" "$pr_number" "$marker" "$comment_body"
  fi

  echo "Directive conflict guard evaluated for PR #${pr_number}."
}
