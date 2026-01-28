# Versioning Utilities

This directory contains reusable utility functions specific to version control workflows.

## Scope

Utilities here support version control operations:

- Git repository and branch management
- Working tree state validation
- Commit and staging operations
- Synchronization utilities

## Current Structure

- **`file_versioning/`** - Utilities for file-level version control
  - `git/` - Git-specific operations

For details on git utilities, see `file_versioning/git/README.md`

## Adding New Versioning Utilities

When adding a versioning utility:

1. **Is it git-specific?** → Place in `file_versioning/git/`
2. **Is it about a different VC tool?** → Create new directory (e.g., `gitlab/`)
3. **Is it generic VC logic?** → Consider if it belongs here or in `core/`

Document the utility in the appropriate README at each level.
