#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Breaking-change detection helpers.

pr_text_indicates_breaking_legacy() {
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

pr_text_indicates_breaking() {
  local text="${1:-}"
  local breaking_result=""

  if command -v va_exec >/dev/null 2>&1; then
    breaking_result="$(printf '%s' "$text" | va_exec pr breaking-detect --stdin 2>/dev/null || true)"
    if [[ "$breaking_result" == "true" ]]; then
      return 0
    fi
    if [[ "$breaking_result" == "false" ]]; then
      return 1
    fi
  fi

  pr_text_indicates_breaking_legacy "$text"
}

pr_extract_breaking_scope_from_subject() {
  local subject="${1:-}"
  local lower
  lower="$(echo "$subject" | tr '[:upper:]' '[:lower:]')"
  if [[ "$lower" =~ ^[[:space:]]*[a-z][a-z0-9_-]*\(([a-z0-9_./,-]+)\)!:[[:space:]]+ ]]; then
    echo "${BASH_REMATCH[1]}"
    return
  fi
  if [[ "$lower" =~ ^[[:space:]]*[a-z][a-z0-9_-]*\(([a-z0-9_./,-]+)\):[[:space:]]+ ]]; then
    echo "${BASH_REMATCH[1]}"
    return
  fi
}

pr_collect_breaking_scope_from_compare_metadata() {
  local range="$1"
  local compare_base compare_head
  local messages payload rec first_line lower scope short_hash
  local -n out_commits_ref="$2"
  local -n out_scopes_ref="$3"
  local -n out_has_breaking_ref="$4"
  declare -A seen_commits=()
  declare -A seen_scopes=()

  compare_base="${range%%..*}"
  compare_head="${range#*..}"
  if [[ -z "$compare_base" || -z "$compare_head" || "$compare_base" == "$range" || "$compare_head" == "$range" ]]; then
    return
  fi

  messages="$(pr_load_compare_commit_messages "$compare_base" "$compare_head" || true)"
  [[ -n "$messages" ]] || return
  payload="$(printf "%s" "$messages" | awk 'BEGIN{RS=""; ORS="\x1e"} {print}')"

  while IFS= read -r -d $'\x1e' rec; do
    [[ -z "$rec" ]] && continue
    if ! pr_text_indicates_breaking "$rec"; then
      continue
    fi
    out_has_breaking_ref=1

    first_line="$(printf "%s" "$rec" | head -n1)"
    lower="$(echo "$first_line" | tr '[:upper:]' '[:lower:]')"

    if [[ "$lower" =~ ^[[:space:]]*([0-9a-f]{7,40})[[:space:]]+ ]]; then
      short_hash="$(printf "%s" "${BASH_REMATCH[1]}" | cut -c1-7)"
      if [[ -z "${seen_commits[$short_hash]:-}" ]]; then
        seen_commits["$short_hash"]=1
        out_commits_ref+=("$short_hash")
      fi
      first_line="${first_line#${BASH_REMATCH[1]}}"
      first_line="${first_line# }"
    fi

    scope="$(pr_extract_breaking_scope_from_subject "$first_line")"
    if [[ -n "$scope" && -z "${seen_scopes[$scope]:-}" ]]; then
      seen_scopes["$scope"]=1
      out_scopes_ref+=("$scope")
    fi
  done < <(printf "%s" "$payload")
}

pr_compute_breaking_scope() {
  local range="${1:-${base_ref_git:-}..${head_ref_git:-}}"
  local raw_log rec full_hash short_hash subject body
  local files crate
  local metadata_has_breaking=0
  local metadata_commits=()
  local metadata_scopes=()
  declare -A seen_breaking_hashes
  declare -A seen_crates
  local commit_list=()
  local crate_list=()

  raw_log="$(git log --format='%H%x1f%s%x1f%b%x1e' "$range" 2>/dev/null || true)"

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

  if [[ ${#commit_list[@]} -eq 0 || ${#crate_list[@]} -eq 0 ]]; then
    pr_collect_breaking_scope_from_compare_metadata "$range" metadata_commits metadata_scopes metadata_has_breaking
    if [[ ${#commit_list[@]} -eq 0 && ${#metadata_commits[@]} -gt 0 ]]; then
      commit_list=("${metadata_commits[@]}")
    fi
    if [[ ${#crate_list[@]} -eq 0 && ${#metadata_scopes[@]} -gt 0 ]]; then
      crate_list=("${metadata_scopes[@]}")
    fi
  fi

  if [[ ${#commit_list[@]} -gt 0 ]]; then
    mapfile -t commit_list < <(printf "%s\n" "${commit_list[@]}" | sort -u)
    breaking_scope_commits="$(printf "\`%s\`, " "${commit_list[@]}")"
    breaking_scope_commits="${breaking_scope_commits%, }"
  elif [[ "${breaking_detected:-0}" -eq 1 && "$metadata_has_breaking" -eq 1 ]]; then
    breaking_scope_commits="metadata-only (diff empty or unavailable)"
  else
    breaking_scope_commits=""
  fi

  if [[ ${#crate_list[@]} -gt 0 ]]; then
    mapfile -t crate_list < <(printf "%s\n" "${crate_list[@]}" | sort -u)
    breaking_scope_crates="$(printf "\`%s\`, " "${crate_list[@]}")"
    breaking_scope_crates="${breaking_scope_crates%, }"
  elif [[ "${breaking_detected:-0}" -eq 1 && "$metadata_has_breaking" -eq 1 ]]; then
    breaking_scope_crates="metadata-only (scope not inferable)"
  else
    breaking_scope_crates=""
  fi
}
