#!/usr/bin/env bash
set -euo pipefail

# Clean build artifacts from the workspace
# Removes target/, ui_dist/, and other build artifacts

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

info "Cleaning build artifacts..."

# Track total size cleaned
TOTAL_SIZE=0

# Function to calculate directory size
get_dir_size() {
  local dir="$1"
  if [[ -d "$dir" ]]; then
    du -sh "$dir" 2>/dev/null | cut -f1
  else
    echo "0"
  fi
}

# Clean cargo target directory
if [[ -d "target" ]]; then
  SIZE=$(get_dir_size "target")
  info "Removing target/ ($SIZE)..."
  rm -rf target
  info "✓ target/ removed"
fi

# Clean UI dist directories
info "Scanning for UI dist directories..."
while IFS= read -r -d '' ui_dist; do
  if [[ -d "$ui_dist" ]]; then
    SIZE=$(get_dir_size "$ui_dist")
    info "Removing $ui_dist ($SIZE)..."
    rm -rf "$ui_dist"
    info "✓ $ui_dist removed"
  fi
done < <(find "$ROOT_DIR/projects" -type d -name "ui_dist" -print0 2>/dev/null || true)

# Clean node_modules if any
info "Scanning for node_modules directories..."
while IFS= read -r -d '' node_modules; do
  if [[ -d "$node_modules" ]]; then
    SIZE=$(get_dir_size "$node_modules")
    info "Removing $node_modules ($SIZE)..."
    rm -rf "$node_modules"
    info "✓ $node_modules removed"
  fi
done < <(find "$ROOT_DIR" -type d -name "node_modules" -print0 2>/dev/null || true)

# Clean Cargo.lock in subdirectories (keep root one)
info "Scanning for stale Cargo.lock files..."
while IFS= read -r -d '' cargo_lock; do
  # Skip root Cargo.lock
  if [[ "$cargo_lock" != "$ROOT_DIR/Cargo.lock" ]]; then
    info "Removing $cargo_lock..."
    rm -f "$cargo_lock"
    info "✓ $cargo_lock removed"
  fi
done < <(find "$ROOT_DIR/projects" -type f -name "Cargo.lock" -print0 2>/dev/null || true)

# Clean test artifacts
info "Removing test artifacts..."
find "$ROOT_DIR" -type f -name "*.profraw" -delete 2>/dev/null || true
find "$ROOT_DIR" -type f -name "*.gcda" -delete 2>/dev/null || true
find "$ROOT_DIR" -type f -name "*.gcno" -delete 2>/dev/null || true

# Clean temporary files
info "Removing temporary files..."
find "$ROOT_DIR" -type f -name "*~" -delete 2>/dev/null || true
find "$ROOT_DIR" -type f -name "*.bak" -delete 2>/dev/null || true
find "$ROOT_DIR" -type f -name "*.tmp" -delete 2>/dev/null || true

# Run cargo clean to be thorough
info "Running cargo clean..."
cargo clean

info "✅ Build artifacts cleaned successfully!"
info ""
info "To rebuild, run: cargo build"
