# File Versioning Utilities

This directory contains reusable utilities for file-level version control workflows.

## Scope

Utilities here support:

- Pure git operations (branches, commits, working tree)
- GitHub CLI operations
- Repository and version control workflows

## Current Structure

- **`git/`** - Pure git operation utilities
  - Repository validation
  - Branch management
  - Working tree operations
  - Commit operations
  - Synchronization utilities

For details, see `git/README.md`

## Adding New File Versioning Utilities

When adding a utility:

1. **Is it a pure git operation?** → Place in `git/`
2. **Is it GitHub-specific?** → Could create `github/` directory
3. **Is it generic VC logic?** → Consider placing in parent `versioning/`

Document the utility in the appropriate README.
