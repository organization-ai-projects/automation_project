#!/usr/bin/env bash
set -euo pipefail

# Prepare a new release: version bump, changelog, and git tag
# Usage: ./release_prepare.sh <version> [--auto-changelog]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/core/command.sh
source "$ROOT_DIR/scripts/common_lib/core/command.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/working_tree.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/working_tree.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/branch.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/branch.sh"

require_git_repo
require_clean_tree
require_cmd cargo

if [[ "$#" -lt 1 ]]; then
  echo "Usage: $0 <version> [--auto-changelog]" >&2
  echo "Example: $0 1.2.0" >&2
  echo "Example: $0 1.2.0 --auto-changelog" >&2
  exit 1
fi

VERSION="$1"
AUTO_CHANGELOG=false

if [[ "${2:-}" == "--auto-changelog" ]]; then
  AUTO_CHANGELOG=true
fi

# Validate version format (semver)
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.-]+)?$ ]]; then
  die "Invalid version format: $VERSION. Expected semver format (e.g., 1.2.0 or 1.2.0-beta.1)"
fi

cd "$ROOT_DIR"

info "Preparing release v$VERSION..."

# 1. Check current branch is main
CURRENT_BRANCH="$(get_current_branch)"
if [[ "$CURRENT_BRANCH" != "main" ]]; then
  warn "Current branch is '$CURRENT_BRANCH', not 'main'."
  warn "Releases should typically be created from 'main' branch."
  read -p "Continue anyway? (y/N) " -n 1 -r
  echo
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    die "Release preparation cancelled."
  fi
fi

# 2. Ensure all tests pass
info "Running tests..."
if ! cargo test --workspace; then
  die "Tests failed. Fix tests before releasing."
fi

# 3. Run security audit
info "Running security audit..."
if command -v cargo-audit >/dev/null 2>&1; then
  if ! cargo audit; then
    warn "Security vulnerabilities detected!"
    read -p "Continue with release anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
      die "Release preparation cancelled due to security issues."
    fi
  fi
fi

# 4. Update version in workspace Cargo.toml
info "Updating version to $VERSION..."

if grep -q "^version = " "$ROOT_DIR/Cargo.toml"; then
  # Workspace has a version field
  sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" "$ROOT_DIR/Cargo.toml"
  rm -f "$ROOT_DIR/Cargo.toml.bak"
  info "✓ Updated workspace version"
fi

# Update version in all member crates
info "Updating version in member crates..."
find "$ROOT_DIR/projects" -name "Cargo.toml" -type f | while read -r cargo_toml; do
  if grep -q "^version = " "$cargo_toml"; then
    sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" "$cargo_toml"
    rm -f "$cargo_toml.bak"
    info "  ✓ Updated $(dirname "$cargo_toml")"
  fi
done

# 5. Generate or update CHANGELOG
CHANGELOG="$ROOT_DIR/CHANGELOG.md"

if [[ "$AUTO_CHANGELOG" == true ]]; then
  info "Generating changelog..."

  # Get the last tag
  LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")

  if [[ -z "$LAST_TAG" ]]; then
    info "No previous tag found. Generating full changelog..."
    COMMITS=$(git log --oneline --no-merges)
  else
    info "Generating changelog since $LAST_TAG..."
    COMMITS=$(git log "$LAST_TAG..HEAD" --oneline --no-merges)
  fi

  # Create/update changelog
  {
    echo "# Changelog"
    echo ""
    echo "## [v$VERSION] - $(date +%Y-%m-%d)"
    echo ""
    echo "### Changes"
    echo ""
    echo "$COMMITS" | sed 's/^/- /'
    echo ""

    # Append existing changelog if it exists
    if [[ -f "$CHANGELOG" ]]; then
      tail -n +2 "$CHANGELOG"
    fi
  } > "$CHANGELOG.new"

  mv "$CHANGELOG.new" "$CHANGELOG"
  info "✓ Changelog updated"
else
  info "Skipping automatic changelog generation."
  info "Please update $CHANGELOG manually."
  read -p "Press Enter when changelog is ready..." -r
fi

# 6. Commit changes
info "Committing release changes..."
git add Cargo.toml projects/*/Cargo.toml "$CHANGELOG" 2>/dev/null || true
git commit -m "chore: prepare release v$VERSION

Release preparation for version $VERSION.

Co-Authored-By: Release Script <release@automation-project.local>"

# 7. Create git tag
info "Creating git tag v$VERSION..."
git tag -a "v$VERSION" -m "Release v$VERSION"

info "✅ Release v$VERSION prepared!"
info ""
info "Next steps:"
info "  1. Review the changes: git show HEAD"
info "  2. Push to remote: git push origin main --tags"
info "  3. Create GitHub release: gh release create v$VERSION"
info ""
info "To undo: git reset --hard HEAD^ && git tag -d v$VERSION"
