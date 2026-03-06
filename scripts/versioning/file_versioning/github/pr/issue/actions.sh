#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Issue duplicate handling helpers.

pr_add_duplicate_entry() {
  local duplicate_issue_raw="$1"
  local canonical_issue_raw="$2"
  local duplicate_issue_key canonical_issue_key

  duplicate_issue_key="$(normalize_issue_key "$duplicate_issue_raw" || true)"
  canonical_issue_key="$(normalize_issue_key "$canonical_issue_raw" || true)"
  [[ -z "$duplicate_issue_key" || -z "$canonical_issue_key" ]] && return

  duplicate_targets["$duplicate_issue_key"]="$canonical_issue_key"
}

pr_process_duplicate_mode() {
  local duplicate_issue_key canonical_issue_key duplicate_issue_number comment_body
  local repo_name_with_owner
  local auto_close_allowed="true"

  [[ -z "$duplicate_mode" ]] && return 0

  if [[ -z "${duplicate_targets+x}" || "${#duplicate_targets[@]}" -eq 0 ]]; then
    echo "Duplicate mode (${duplicate_mode}): no duplicate declarations detected."
    return 0
  fi

  if [[ "$dry_run" == "true" ]]; then
    echo "Duplicate mode (${duplicate_mode}): dry-run simulation; no GitHub mutation applied."
    return 0
  fi

  if [[ "$duplicate_mode" == "auto-close" && "$assume_yes" != "true" ]]; then
    auto_close_allowed="false"
    echo "Warning: duplicate auto-close requested without --yes; close action will be skipped." >&2
  fi

  repo_name_with_owner="$(pr_get_repo_name_with_owner)"
  if [[ -z "$repo_name_with_owner" ]]; then
    echo "Warning: unable to resolve repository; duplicate mode skipped." >&2
    return 0
  fi

  for duplicate_issue_key in "${!duplicate_targets[@]}"; do
    canonical_issue_key="${duplicate_targets[$duplicate_issue_key]}"
    duplicate_issue_number="${duplicate_issue_key//#/}"

    if [[ "$duplicate_mode" == "safe" ]]; then
      comment_body="Potential duplicate detected by PR generation workflow: ${duplicate_issue_key} may duplicate ${canonical_issue_key}. Please review manually."
    else
      comment_body="Duplicate of ${canonical_issue_key}"
    fi

    gh api "repos/${repo_name_with_owner}/issues/${duplicate_issue_number}/comments" \
      --raw-field body="${comment_body}" >/dev/null
    echo "Duplicate mode (${duplicate_mode}): commented on ${duplicate_issue_key} (target ${canonical_issue_key})."

    if [[ "$duplicate_mode" == "auto-close" && "$auto_close_allowed" == "true" ]]; then
      gh api -X PATCH "repos/${repo_name_with_owner}/issues/${duplicate_issue_number}" \
        -f state="closed" -f state_reason="not_planned" >/dev/null
      echo "Duplicate mode (${duplicate_mode}): closed ${duplicate_issue_key}."
    elif [[ "$duplicate_mode" == "auto-close" ]]; then
      echo "Duplicate mode (${duplicate_mode}): close skipped for ${duplicate_issue_key} (missing --yes)."
    fi
  done
}
