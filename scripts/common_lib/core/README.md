# Core Utilities Documentation

Essential utility functions used across all scripts in the project.

## Role in the Project

This directory is responsible for providing foundational utility functions that are generic and cross-cutting, supporting all scripts with consistent logging, command execution, file operations, string manipulation, and network utilities.
It interacts mainly with:

- All automation and versioning scripts
- System commands and shell utilities
- File system operations
- Network resources

## Role in the Project

This directory is responsible for providing foundational utility functions that are generic and cross-cutting, supporting all scripts with consistent logging, command execution, file operations, string manipulation, and network utilities.
It interacts mainly with:

- All automation and versioning scripts
- System commands and shell utilities
- File system operations
- Network resources

## Directory Structure

```plaintext
core/
├── command.sh           # Command execution and validation utilities
├── file_operations.sh   # File and directory operation utilities
├── logging.sh           # Core logging functions with consistent formatting
├── network_utils.sh     # Network-related utilities
└── string_utils.sh      # String manipulation utilities
```

## Scope

This directory contains foundational utilities that:

- Are generic (not specific to git or versioning)
- Support all other scripts in the project
- Provide consistent behavior across the codebase

## Current Modules

### logging.sh

Core logging functions with consistent formatting:

- `info()` - Info-level log message
- `warn()` - Warning-level log message
- `die()` - Error message + exit

**Usage**: `source "$ROOT_DIR/scripts/common_lib/core/logging.sh"`

### command.sh

Command execution and validation utilities:

- `require_cmd()` - Require a command is available
- `command_exists()` - Check if command exists
- `retry_command()` - Retry a command with backoff

**Usage**: `source "$ROOT_DIR/scripts/common_lib/core/command.sh"`

### file_operations.sh

File and directory operation utilities:

- `file_exists()` - Check if file exists
- `dir_exists()` - Check if directory exists
- `backup_file()` - Backup a file
- `ensure_dir()` - Ensure directory exists

**Usage**: `source "$ROOT_DIR/scripts/common_lib/core/file_operations.sh"`

### string_utils.sh

String manipulation utilities:

- `string_to_upper()` - Convert to uppercase
- `string_to_lower()` - Convert to lowercase
- `string_trim()` - Trim whitespace
- `string_contains()` - Check if contains substring

**Usage**: `source "$ROOT_DIR/scripts/common_lib/core/string_utils.sh"`

### network_utils.sh

Network-related utilities:

- `url_reachable()` - Check if URL is reachable
- `download_file()` - Download a file from URL

**Usage**: `source "$ROOT_DIR/scripts/common_lib/core/network_utils.sh"`

## Adding New Core Utilities

When adding a new utility:

1. **Is it generic?** - Should work across different domains
2. **Find the right file** - Does it fit in existing module or need new one?
3. **Keep it focused** - One file = one domain
4. **Document it** - Add to this README and in the script file

Good candidates for core utilities:

- Cross-cutting concerns (logging, error handling)
- Generic operations (file, string, network)
- Foundational features used by multiple scripts
