#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# PR-view helpers shared across PR modules.

pr_get_pr_body() {
  local pr_number="$1"
  local fallback_context="$2"
  local context_payload=""
  local pr_body=""

  context_payload="$(github_pr_body_context "" "$pr_number" || true)"
  if [[ "$context_payload" == *$'\x1f'* ]]; then
    local va_tail
    va_tail="${context_payload#*$'\x1f'}"
    printf '%s' "${va_tail%%$'\x1f'*}"
    return
  fi

  pr_body="$(github_pr_field "" "$pr_number" "body" 2>/dev/null || true)"
  if [[ -n "$pr_body" ]]; then
    printf '%s' "$pr_body"
    return
  fi

  pr_gh_optional "$fallback_context" pr view "$pr_number" --json body -q '.body'
}
