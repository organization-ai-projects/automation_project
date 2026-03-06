#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154


# CLI/help helpers for generate_pr_description.sh.

pr_print_usage() {
  cat <<USAGE
Usage: ${SCRIPT_PATH} [--keep-artifacts] [--debug] [--duplicate-mode MODE] [--auto-edit PR_NUMBER] [--refresh-pr PR_NUMBER] MAIN_PR_NUMBER [OUTPUT_FILE]
       ${SCRIPT_PATH} --dry-run [--base BRANCH] [--head BRANCH] [--create-pr] [--allow-partial-create] [--duplicate-mode MODE] [--debug] [--auto-edit PR_NUMBER|--refresh-pr PR_NUMBER] [--validation-only] [--yes] [OUTPUT_FILE]
       ${SCRIPT_PATH} --auto [--base BRANCH] [--head BRANCH] [--debug] [--yes]
USAGE
}

pr_print_help() {
  pr_print_usage
  cat <<'HELP'

Notes:
  (default)      With no mode/arguments, behaves like --auto.
  --dry-run       Extract PRs from local git history (base..head).
  --create-pr     In dry-run mode, attempts GitHub enrichment before creating the PR.
  --auto-edit     Generate body in memory and update an existing PR directly.
  --refresh-pr    Alias of --auto-edit.
  --validation-only  In --auto-edit/--refresh-pr mode, update only the "Validation Gate" section.
  --duplicate-mode  Duplicate handling mode: safe | auto-close.
  --debug         Print extraction/classification trace to stderr.
  --auto          RAM-first mode: dry-run + create-pr, body kept in memory.
HELP
}

pr_usage_error() {
  local message="$1"
  echo "Error: ${message}" >&2
  pr_print_usage >&2
  exit "$E_USAGE"
}

pr_require_option_value() {
  local option_name="$1"
  local option_value="${2:-}"
  if [[ -z "$option_value" || "$option_value" == --* ]]; then
    pr_usage_error "${option_name} requires a value."
  fi
}
