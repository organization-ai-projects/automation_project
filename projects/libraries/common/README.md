# Common Library Documentation

This directory contains shared utilities and types for the entire automation project.

## Role in the Project

This library is responsible for providing foundational types and utilities used across all other crates in the workspace. It includes ID generation, name validation, error types, and string manipulation that form the basis for all other modules.

It interacts mainly with:

- All other libraries and products (as a dependency)
- Identity library - For ID generation
- Various products - For common type definitions

## Directory Structure

```
common/
├── Cargo.toml          # Package configuration
├── README.md           # This file
├── documentation/      # Additional documentation
│   └── TOC.md
└── src/               # Source code
    ├── lib.rs
    ├── id.rs
    ├── name.rs
    └── ...
```

## Files

- `README.md`: This file.
- `Cargo.toml`: Package configuration.
- `documentation/`: Additional documentation.
- `src/`: Source code.


## Overview

This library provides foundational types and utilities used across all other crates in the workspace. It includes ID generation, name validation, error types, and string manipulation.

## Features

- **Id128** - Custom 128-bit unique identifier with embedded timestamp
- **CommonID** - Generic ID wrapper type
- **Name** - Validated name type for entities
- **ErrorType** - Common error classification
- **String Utilities** - Safe string manipulation functions

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
common = { path = "../common" }
```

## Usage

### Generate unique IDs

```rust
use common::Id128;

// Create a new unique ID
let id = Id128::new(1, None, None);

// Convert to hex string (32 characters)
let hex = id.to_hex();
println!("ID: {}", hex);

// Parse from hex
let parsed = Id128::from_hex(&hex).expect("valid hex");

// Extract components
println!("Timestamp: {} ms", id.timestamp_ms());
println!("Node ID: {}", id.node_id());
println!("Sequence: {}", id.seq());
```

### ID structure

The 128-bit ID contains:

| Bytes | Component  | Description                             |
| ----- | ---------- | --------------------------------------- |
| 0-5   | Timestamp  | 48-bit millisecond timestamp            |
| 6-7   | Node ID    | Machine/agent identifier                |
| 8-9   | Process ID | Process identifier                      |
| 10-11 | Boot ID    | Changes each program start              |
| 12-15 | Sequence   | Counter for same-millisecond uniqueness |

### Persistent node ID

```rust
use common::Id128;
use std::path::Path;

// Load or create a stable node ID for this machine
let node_id = Id128::load_or_create_node_id(Path::new("/var/lib/myapp/node_id"));
let id = Id128::new(node_id, None, None);
```

### String utilities

```rust
use common::{trim_lossy, truncate_utf8};

// Trim with lossy UTF-8 handling
let trimmed = trim_lossy("  hello  ");

// Truncate to max bytes (UTF-8 safe)
let truncated = truncate_utf8("Hello, 世界!", 10);
```

## Examples

### Name Validation

```rust
use common::Name;

let name = Name::new("valid_name").unwrap();
assert_eq!(name.as_str(), "valid_name");
```

## Thread Safety

`Id128::new()` is thread-safe and guarantees no duplicates even under high concurrency. The internal sequence counter and monotonic timestamp ensure uniqueness.

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/common/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
