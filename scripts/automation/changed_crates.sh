#!/usr/bin/env bash
set -euo pipefail

# Output the crates touched in a git diff
# Usage: ./changed_crates.sh [ref1] [ref2]
# If no refs provided, uses HEAD and working tree

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"
# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"

require_git_repo

cd "$ROOT_DIR"

# Determine refs to compare
if [[ "$#" -eq 2 ]]; then
  REF1="$1"
  REF2="$2"
  info "Comparing $REF1...$REF2"
  CHANGED_FILES=$(git diff --name-only "$REF1" "$REF2" || true)
elif [[ "$#" -eq 1 ]]; then
  REF1="$1"
  info "Comparing $REF1...HEAD"
  CHANGED_FILES=$(git diff --name-only "$REF1" HEAD || true)
else
  info "Comparing working tree with HEAD"
  CHANGED_FILES=$(git diff --name-only HEAD || true)
fi

if [[ -z "$CHANGED_FILES" ]]; then
  info "No changed files."
  exit 0
fi

# Extract crate paths from changed files
# Crates are in projects/libraries/* and projects/products/*
CRATE_PATHS=$(echo "$CHANGED_FILES" | grep -E "^projects/(libraries|products)/" | \
  while read -r file; do
    # Find the directory containing Cargo.toml
    dir=$(dirname "$file")
    while [[ "$dir" != "." && "$dir" != "/" ]]; do
      if [[ -f "$ROOT_DIR/$dir/Cargo.toml" ]]; then
        echo "$dir"
        break
      fi
      dir=$(dirname "$dir")
    done
  done | sort -u || true)

if [[ -z "$CRATE_PATHS" ]]; then
  info "No crates affected."
  exit 0
fi

info "Changed crates:"
echo "$CRATE_PATHS" | while read -r crate_path; do
  if [[ -f "$ROOT_DIR/$crate_path/Cargo.toml" ]]; then
    CRATE_NAME=$(grep -E "^name\s*=" "$ROOT_DIR/$crate_path/Cargo.toml" | head -1 | sed 's/.*"\(.*\)".*/\1/')
    echo "  - $CRATE_NAME ($crate_path)"
  else
    echo "  - $crate_path"
  fi
done

# Output just the paths for scripting
if [[ "${OUTPUT_FORMAT:-}" == "paths" ]]; then
  echo "$CRATE_PATHS"
fi
