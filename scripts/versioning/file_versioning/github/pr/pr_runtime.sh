#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154


# Runtime/support helpers extracted from generate_pr_description.sh.

pr_debug_log() {
  if [[ "$debug_mode" == "true" ]]; then
    echo "[debug] $*" >&2
  fi
}

pr_warn_optional() {
  local message="$1"
  local detail="${2:-}"
  echo "Warning: ${message}" >&2
  if [[ -n "$detail" ]]; then
    echo "Detail: ${detail}" >&2
  fi
}

pr_gh_optional() {
  local description="$1"
  shift

  if [[ "$has_gh" != "true" ]]; then
    pr_debug_log "${description}: gh unavailable, skipping."
    return 1
  fi

  local err_file
  local output
  local attempt
  local max_attempts=3
  local delay_seconds=2

  err_file="$(mktemp)"
  for ((attempt = 1; attempt <= max_attempts; attempt++)); do
    if output="$(gh "$@" 2>"$err_file")"; then
      rm -f "$err_file"
      printf "%s" "$output"
      return 0
    fi
    if [[ "$attempt" -lt "$max_attempts" ]]; then
      sleep "$delay_seconds"
    fi
  done

  pr_warn_optional "${description} failed after ${max_attempts} attempts; continuing without GitHub data." "$(cat "$err_file" 2>/dev/null || true)"
  rm -f "$err_file"
  return 1
}

pr_is_human_interactive_terminal() {
  [[ -t 0 && -t 1 && -z "${CI:-}" ]]
}

pr_cleanup_tmp_files() {
  rm -f "$features_tmp" "$bugs_tmp" "$refactors_tmp" "$sync_tmp" "$issues_tmp" "$reopen_tmp" "$conflict_tmp" "$directive_resolution_tmp"
  if [[ "$keep_artifacts" != "true" ]]; then
    rm -f "$extracted_prs_file" "$resolved_issues_file" "$reopened_issues_file" "$conflict_issues_file"
  fi
}

pr_preferred_base_ref_with_origin() {
  local ref_name="$1"
  if [[ -z "$ref_name" || "$ref_name" == "HEAD" ]]; then
    echo "$ref_name"
    return
  fi

  if git show-ref --verify --quiet "refs/remotes/origin/${ref_name}"; then
    echo "origin/${ref_name}"
    return
  fi

  echo "$ref_name"
}

pr_normalize_branch_display_ref() {
  local raw_ref="$1"
  local normalized

  normalized="${raw_ref#refs/remotes/}"
  normalized="${normalized#refs/heads/}"
  normalized="${normalized#remotes/}"
  normalized="${normalized#origin/}"
  echo "$normalized"
}

pr_get_repo_name_with_owner() {
  if [[ "$has_gh" != "true" ]]; then
    echo ""
    return
  fi

  if [[ -n "$repo_name_with_owner_cache" ]]; then
    echo "$repo_name_with_owner_cache"
    return
  fi

  if [[ -n "${GH_REPO:-}" ]]; then
    repo_name_with_owner_cache="$GH_REPO"
    echo "$repo_name_with_owner_cache"
    return
  fi

  repo_name_with_owner_cache="$(pr_gh_optional "resolve repository name" repo view --json nameWithOwner -q '.nameWithOwner')"
  echo "$repo_name_with_owner_cache"
}

pr_seed_pr_ref_cache() {
  local pr_ref

  if [[ -n "$main_pr_number" ]]; then
    pr_ref_cache["#${main_pr_number}"]="1"
  fi

  if [[ -s "$extracted_prs_file" ]]; then
    while read -r pr_ref; do
      [[ -z "$pr_ref" ]] && continue
      pr_ref_cache["$pr_ref"]="1"
    done <"$extracted_prs_file"
  fi
}
