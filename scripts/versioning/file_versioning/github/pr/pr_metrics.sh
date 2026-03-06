#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154


# CI/breaking metrics helpers extracted from generate_pr_description.sh.

pr_text_indicates_breaking() {
  local text="${1:-}"
  local line
  local lower
  local cc_breaking_re='^[[:space:]]*[a-z][a-z0-9_-]*(\([a-z0-9_./,-]+\))?!:[[:space:]]+'

  while IFS= read -r line; do
    lower="$(echo "$line" | tr '[:upper:]' '[:lower:]')"

    if [[ "$lower" =~ non[[:space:]-]?breaking[[:space:]_-]*change ]]; then
      continue
    fi
    if [[ "$lower" =~ ^[[:space:]]*(no|without)[[:space:]]+breaking[[:space:]_-]*changes? ]]; then
      continue
    fi

    if [[ "$lower" =~ ^[[:space:]]*-[[:space:]]*\[[xX]\][[:space:]]*breaking[[:space:]_-]*change([[:space:]]|$) ]]; then
      return 0
    fi

    if [[ "$lower" =~ ^[[:space:]]*breaking[[:space:]_-]*change[[:space:]]*: ]]; then
      return 0
    fi

    if [[ "$lower" =~ $cc_breaking_re ]]; then
      return 0
    fi
  done <<<"$text"

  return 1
}

pr_compute_breaking_scope() {
  local range="${1:-${base_ref_git:-}..${head_ref_git:-}}"
  local raw_log rec full_hash short_hash subject body
  local files crate
  declare -A seen_breaking_hashes
  declare -A seen_crates
  local commit_list=()
  local crate_list=()

  raw_log="$(git log --format='%H%x1f%s%x1f%b%x1e' "$range" 2>/dev/null || true)"
  if [[ -z "$raw_log" ]]; then
    breaking_scope_crates=""
    breaking_scope_commits=""
    return
  fi

  while IFS= read -r -d $'\x1e' rec; do
    [[ -z "$rec" ]] && continue
    full_hash="$(printf "%s" "$rec" | awk -F $'\x1f' '{print $1}')"
    subject="$(printf "%s" "$rec" | awk -F $'\x1f' '{print $2}')"
    body="$(printf "%s" "$rec" | awk -F $'\x1f' '{print $3}')"
    if ! pr_text_indicates_breaking "${subject}"$'\n'"${body}"; then
      continue
    fi
    short_hash="$(printf "%s" "$full_hash" | cut -c1-7)"
    if [[ -z "${seen_breaking_hashes[$short_hash]:-}" ]]; then
      seen_breaking_hashes["$short_hash"]=1
      commit_list+=("$short_hash")
    fi

    files="$(git show --name-only --pretty=format: "$full_hash" 2>/dev/null || true)"
    while IFS= read -r rel_path; do
      [[ -z "$rel_path" ]] && continue
      crate="$(pr_infer_crate_from_path "$rel_path")"
      [[ -z "$crate" ]] && continue
      if [[ -z "${seen_crates[$crate]:-}" ]]; then
        seen_crates["$crate"]=1
        crate_list+=("$crate")
      fi
    done <<<"$files"
  done < <(printf "%s" "$raw_log")

  if [[ ${#commit_list[@]} -gt 0 ]]; then
    mapfile -t commit_list < <(printf "%s\n" "${commit_list[@]}" | sort -u)
    breaking_scope_commits="$(printf "\`%s\`, " "${commit_list[@]}")"
    breaking_scope_commits="${breaking_scope_commits%, }"
  else
    breaking_scope_commits=""
  fi

  if [[ ${#crate_list[@]} -gt 0 ]]; then
    mapfile -t crate_list < <(printf "%s\n" "${crate_list[@]}" | sort -u)
    breaking_scope_crates="$(printf "\`%s\`, " "${crate_list[@]}")"
    breaking_scope_crates="${breaking_scope_crates%, }"
  else
    breaking_scope_crates=""
  fi
}

pr_compute_ci_status() {
  local target_pr_number=""
  local rollup_json conclusions unresolved

  if [[ -n "$auto_edit_pr_number" ]]; then
    target_pr_number="$auto_edit_pr_number"
  elif [[ "$dry_run" == "false" && -n "$main_pr_number" ]]; then
    target_pr_number="$main_pr_number"
  fi

  ci_status="UNKNOWN"
  [[ -z "$target_pr_number" ]] && return

  rollup_json="$(pr_gh_optional "read checks for PR #${target_pr_number}" pr view "$target_pr_number" --json statusCheckRollup)"
  if [[ -z "$rollup_json" ]]; then
    return
  fi

  conclusions="$(echo "$rollup_json" | jq -r '.statusCheckRollup // [] | map((.conclusion // .state // .status // "UNKNOWN") | tostring | ascii_upcase) | map(select(length > 0)) | .[]' 2>/dev/null || true)"
  if [[ -z "$conclusions" ]]; then
    return
  fi

  if echo "$conclusions" | grep -Eq '^(FAILURE|FAILED|CANCELLED|TIMED_OUT|ACTION_REQUIRED|STARTUP_FAILURE)$'; then
    ci_status="FAIL"
    return
  fi

  if echo "$conclusions" | grep -Eq '^(IN_PROGRESS|QUEUED|PENDING|WAITING|REQUESTED)$'; then
    ci_status="RUNNING"
    return
  fi

  unresolved="$(echo "$conclusions" | grep -Ev '^(SUCCESS|PASSED|NEUTRAL|SKIPPED|COMPLETED)$' || true)"
  if [[ -n "$unresolved" ]]; then
    ci_status="UNKNOWN"
    return
  fi

  if echo "$conclusions" | grep -Eq '^(SUCCESS|PASSED)$'; then
    ci_status="PASS"
  fi
}
