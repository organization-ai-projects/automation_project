# Versioning Utilities Documentation

This directory contains reusable utility functions specific to version control workflows.

## Role in the Project

This directory is responsible for providing reusable abstractions for version control operations, primarily focused on Git repository management, branch operations, and working tree state.
It interacts mainly with:

- Git version control system
- Automation and versioning scripts
- Repository state and history
- Branch and commit management

## Directory Structure

```plaintext
versioning/
└── file_versioning/     # Utilities for file-level version control
    └── git/             # Git-specific operations
```

## Files

- `README.md`: This file.
- `file_versioning/`: Utilities for file-level version control.

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
