#!/usr/bin/env bash
set -euo pipefail

# Generate test coverage report using cargo-tarpaulin
# Outputs HTML report in target/coverage/

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

info "Generating test coverage report..."

# Check if cargo-tarpaulin is installed
if ! command -v cargo-tarpaulin >/dev/null 2>&1; then
  warn "cargo-tarpaulin not found. Installing..."
  info "Note: This may take several minutes..."
  cargo install cargo-tarpaulin || die "Failed to install cargo-tarpaulin"
fi

# Create coverage directory
mkdir -p target/coverage

# Run tarpaulin with HTML output
info "Running tests with coverage instrumentation..."
info "This may take a while..."

TARPAULIN_ARGS=(
  --workspace
  --all-features
  --out Html
  --output-dir target/coverage
  --timeout 300
  --exclude-files "*/tests/*"
  --exclude-files "*/benches/*"
)

# Optional: Generate multiple output formats
if [[ "${COVERAGE_FORMATS:-html}" =~ "lcov" ]]; then
  TARPAULIN_ARGS+=(--out Lcov)
fi

if [[ "${COVERAGE_FORMATS:-html}" =~ "json" ]]; then
  TARPAULIN_ARGS+=(--out Json)
fi

if cargo tarpaulin "${TARPAULIN_ARGS[@]}"; then
  info "✅ Coverage report generated!"
  info ""
  info "Report location: $ROOT_DIR/target/coverage/index.html"
  info "To view: open target/coverage/index.html"

  # Parse coverage percentage if available
  if [[ -f "target/coverage/tarpaulin-report.html" ]]; then
    COVERAGE=$(grep -oP 'Coverage: \K[\d.]+%' target/coverage/tarpaulin-report.html 2>/dev/null || echo "unknown")
    info "Overall coverage: $COVERAGE"
  fi

  exit 0
else
  die "Coverage generation failed"
fi
