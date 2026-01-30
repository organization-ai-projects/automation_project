#!/usr/bin/env bash
set -euo pipefail

# Setup Git hooks for the repository
# Creates pre-commit, pre-push, and commit-msg hooks

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

HOOKS_DIR="$ROOT_DIR/.git/hooks"

info "Setting up Git hooks..."

# Pre-commit hook: Run formatting and clippy checks
info "Creating pre-commit hook..."
cat > "$HOOKS_DIR/pre-commit" << 'EOF'
#!/usr/bin/env bash
set -euo pipefail

echo "Running pre-commit checks..."

# Check formatting
if ! cargo fmt --check; then
  echo "Error: Code is not formatted. Run: cargo fmt" >&2
  exit 1
fi

# Run clippy on staged files
if ! cargo clippy --workspace --all-targets -- -D warnings; then
  echo "Error: Clippy warnings detected." >&2
  exit 1
fi

echo "✓ Pre-commit checks passed."
EOF

chmod +x "$HOOKS_DIR/pre-commit"
info "✓ pre-commit hook installed"

# Pre-push hook: Run tests and security audit
info "Creating pre-push hook..."
cat > "$HOOKS_DIR/pre-push" << 'EOF'
#!/usr/bin/env bash
set -euo pipefail

echo "Running pre-push checks..."

# Run tests
if ! cargo test --workspace; then
  echo "Error: Tests failed." >&2
  exit 1
fi

# Check for security vulnerabilities (if cargo-audit is installed)
if command -v cargo-audit >/dev/null 2>&1; then
  if ! cargo audit; then
    echo "Warning: Security vulnerabilities detected." >&2
    # Don't fail the push, just warn
  fi
fi

echo "✓ Pre-push checks passed."
EOF

chmod +x "$HOOKS_DIR/pre-push"
info "✓ pre-push hook installed"

# Commit-msg hook: Validate commit message format
info "Creating commit-msg hook..."
cat > "$HOOKS_DIR/commit-msg" << 'EOF'
#!/usr/bin/env bash
set -euo pipefail

COMMIT_MSG_FILE="$1"
COMMIT_MSG=$(cat "$COMMIT_MSG_FILE")

# Skip validation for merge commits
if [[ "$COMMIT_MSG" =~ ^Merge ]]; then
  exit 0
fi

# Validate commit message format (conventional commits)
# Format: type(scope?): description
# Types: feat, fix, docs, style, refactor, test, chore

PATTERN="^(feat|fix|docs|style|refactor|test|chore|ci|perf|build|revert)(\([a-z0-9_-]+\))?: .{1,80}"

if ! echo "$COMMIT_MSG" | head -1 | grep -qE "$PATTERN"; then
  cat >&2 << 'ERROR'
Error: Commit message does not follow conventional commits format.

Expected format:
  type(scope): description

Types: feat, fix, docs, style, refactor, test, chore, ci, perf, build, revert

Examples:
  feat: add user authentication
  fix(api): handle null pointer in login
  docs: update README with setup instructions
  chore: bump dependency versions

ERROR
  exit 1
fi

echo "✓ Commit message format validated."
EOF

chmod +x "$HOOKS_DIR/commit-msg"
info "✓ commit-msg hook installed"

info "✅ Git hooks setup complete!"
info ""
info "Installed hooks:"
info "  - pre-commit: Runs fmt and clippy checks"
info "  - pre-push: Runs tests and security audit"
info "  - commit-msg: Validates commit message format"
info ""
info "To skip hooks temporarily, use: git commit --no-verify"
