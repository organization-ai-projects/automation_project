#!/usr/bin/env bash
# shellcheck shell=bash

# Core issue-reference parsing helpers.

_issue_refs_cache_key_for_text() {
  local text="$1"
  # cksum returns checksum + byte-length; stable enough for in-process cache keys.
  printf '%s' "$text" | cksum | awk '{ print $1 ":" $2 }'
}

_issue_refs_build_va_cmd() {
  local subcommand="$1"
  local out_var_name="$2"
  local -n out_ref="$out_var_name"

  out_ref=()
  if [[ -n "${VA_PR_DIRECTIVES_BIN:-}" ]]; then
    out_ref=("${VA_PR_DIRECTIVES_BIN}" pr "$subcommand")
    return 0
  fi
  out_ref=(va_exec pr "$subcommand")
  return 0
}

_parse_issue_directive_event_refs() {
  local text="$1"
  local event_action="$2"
  local emitted_action="${3:-$event_action}"
  parse_issue_directive_records_from_text "$text" | awk -F'|' -v event_action="$event_action" -v emitted_action="$emitted_action" '$1 == "EV" && $2 == event_action { print emitted_action "|" $3 }'
}

_parse_closure_refs_via_va() {
  local text="$1"
  local mode="$2"
  local -a cmd=()

  if ! _issue_refs_build_va_cmd "closure-refs" cmd; then
    return 1
  fi

  case "$mode" in
  close)
    printf '%s' "$text" | "${cmd[@]}" --stdin | awk -F'|' '$1 == "CLOSE" { print $2 "|" $3 }'
    ;;
  pre)
    printf '%s' "$text" | "${cmd[@]}" --stdin | awk -F'|' '$1 == "PRE" { print $2 "|" $3 }'
    ;;
  all)
    printf '%s' "$text" | "${cmd[@]}" --stdin | awk -F'|' '($1 == "CLOSE" || $1 == "PRE") { print $2 "|" $3 }'
    ;;
  *)
    return 1
    ;;
  esac
}

_parse_directives_state_via_va() {
  local text="$1"
  local mode="$2"
  local -a cmd=()

  if ! _issue_refs_build_va_cmd "directives-state" cmd; then
    return 1
  fi

  case "$mode" in
  reopen)
    printf '%s' "$text" | "${cmd[@]}" --stdin | awk -F'|' '$1 == "EV" && $2 == "Reopen" { print $2 "|" $3 }'
    ;;
  duplicate)
    printf '%s' "$text" | "${cmd[@]}" --stdin | awk -F'|' '$1 == "DUP" { print $2 "|" $3 }'
    ;;
  decision)
    printf '%s' "$text" | "${cmd[@]}" --stdin | awk -F'|' '$1 == "DEC" { print $2 "|" $3 }'
    ;;
  inferred)
    printf '%s' "$text" | "${cmd[@]}" --stdin | awk -F'|' '$1 == "INF" { print $2 "|" $3 }'
    ;;
  *)
    return 1
    ;;
  esac
}

_parse_non_closing_refs_via_va() {
  local text="$1"
  local -a cmd=()

  if ! _issue_refs_build_va_cmd "non-closing-refs" cmd; then
    return 1
  fi

  printf '%s' "$text" | "${cmd[@]}" --stdin
}

_parse_issue_directive_records_by_type() {
  local text="$1"
  local record_type="$2"
  parse_issue_directive_records_from_text "$text" | awk -F'|' -v record_type="$record_type" '$1 == record_type { print $2 "|" $3 }'
}

parse_closing_issue_refs_from_text() {
  local text="$1"
  local parsed
  if parsed="$(_parse_closure_refs_via_va "$text" "close" 2>/dev/null)"; then
    printf '%s\n' "$parsed" | sed '/^$/d' | sort -u
    return 0
  fi
  _parse_issue_directive_event_refs "$text" "Closes" | sort -u
}

parse_pr_body_closing_issue_refs_from_text() {
  local text="$1"
  # Semantic alias: PR body parsing rules currently match generic closing-reference rules.
  parse_closing_issue_refs_from_text "$text"
}

parse_non_closing_issue_refs_from_text() {
  local text="$1"
  local parsed
  if parsed="$(_parse_non_closing_refs_via_va "$text" 2>/dev/null)"; then
    printf '%s\n' "$parsed" | sed '/^$/d' | sort -u
    return 0
  fi
  _parse_issue_directive_event_refs "$text" "Part of" | sort -u
}

parse_neutralized_closing_issue_refs_from_text() {
  local text="$1"
  local parsed
  if parsed="$(_parse_closure_refs_via_va "$text" "pre" 2>/dev/null)"; then
    printf '%s\n' "$parsed" | sed '/^$/d' | sort -u
    return 0
  fi
  _parse_issue_directive_event_refs "$text" "Closes rejected" "Closes" | sort -u
}

parse_all_closing_issue_refs_from_text() {
  local text="$1"
  local parsed
  if parsed="$(_parse_closure_refs_via_va "$text" "all" 2>/dev/null)"; then
    printf '%s\n' "$parsed" | sed '/^$/d' | sort -u
    return 0
  fi
  parse_issue_directive_records_from_text "$text" | awk -F'|' '
    $1 == "EV" && ($2 == "Closes" || $2 == "Closes rejected") { print "Closes|" $3 }
  ' | sort -u
}

parse_issue_directive_records_from_text() {
  local text="$1"
  local native_output
  local cache_key

  if [[ "${issue_refs_records_cache_initialized:-0}" != "1" ]]; then
    declare -gA issue_refs_records_cache
    issue_refs_records_cache_initialized="1"
  fi

  cache_key="$(_issue_refs_cache_key_for_text "$text")"
  if [[ -v "issue_refs_records_cache[$cache_key]" ]]; then
    [[ -n "${issue_refs_records_cache[$cache_key]}" ]] && printf '%s\n' "${issue_refs_records_cache[$cache_key]}"
    return 0
  fi

  if native_output="$(_parse_issue_directive_records_via_va "$text" 2>/dev/null)"; then
    issue_refs_records_cache["$cache_key"]="$native_output"
    [[ -n "$native_output" ]] && printf '%s\n' "$native_output"
    return 0
  fi

  native_output="$(parse_issue_directive_records_legacy_from_text "$text")"
  issue_refs_records_cache["$cache_key"]="$native_output"
  [[ -n "$native_output" ]] && printf '%s\n' "$native_output"
}

_parse_issue_directive_records_via_va() {
  local text="$1"
  local -a cmd=()

  if ! _issue_refs_build_va_cmd "directives" cmd; then
    return 1
  fi

  if printf '%s' "$text" | "${cmd[@]}" --stdin --format plain; then
    return 0
  fi
  return 1
}

parse_directive_events_from_text() {
  local text="$1"
  _parse_issue_directive_records_by_type "$text" "EV"
}

parse_reopen_issue_refs_from_text() {
  local text="$1"
  local parsed
  if parsed="$(_parse_directives_state_via_va "$text" "reopen" 2>/dev/null)"; then
    printf '%s\n' "$parsed" | sed '/^$/d' | sort -u
    return 0
  fi
  _parse_issue_directive_event_refs "$text" "Reopen" | sort -u
}

parse_duplicate_refs_from_text() {
  local text="$1"
  local parsed
  if parsed="$(_parse_directives_state_via_va "$text" "duplicate" 2>/dev/null)"; then
    printf '%s\n' "$parsed" | sed '/^$/d' | sort -u
    return 0
  fi
  _parse_issue_directive_records_by_type "$text" "DUP" | sort -u
}

parse_directive_decisions_from_text() {
  local text="$1"
  local parsed
  if parsed="$(_parse_directives_state_via_va "$text" "decision" 2>/dev/null)"; then
    printf '%s\n' "$parsed" | sed '/^$/d' | sort -u
    return 0
  fi
  _parse_issue_directive_records_by_type "$text" "DEC" | sort -u
}

parse_inferred_directive_decisions_from_text() {
  local text="$1"
  local parsed
  local action issue_key
  local -A inferred=()

  if parsed="$(_parse_directives_state_via_va "$text" "inferred" 2>/dev/null)"; then
    printf '%s\n' "$parsed" | sed '/^$/d' | sort -u
    return 0
  fi

  while IFS='|' read -r action issue_key; do
    case "$action" in
    Closes) inferred["$issue_key"]="close" ;;
    Reopen) inferred["$issue_key"]="reopen" ;;
    esac
  done < <(parse_directive_events_from_text "$text")

  for issue_key in "${!inferred[@]}"; do
    printf '%s|%s\n' "$issue_key" "${inferred[$issue_key]}"
  done | sort -u
}

parse_issue_numbers_from_refs() {
  local refs="$1"
  printf '%s\n' "$refs" |
    cut -d'|' -f2 |
    sed -nE 's/^#([0-9]+)$/\1/p' |
    sort -u
}

normalize_issue_key() {
  local raw="${1:-}"

  if [[ "$raw" =~ \#([0-9]+) ]]; then
    echo "#${BASH_REMATCH[1]}"
    return 0
  fi

  return 1
}
