#!/usr/bin/env bash

# Generic hook utilities (non-policy helpers).

# shellcheck source=scripts/common_lib/automation/file_types.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/automation/file_types.sh"

hook_utils_resolve_upstream_branch() {
  local upstream
  upstream="$(git rev-parse --abbrev-ref --symbolic-full-name @{u} 2>/dev/null || echo "")"
  if [[ -z "$upstream" ]]; then
    echo "⚠️  No upstream branch detected. Falling back to origin/dev for scope detection." >&2
    upstream="origin/dev"
  fi
  printf '%s\n' "$upstream"
}

hook_utils_collect_push_commits() {
  local upstream="$1"
  git log "$upstream"..HEAD --format=%B 2>/dev/null || true
}

hook_utils_compute_changed_files() {
  local upstream="$1"
  local files=""

  files=$(git diff --name-only "${upstream}"..HEAD 2>/dev/null || true)
  if [[ -n "$files" ]]; then
    printf '%s\n' "$files"
    return 0
  fi

  if git rev-parse --verify --quiet origin/dev >/dev/null; then
    local base
    base=$(git merge-base origin/dev HEAD 2>/dev/null || true)
    if [[ -n "$base" ]]; then
      files=$(git diff --name-only "${base}"..HEAD 2>/dev/null || true)
      if [[ -n "$files" ]]; then
        printf '%s\n' "$files"
        return 0
      fi
    fi
  fi

  files=$(git diff-tree --no-commit-id --name-only -r HEAD 2>/dev/null || true)
  printf '%s\n' "$files"
}

hook_utils_run_shell_syntax_checks() {
  local files="$1"
  local checked=0
  local file

  while IFS= read -r file; do
    if is_shell_file "$file"; then
      echo "   - bash -n $file"
      bash -n "$file"
      checked=1
    fi
  done <<< "$files"

  if [[ $checked -eq 0 ]]; then
    echo "   (no shell scripts changed)"
  fi
}

hook_utils_collect_markdown_files() {
  local files="$1"
  local file

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    if is_markdown_path_file "$file" && [[ -f "$file" ]]; then
      printf '%s\n' "$file"
    fi
  done <<< "$files"
}

hook_utils_run_markdownlint_checks() {
  local markdown_files="$1"
  local -a files=()
  local file

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    files+=("$file")
  done <<< "$markdown_files"

  if [[ ${#files[@]} -eq 0 ]]; then
    return 0
  fi

  if ! command -v pnpm >/dev/null 2>&1; then
    echo "❌ Markdown lint requires pnpm when markdown files are changed."
    echo "   Install: corepack enable && corepack prepare pnpm@9 --activate"
    return 1
  fi

  if [[ ! -f package.json ]]; then
    echo "❌ Markdown lint requires package.json at repository root."
    return 1
  fi

  if [[ ! -f node_modules/.bin/markdownlint-cli2 ]]; then
    echo "❌ Markdown lint dependencies are missing for changed markdown files."
    echo "   Run: pnpm install --frozen-lockfile"
    return 1
  fi

  pnpm run -s lint-md-files -- "${files[@]}"
}
