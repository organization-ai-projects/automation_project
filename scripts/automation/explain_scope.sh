#!/usr/bin/env bash
set -euo pipefail

# Explain how scope/category are resolved for current staged files.

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"
# shellcheck source=scripts/common_lib/automation/scope_resolver.sh
source "$ROOT_DIR/scripts/common_lib/automation/scope_resolver.sh"

require_git_repo
cd "$ROOT_DIR"

FILES="$(git diff --cached --name-only --diff-filter=ACMRUD)"
if [[ -z "$FILES" ]]; then
  info "No staged files."
  exit 0
fi

echo "Staged files:"
printf '%s\n' "$FILES" | sed 's/^/  - /'
echo ""

echo "Resolved scope per file:"
while IFS= read -r file; do
  [[ -z "$file" ]] && continue
  if scope="$(resolve_scope_from_path "$file" 2>/dev/null)"; then
    printf '  - %s -> %s\n' "$file" "$scope"
  elif is_shell_path_file "$file"; then
    printf '  - %s -> shell-candidate\n' "$file"
  elif is_markdown_path_file "$file"; then
    printf '  - %s -> markdown-candidate\n' "$file"
  else
    printf '  - %s -> other-candidate\n' "$file"
  fi
done <<< "$FILES"
echo ""

echo "Detected format categories:"
collect_format_categories_from_files "$FILES" | sed 's/^/  - /'
echo ""

echo "Required scope set (commit-msg validation):"
REQUIRED_SCOPES="$(detect_required_scopes_from_staged_files)"
if [[ -n "$REQUIRED_SCOPES" ]]; then
  printf '%s\n' "$REQUIRED_SCOPES" | sed 's/^/  - /'
else
  echo "  - (none)"
fi

