#!/usr/bin/env bash

# Shared file classification helpers used across hooks, CI, and scripts.

is_docs_file() {
  local file="$1"
  [[ "$file" == documentation/* ]] \
    || [[ "$file" == .github/documentation/* ]] \
    || [[ "$file" == .github/ISSUE_TEMPLATE/* ]] \
    || [[ "$file" == .github/PULL_REQUEST_TEMPLATE/* ]] \
    || [[ "$file" == *.md ]]
}

is_workflow_file() {
  local file="$1"
  [[ "$file" == .github/workflows/* ]]
}

is_script_path_file() {
  local file="$1"
  [[ "$file" == scripts/* ]]
}

is_shell_file() {
  local file="$1"
  [[ -z "$file" || ! -f "$file" ]] && return 1

  if [[ "$file" == *.sh ]]; then
    return 0
  fi

  if [[ -x "$file" ]]; then
    local shebang
    shebang=$(head -n1 "$file" 2>/dev/null || true)
    if [[ "$shebang" =~ ^#!.*(ba)?sh([[:space:]]|$) ]]; then
      return 0
    fi
  fi

  return 1
}

is_test_file() {
  local file="$1"
  [[ "$file" == *"/tests/"* ]] \
    || [[ "$file" == *"_test.rs" ]] \
    || [[ "$file" == *"/tests.rs" ]]
}
