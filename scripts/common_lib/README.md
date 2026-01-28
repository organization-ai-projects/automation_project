# Common Utility Libraries

This directory contains **reusable function libraries** sourced by other scripts.

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
