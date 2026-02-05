# Common Lib Documentation

This directory contains reusable function libraries sourced by other scripts.

## Role in the Project

This directory is responsible for providing reusable utility functions and abstractions that standardize common operations across all scripts.
It interacts mainly with:

- All automation and versioning scripts
- Core system utilities (logging, file operations, network)
- Git command-line interface
- String manipulation and validation

## Directory Structure

```plaintext
common_lib/
├── core/                               # Core utilities for all scripts
│   ├── command.sh                      # Command execution and validation
│   ├── file_operations.sh              # File and directory operations
│   ├── logging.sh                      # Consistent logging functions
│   ├── network_utils.sh                # Network-related utilities
│   └── string_utils.sh                 # String manipulation utilities
└── versioning/                         # Version control utilities
    └── file_versioning/                # File-level version control
        └── git/                        # Git-specific operations
            ├── branch.sh               # Branch management
            ├── commit.sh               # Commit operations
            ├── repo.sh                 # Repository validation
            ├── staging.sh              # Staging/index operations
            ├── synch.sh                # Synchronization utilities
            └── working_tree.sh         # Working tree state
```

## Organization Principle

Utility libraries are organized by **scope and tool**:

- **`core/`** - Core utilities used across all scripts (logging, commands, files, strings, network)
- **`versioning/file_versioning/git/`** - Git-specific utilities (repository, branches, commits, etc.)

## Scope

Scripts here:

- Define reusable functions for other scripts
- Should NOT be executed directly (they are `source`d, not run)
- Focus on a specific domain or tool
- Are sourced via: `source "$ROOT_DIR/scripts/common_lib/core/logging.sh"`

## Core Utilities (`core/`)

Essential utilities used across the project:

- `logging.sh` - Consistent logging (info, warn, die)
- `command.sh` - Command utilities (command_exists, require_cmd, retry_command)
- `file_operations.sh` - File helpers (file_exists, dir_exists, backup_file)
- `string_utils.sh` - String manipulation (to_upper, to_lower, trim, contains)
- `network_utils.sh` - Network helpers (url_reachable, download_file)

## Git Utilities (`versioning/file_versioning/git/`)

Git-specific utilities:

- `repo.sh` - Repository validation (require_git_repo)
- `branch.sh` - Branch operations (create, exists, delete, protect, track)
- `working_tree.sh` - Working tree state (clean, dirty, untracked)
- `staging.sh` - Staging/index operations (add, reset, status)
- `commit.sh` - Commit operations (commit, amend, has changes)
- `synch.sh` - Synchronization (fetch with prune)

## Adding New Utilities

When adding a utility function:

1. **Identify the domain** - What does it do? (logging, commands, git operations, etc.)
2. **Find the right file** - Does it fit existing file or need a new one?
3. **Keep it focused** - One file = one domain/tool
4. **Document it** - Add description to this README and in the file itself

## Usage Example

```bash
#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Source utilities
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
source "$ROOT_DIR/scripts/common_lib/core/command.sh"

# Use them
require_cmd "git"
info "Starting deployment..."
die "Something went wrong"
```
