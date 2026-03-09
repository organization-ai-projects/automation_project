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
  local record_type field_a field_b action issue_key decision

  while IFS='|' read -r record_type field_a field_b; do
    case "$record_type" in
    EV)
      action="$(pr_directive_conflict_guard_trim "$field_a")"
      issue_key="$(pr_directive_conflict_guard_trim "$field_b")"
      [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue
      if [[ "$action" == "Closes" ]]; then
        _out_closing_requested_ref["$issue_key"]=1
      elif [[ "$action" == "Reopen" ]]; then
        _out_reopen_requested_ref["$issue_key"]=1
      fi
      ;;
    DEC)
      issue_key="$(pr_directive_conflict_guard_trim "$field_a")"
      decision="$(pr_directive_conflict_guard_trim "$field_b" | tr '[:upper:]' '[:lower:]')"
      [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue
      [[ "$decision" == "close" || "$decision" == "reopen" ]] || continue
      _out_directive_decision_ref["$issue_key"]="$decision"
      ;;
    esac
  done < <(parse_issue_directive_records_from_text "$text")
}

pr_directive_conflict_guard_collect_inferred_decisions() {
  local text="$1"
  local _out_inferred_decision_var="$2"
  local -n _out_inferred_decision_ref="$_out_inferred_decision_var"
  local record_type field_a field_b action issue_key

  while IFS='|' read -r record_type field_a field_b; do
    [[ "$record_type" == "EV" ]] || continue
    action="$(pr_directive_conflict_guard_trim "$field_a")"
    issue_key="$(pr_directive_conflict_guard_trim "$field_b")"
    [[ "$issue_key" =~ ^#[0-9]+$ ]] || continue
    case "$action" in
    Closes) _out_inferred_decision_ref["$issue_key"]="close" ;;
    Reopen) _out_inferred_decision_ref["$issue_key"]="reopen" ;;
    esac
  done < <(parse_issue_directive_records_from_text "$text")
}

pr_directive_conflict_guard_run() {
  local pr_number=""
  local repo_name="${GH_REPO:-}"
  local pr_json original_body updated_body commit_messages directive_payload
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

  pr_json="$(pr_directive_conflict_guard_fetch_pr_json "$repo_name" "$pr_number")"
  if [[ -z "$pr_json" ]]; then
    echo "Error: unable to read PR #${pr_number}." >&2
    exit 4
  fi

  original_body="$(echo "$pr_json" | jq -r '.body // ""')"
  updated_body="$original_body"

  pr_directive_conflict_guard_collect_explicit_directives \
    "$original_body" \
    closing_requested \
    reopen_requested \
    directive_decision

  commit_messages="$(pr_directive_conflict_guard_fetch_commit_messages "$repo_name" "$pr_number")"
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
  pr_directive_conflict_guard_collect_inferred_decisions \
    "$directive_payload" \
    inferred_decision

  for issue_key in "${!closing_requested[@]}"; do
    if [[ -z "${reopen_requested[$issue_key]:-}" ]]; then
      continue
    fi
    if [[ -n "${directive_decision[$issue_key]:-}" ]]; then
      resolved_conflict["$issue_key"]="${directive_decision[$issue_key]} (explicit)"
      resolved_count=$((resolved_count + 1))
    elif [[ "$allow_inferred_resolution" == "true" && -n "${inferred_decision[$issue_key]:-}" ]]; then
      resolved_conflict["$issue_key"]="${inferred_decision[$issue_key]} (inferred from latest directive)"
      resolved_count=$((resolved_count + 1))
    else
      if [[ "$allow_inferred_resolution" != "true" ]]; then
        unresolved_conflict["$issue_key"]="Closes + Reopen detected across multiple source branches; explicit decision required."
      else
        unresolved_conflict["$issue_key"]="Closes + Reopen detected without explicit decision."
      fi
      unresolved_count=$((unresolved_count + 1))
    fi
  done

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
