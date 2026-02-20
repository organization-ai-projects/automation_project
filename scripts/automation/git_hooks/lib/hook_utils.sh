#!/usr/bin/env bash

# Generic hook utilities (non-policy helpers).

# shellcheck source=scripts/automation/git_hooks/lib/file_types.sh
source "$(git rev-parse --show-toplevel)/scripts/automation/git_hooks/lib/file_types.sh"

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
