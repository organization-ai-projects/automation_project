# Versioning Library

Custom versioning system for managing project releases independently of Git.

## Overview

The versioning library provides a standalone version tracking system designed specifically for `automation_project`. It implements a custom three-tier release identifier scheme, comprehensive changelog management, and document generation capabilities without relying on Git operations.

## Role in Project

This library enables projects to:
- Track release versions using a custom identifier format
- Maintain detailed revision histories with categorized modifications
- Generate formatted documentation of changes
- Persist version state to files for portability
- Manage releases without Git dependency

The versioning system uses a three-tier numbering approach (e.g., 2.5.8) where:
- **Tier 1**: Breaking changes that affect compatibility
- **Tier 2**: New features and capabilities (backward compatible)
- **Tier 3**: Corrections, refinements, and minor updates

## Directory Structure

```
versioning/
├── Cargo.toml              # Package configuration
├── README.md               # This file
├── documentation/          # Additional documentation
└── src/
    ├── lib.rs             # Library root and public exports
    ├── document_builder.rs # Documentation generation
    ├── modification_category.rs # Modification category enum
    ├── modification_entry.rs # Individual changelog entries
    ├── output_format.rs   # Output format enum
    ├── release_id.rs      # Release identifier implementation
    ├── release_id_error.rs # Release identifier parsing errors
    ├── release_tracker.rs # Version state management
    ├── revision_entry.rs  # Release revision entries
    ├── revision_log.rs    # Changelog and modification tracking
    └── tests/             # Test modules
        ├── mod.rs
        ├── release_id_tests.rs
        ├── revision_log_tests.rs
        ├── release_tracker_tests.rs
        └── document_builder_tests.rs
```

## Files Description

### Core Modules

- **lib.rs**: Public API surface and module exports for the library
- **document_builder.rs**: Contains `DocumentBuilder` for generating formatted changelog documents in Markdown or plain text
- **modification_category.rs**: Defines `ModificationCategory` enum for changelog classification
- **modification_entry.rs**: Defines `ModificationEntry` for individual change records
- **output_format.rs**: Defines `OutputFormat` enum for document generation
- **release_id.rs**: Defines `ReleaseId` structure for three-tier version identifiers with parsing, formatting, and advancement logic
- **release_id_error.rs**: Defines `ReleaseIdError` for version parsing failures
- **release_tracker.rs**: Provides `ReleaseTracker` for managing version state and release registration with file persistence
- **revision_entry.rs**: Defines `RevisionEntry` for per-release change groups
- **revision_log.rs**: Implements `RevisionLog` for tracking the release history

### Features

#### Release Identifiers

```rust
use versioning::ReleaseId;

// Create release identifiers
let release = ReleaseId::build(2, 5, 8);
let initial = ReleaseId::initial(); // 1.0.0

// Parse from string
let parsed: ReleaseId = "3.2.1".parse().unwrap();

// Advance versions
let next_major = release.advance_major();    // 3.0.0
let next_feature = release.advance_feature(); // 2.6.0
let next_fix = release.advance_correction();  // 2.5.9

// Check compatibility
let breaks = next_major.breaks_compatibility_with(&release); // true
```

#### Modification Tracking

```rust
use versioning::{ModificationEntry, ModificationCategory};

let entry = ModificationEntry::create(
    "Add user authentication system".to_string(),
    ModificationCategory::NewCapability,
);
```

Available categories:
- `BreakingModification` - Changes that break compatibility
- `NewCapability` - New features added
- `Enhancement` - Improvements to existing features
- `CorrectionApplied` - Bug fixes and corrections
- `SecurityUpdate` - Security-related changes
- `DeprecationNotice` - Deprecated features

#### Release Tracking

```rust
use versioning::{ReleaseTracker, ModificationEntry, ModificationCategory};

// Initialize tracker
let mut tracker = ReleaseTracker::initialize("MyProject".to_string());

// Register a new feature release
let modifications = vec![
    ModificationEntry::create(
        "Add dashboard view".to_string(),
        ModificationCategory::NewCapability,
    ),
];
tracker.register_feature_release(modifications, vec!["Alice".to_string()]);

// Access current version
println!("Current: {}", tracker.active_release());

// Persist to file
tracker.persist_to_file("version.json").unwrap();

// Load from file
let loaded = ReleaseTracker::load_from_file("version.json").unwrap();
```

#### Document Generation

```rust
use versioning::{DocumentBuilder, OutputFormat};

let builder = DocumentBuilder::with_format(OutputFormat::Markdown);
let changelog = builder.generate_document(tracker.log());

// Write to file
std::fs::write("CHANGELOG.md", changelog).unwrap();
```

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
versioning = { workspace = true }
```

Or with explicit path:

```toml
[dependencies]
versioning = { path = "../versioning" }
```

## Usage Examples

### Complete Release Workflow

```rust
use versioning::{
    ReleaseTracker, ModificationEntry, ModificationCategory,
    DocumentBuilder, OutputFormat,
};

// Setup
let mut tracker = ReleaseTracker::initialize("MyApplication".to_string());

// Add a feature release
tracker.register_feature_release(
    vec![
        ModificationEntry::create(
            "Implement REST API endpoints".to_string(),
            ModificationCategory::NewCapability,
        ),
    ],
    vec!["Developer A".to_string()],
);

// Add a correction release
tracker.register_correction_release(
    vec![
        ModificationEntry::create(
            "Fix authentication token expiry".to_string(),
            ModificationCategory::CorrectionApplied,
        ),
    ],
    vec!["Developer B".to_string()],
);

// Generate changelog
let builder = DocumentBuilder::with_format(OutputFormat::Markdown);
let changelog = builder.generate_document(tracker.log());
println!("{}", changelog);

// Save state
tracker.persist_to_file("releases.json").unwrap();
```

### Version Comparison

```rust
use versioning::ReleaseId;

let current = ReleaseId::build(2, 5, 3);
let previous = ReleaseId::build(2, 4, 8);

if current > previous {
    println!("Newer version");
}

if current.breaks_compatibility_with(&previous) {
    println!("Breaking change!");
} else {
    println!("Backward compatible");
}
```

## Testing

Run tests with:

```bash
cargo test -p versioning
```

Run with verbose output:

```bash
cargo test -p versioning -- --nocapture
```

## Thread Safety

All types in this library are thread-safe when used appropriately:
- `ReleaseId` is `Copy` and can be freely shared
- `ReleaseTracker` and `RevisionLog` should be protected with `Mutex` or similar for concurrent modifications

## License

This project is licensed under the MIT License. See [LICENSE](https://github.com/organization-ai-projects/automation_project/blob/main/LICENSE).

## Documentation

- [Documentation Index](https://github.com/organization-ai-projects/automation_project/blob/main/projects/libraries/versioning/documentation/TOC.md)

## Contributing

See the workspace README and contribution guide:

- [Workspace README](https://github.com/organization-ai-projects/automation_project/blob/main/README.md)
- [Contributing](https://github.com/organization-ai-projects/automation_project/blob/main/CONTRIBUTING.md)
