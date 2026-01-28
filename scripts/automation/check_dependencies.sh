#!/usr/bin/env bash
set -euo pipefail

# Check for outdated or missing dependencies in Cargo workspace

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/core/command.sh
source "$ROOT_DIR/scripts/common_lib/core/command.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"

require_git_repo
require_cmd cargo

cd "$ROOT_DIR"

info "Checking Cargo dependencies..."

# Check if cargo-outdated is installed
if ! command -v cargo-outdated >/dev/null 2>&1; then
  warn "cargo-outdated not found. Install with: cargo install cargo-outdated"
  warn "Skipping outdated dependencies check."
else
  info "Checking for outdated dependencies..."
  cargo outdated --workspace --root-deps-only || warn "Some dependencies may be outdated."
fi

# Check for missing dependencies (cargo check)
info "Verifying dependencies are resolvable..."
if cargo check --workspace --all-targets; then
  info "✓ All dependencies are resolvable."
else
  die "Dependency check failed. Some dependencies may be missing or incompatible."
fi

# Optional: check for unused dependencies with cargo-udeps
if command -v cargo-udeps >/dev/null 2>&1; then
  info "Checking for unused dependencies..."
  cargo +nightly udeps --workspace || warn "Some dependencies may be unused."
else
  info "cargo-udeps not found. Install with: cargo install cargo-udeps"
  info "Skipping unused dependencies check."
fi

info "✓ Dependency check complete."
