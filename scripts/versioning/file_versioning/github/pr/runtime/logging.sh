#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# Runtime logging and optional gh helpers.

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
