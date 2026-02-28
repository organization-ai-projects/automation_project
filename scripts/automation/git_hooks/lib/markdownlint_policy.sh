#!/usr/bin/env bash

# Shared markdownlint resolution and execution helpers.

markdownlint_policy_extract_first_semver() {
  local raw="${1:-}"
  printf '%s\n' "$raw" | grep -Eo '[0-9]+\.[0-9]+\.[0-9]+' | head -n1
}

markdownlint_policy_expected_version() {
  local spec=""

  if command -v node >/dev/null 2>&1 && [[ -f package.json ]]; then
    spec="$(
      node -p "const p=require('./package.json'); (p.devDependencies&&p.devDependencies['markdownlint-cli2']) || (p.dependencies&&p.dependencies['markdownlint-cli2']) || ''" \
        2>/dev/null || true
    )"
  fi

  if [[ -z "$spec" && -f package.json ]]; then
    spec="$(
      grep -E '"markdownlint-cli2"[[:space:]]*:' package.json \
        | head -n1 \
        | sed -E 's/.*:[[:space:]]*"([^"]+)".*/\1/' \
        || true
    )"
  fi

  markdownlint_policy_extract_first_semver "$spec"
}

markdownlint_policy_version_of_bin() {
  local bin="${1:-}"
  local output=""
  [[ -n "$bin" ]] || return 1
  output="$("$bin" --version 2>/dev/null || true)"
  markdownlint_policy_extract_first_semver "$output"
}

markdownlint_policy_resolve_bin() {
  local expected_version="$1"
  local global_bin=""
  local local_bin="./node_modules/.bin/markdownlint-cli2"
  local global_version=""
  local local_version=""

  global_bin="$(command -v markdownlint-cli2 2>/dev/null || true)"
  if [[ -n "$global_bin" ]]; then
    global_version="$(markdownlint_policy_version_of_bin "$global_bin" || true)"
    if [[ "$global_version" == "$expected_version" ]]; then
      printf '%s\n' "$global_bin"
      return 0
    fi
  fi

  if [[ -x "$local_bin" ]]; then
    local_version="$(markdownlint_policy_version_of_bin "$local_bin" || true)"
    if [[ "$local_version" == "$expected_version" ]]; then
      printf '%s\n' "$local_bin"
      return 0
    fi
  fi

  echo "❌ markdownlint-cli2 ${expected_version} is required (source: package.json)." >&2
  if [[ -n "$global_bin" ]]; then
    echo "   - global: ${global_bin} (version: ${global_version:-unknown})" >&2
  else
    echo "   - global: not found" >&2
  fi
  if [[ -x "$local_bin" ]]; then
    echo "   - local: ${local_bin} (version: ${local_version:-unknown})" >&2
  else
    echo "   - local: not found" >&2
  fi
  echo "   Install matching global version, or install repo dependencies locally." >&2
  return 1
}

markdownlint_policy_run_checks() {
  local markdown_files="$1"
  local -a files=()
  local file
  local expected_version=""
  local markdownlint_bin=""

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    files+=("$file")
  done <<< "$markdown_files"

  if [[ ${#files[@]} -eq 0 ]]; then
    return 0
  fi

  if [[ ! -f package.json ]]; then
    echo "❌ Markdown lint requires package.json at repository root."
    return 1
  fi

  expected_version="$(markdownlint_policy_expected_version)"
  if [[ -z "$expected_version" ]]; then
    echo "❌ Unable to determine markdownlint-cli2 expected version from package.json."
    return 1
  fi

  if ! markdownlint_bin="$(markdownlint_policy_resolve_bin "$expected_version")"; then
    return 1
  fi

  "$markdownlint_bin" "#target" "#node_modules" "${files[@]}"
}

markdownlint_policy_run_fix_and_checks() {
  local markdown_files="$1"
  local -a files=()
  local file
  local expected_version=""
  local markdownlint_bin=""

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    files+=("$file")
  done <<< "$markdown_files"

  if [[ ${#files[@]} -eq 0 ]]; then
    return 0
  fi

  if [[ ! -f package.json ]]; then
    echo "❌ Markdown lint requires package.json at repository root."
    return 1
  fi

  expected_version="$(markdownlint_policy_expected_version)"
  if [[ -z "$expected_version" ]]; then
    echo "❌ Unable to determine markdownlint-cli2 expected version from package.json."
    return 1
  fi

  if ! markdownlint_bin="$(markdownlint_policy_resolve_bin "$expected_version")"; then
    return 1
  fi

  "$markdownlint_bin" --fix "#target" "#node_modules" "${files[@]}"
  "$markdownlint_bin" "#target" "#node_modules" "${files[@]}"
}
