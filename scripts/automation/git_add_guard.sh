#!/usr/bin/env bash
set -euo pipefail

# Guarded staging command.
# Enforces repository staging policy before applying git add for real.
#
# Policy:
# - Block broad staging commands ('.', '-A', '--all') unless explicitly allowed.
# - Block mixed docs + non-doc staged sets.
# - Block staged sets touching more than one crate in projects/.
#
# Bypass:
# - ALLOW_BROAD_STAGE=1 scripts/automation/git_add_guard.sh -A
# - ALLOW_MIXED_STAGE=1 scripts/automation/git_add_guard.sh <paths...>

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"
# shellcheck source=scripts/common_lib/automation/scope_resolver.sh
source "$ROOT_DIR/scripts/common_lib/automation/scope_resolver.sh"
# shellcheck source=scripts/common_lib/automation/change_policy.sh
source "$ROOT_DIR/scripts/common_lib/automation/change_policy.sh"

require_git_repo
cd "$ROOT_DIR"

if [[ $# -eq 0 ]]; then
  die "Usage: scripts/automation/git_add_guard.sh <pathspec...>"
fi

check_broad_args() {
  local arg
  for arg in "$@"; do
    if [[ "$arg" == "." || "$arg" == "-A" || "$arg" == "--all" ]]; then
      if [[ "${ALLOW_BROAD_STAGE:-}" != "1" ]]; then
        die "Broad staging ('$arg') is blocked. Stage explicit paths (or bypass once with ALLOW_BROAD_STAGE=1)."
      fi
    fi
  done
}

check_broad_args "$@"

# Simulate staging in a temporary index first.
ORIGINAL_INDEX="$(git rev-parse --git-path index)"
TMP_INDEX="$(mktemp /tmp/git-index-guard.XXXXXX)"
trap 'rm -f "$TMP_INDEX"' EXIT

if [[ -f "$ORIGINAL_INDEX" ]]; then
  cp "$ORIGINAL_INDEX" "$TMP_INDEX"
else
  : > "$TMP_INDEX"
fi

GIT_INDEX_FILE="$TMP_INDEX" git add "$@"

PROJECTED_STAGED_FILES="$(GIT_INDEX_FILE="$TMP_INDEX" git diff --cached --name-only --diff-filter=ACMRU)"

if [[ -z "$PROJECTED_STAGED_FILES" ]]; then
  warn "No staged files after applying pathspec."
  exit 0
fi

DOC_COUNT=0
NON_DOC_COUNT=0
declare -A CRATES=()

while IFS= read -r file; do
  [[ -z "$file" ]] && continue

  if is_docs_file "$file"; then
    DOC_COUNT=$((DOC_COUNT + 1))
  else
    NON_DOC_COUNT=$((NON_DOC_COUNT + 1))
  fi

  if crate_scope="$(resolve_scope_from_path "$file")"; then
    CRATES["$crate_scope"]=1
  fi
done <<< "$PROJECTED_STAGED_FILES"

if [[ "${ALLOW_MIXED_STAGE:-}" != "1" ]]; then
  if is_mixed_docs_and_non_docs_change "$PROJECTED_STAGED_FILES"; then
    echo "❌ Staging policy violation: mixed docs + non-doc files in index." >&2
    echo "   Split into separate commits." >&2
    echo "   Bypass (exception): ALLOW_MIXED_STAGE=1 scripts/automation/git_add_guard.sh <paths...>" >&2
    exit 1
  fi

  if has_multiple_scopes_in_files "$PROJECTED_STAGED_FILES"; then
    echo "❌ Staging policy violation: multiple crates detected in index." >&2
    echo "   Detected crates/scopes:" >&2
    for crate in "${!CRATES[@]}"; do
      echo "   - $crate" >&2
    done
    echo "   Split by crate before commit." >&2
    echo "   Bypass (exception): ALLOW_MIXED_STAGE=1 scripts/automation/git_add_guard.sh <paths...>" >&2
    exit 1
  fi
fi

# Policy passed, apply staging for real.
git add "$@"

info "✅ Staging accepted by policy."
info "Staged files:"
echo "$PROJECTED_STAGED_FILES" | sed 's/^/  - /'
