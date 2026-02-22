# Table of Contents

Language: **English** | [Francais](i18n/fr/TOC.md)

This document provides an overview of all documentation files in this directory.

## Documentation

- [README.md](README.md): Main documentation for common libraries

## Subdirectories

- [automation/README.md](automation/README.md): Shared automation helpers for hooks/CI/scripts
- [core/README.md](core/README.md): Core utilities for all scripts
- [testing/README.md](testing/README.md): Shared shell test helpers
- [versioning/README.md](versioning/README.md): Version control utilities

## Automation Libraries

- [automation/file_types.sh](automation/file_types.sh): File classifiers (docs/tests/scripts/workflows/shell)
- [automation/scope_resolver.sh](automation/scope_resolver.sh): Scope and crate resolution helpers
- [automation/workspace_rust.sh](automation/workspace_rust.sh): Workspace-members based Rust scope resolver
- [automation/non_workspace_rust.sh](automation/non_workspace_rust.sh): Fallback nearest-Cargo.toml scope resolver
- [automation/change_policy.sh](automation/change_policy.sh): Shared changed-file policy predicates
- [automation/rust_checks.sh](automation/rust_checks.sh): Shared Rust check/fmt/clippy/test runners

## Core Libraries

- [core/command.sh](core/command.sh): Command execution and validation utilities
- [core/file_operations.sh](core/file_operations.sh): File and directory operation utilities
- [core/logging.sh](core/logging.sh): Core logging functions with consistent formatting
- [core/network_utils.sh](core/network_utils.sh): Network-related utilities
- [core/string_utils.sh](core/string_utils.sh): String manipulation utilities

## Versioning Libraries

- [versioning/file_versioning/TOC.md](versioning/file_versioning/TOC.md): File versioning utilities

## Testing Libraries

- [testing/shell_test_helpers.sh](testing/shell_test_helpers.sh): Shared shell test helper functions
