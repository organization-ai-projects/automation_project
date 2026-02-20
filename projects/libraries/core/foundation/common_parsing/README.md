# Common Parsing Library Documentation

This directory contains text parsing utilities for the automation project.

## Role in the Project

This library is responsible for providing parsing utilities across the automation project. It includes cursor-based text parsing, unified diff parsing, and date validation functionality.

It interacts mainly with:

- Version control tools - For diff parsing
- Common calendar library - For date parsing
- Various products - For text processing needs

## Directory Structure

```
common_parsing/
├── Cargo.toml          # Package configuration
├── README.md           # This file
├── documentation/      # Additional documentation
│   └── TOC.md
└── src/               # Source code
    ├── lib.rs
    ├── cursor.rs
    ├── diff.rs
    └── ...
```

## Files

- `README.md`: This file.
- `Cargo.toml`: Package configuration.
- `documentation/`: Additional documentation.
- `src/`: Source code.


## Overview

This library provides parsing utilities including a cursor-based text parser, unified diff parsing, and date validation.

## Features

- **Cursor** - Position-tracking text cursor with line/column information
- **Diff Parsing** - Extract touched file paths from unified diff output
- **Date Parsing** - Validate and parse `YYYY-MM-DD` date strings

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
common_parsing = { path = "../common_parsing" }
```

## Usage

### Cursor-based parsing

```rust
use common_parsing::Cursor;

let mut cursor = Cursor::new("hello\nworld");

while let Some(ch) = cursor.next_char() {
    println!("Char: {} at line {}, col {}", ch, cursor.line(), cursor.column());
}

// Save and restore position
let pos = cursor.position();
cursor.restore(pos);
```

### Parse unified diff

```rust
use common_parsing::parse_unified_diff_touched_paths;

let diff = r#"
--- a/src/main.rs
+++ b/src/main.rs
@@ -1,3 +1,4 @@
+use std::io;
 fn main() {}
"#;

let paths = parse_unified_diff_touched_paths(diff);
// Returns: [PathBuf("src/main.rs")]
```

### Validate date strings

```rust
use common_parsing::parse_date;

assert!(parse_date("2024-01-15").is_some());
assert!(parse_date("invalid").is_none());
assert!(parse_date("2024-13-01").is_none()); // Invalid month
```

## Examples

### Unified Diff Parsing

```rust
use common_parsing::DiffParser;

let diff = "diff --git a/file.txt b/file.txt\n--- a/file.txt\n+++ b/file.txt\n@@ -1 +1 @@\n-Hello\n+Hi";
let parser = DiffParser::new(diff);
let files = parser.touched_files();
assert_eq!(files, vec!["file.txt"]);
```

## License

This project is licensed under the MIT License. See [License](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/common_parsing/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
