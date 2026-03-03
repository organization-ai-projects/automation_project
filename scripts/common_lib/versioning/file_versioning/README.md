# File Versioning Utilities Documentation

This directory contains reusable utilities for file-level version control workflows.

## Role in the Project

This directory is responsible for providing utilities that abstract Git operations for file-level version control, including repository validation, branch management, working tree state, commit operations, and synchronization.
It interacts mainly with:

- Git command-line interface
- Version control workflows and orchestrators
- Repository state and history
- Staging area and working tree

## Directory Structure

```plaintext
file_versioning/
├── conventions.sh         # Shared naming/message contract (commit + PR title)
├── git/                   # Pure git operation utilities
│   ├── commands.sh        # Local VCS backend wrappers (vcs_local_*)
│   ├── branch.sh          # Branch management
│   ├── commit.sh          # Commit operations
│   ├── repo.sh            # Repository validation
│   ├── staging.sh         # Staging/index operations
│   ├── synch.sh           # Synchronization utilities
│   └── working_tree.sh    # Working tree state
└── github/                # Shared GitHub automation helpers
    ├── commands.sh        # Remote VCS backend wrappers (vcs_remote_*)
    ├── issue_helpers.sh   # Shared issue reference/status comment helpers
    └── pull_request_lookup.sh  # Shared PR lookup helpers
```

## Files

- `README.md`: This file.
- `conventions.sh`: Shared conventions/validation contract for commit and PR titles.
- `git/`: Pure git operation utilities.
- `github/`: Shared GitHub automation helpers.

## Scope

Utilities here support:

- Shared commit/PR title conventions and validation
- Local VCS operations via `vcs_local_*`
- Remote provider operations via `vcs_remote_*`
- Repository and version control workflows

## Current Structure

- **`git/`** - Pure git operation utilities
  - Repository validation
  - Branch management
  - Working tree operations
  - Commit operations
  - Synchronization utilities
- **`github/`** - Shared GitHub automation helpers
  - Issue task-list reference extraction
  - Sub-issue lookup via GraphQL
  - Marker-based status comment upsert

For details, see `git/README.md`
For remote helpers, see `github/README.md`

## Adding New File Versioning Utilities

When adding a utility:

1. **Is it a pure git operation?** → Place in `git/`
2. **Is it GitHub-specific?** → Could create `github/` directory
3. **Is it generic VC logic?** → Consider placing in parent `versioning/`

Document the utility in the appropriate README.
