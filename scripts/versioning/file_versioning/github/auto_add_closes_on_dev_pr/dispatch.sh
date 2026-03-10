#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC1091

auto_add_closes_legacy_dispatch() {
  source "${ROOT_GITHUB_DIR}/lib/gh_cli.sh"
  source "${ROOT_GITHUB_DIR}/lib/issue_refs.sh"
  source "${BASH_SOURCE[0]%/*}/cli.sh"
  source "${BASH_SOURCE[0]%/*}/helpers.sh"
  source "${BASH_SOURCE[0]%/*}/workflow.sh"

  local entrypoint_fn="auto_add_closes_run"
  "$entrypoint_fn" "$@"
}

auto_add_closes_try_va_dispatch() {
  local -a va_cmd=()

  if [[ "${VA_AUTO_ADD_CLOSES_WRAPPER_ACTIVE:-0}" == "1" ]]; then
    return 1
  fi

  if [[ "${VA_AUTO_ADD_CLOSES_FORCE_LEGACY:-0}" == "1" ]]; then
    return 1
  fi

  if command -v va >/dev/null 2>&1; then
    va_cmd=(va pr auto-add-closes)
  elif command -v versioning_automation >/dev/null 2>&1; then
    va_cmd=(versioning_automation pr auto-add-closes)
  else
    return 1
  fi

  VA_AUTO_ADD_CLOSES_WRAPPER_ACTIVE=1 "${va_cmd[@]}" "$@"
}

auto_add_closes_dispatch() {
  if auto_add_closes_try_va_dispatch "$@"; then
    return 0
  fi
  auto_add_closes_legacy_dispatch "$@"
}

auto_add_closes_entry_run() {
  auto_add_closes_dispatch "$@"
}
