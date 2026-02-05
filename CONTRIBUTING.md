# Contributing

## Overview

Contributions are welcome. Keep changes focused, follow existing structure, and link your work to an issue when possible.

For detailed workflow documentation, see the [scripts TOC](scripts/TOC.md).

---

## Getting Started

1. Fork the repository (external contributors) or clone directly (team members).
2. Create a branch from `dev` following the naming convention below.
3. Make your changes with clear, focused commits.
4. Open a pull request to `dev`.

---

## Branch Naming

Use descriptive branch names with a type prefix:

```text
<type>/<short-description>
```

**Types**:

- `feat/` – New feature
- `fix/` – Bug fix
- `doc/` – Documentation changes
- `refactor/` – Code refactoring
- `test/` – Adding or updating tests
- `chore/` – Maintenance tasks

**Examples**:

- `feat/user-authentication`
- `fix/json-parser-panic`
- `doc/update-api-docs`

---

## Commit Guidelines

- Keep commits small and focused on a single change.
- Use clear, descriptive commit messages.
- Reference issues when applicable: `fix: Resolve panic in parser (#42)`

See [Git scripts TOC](scripts/versioning/file_versioning/git/TOC.md) for details.

---

## Pull Request Guidelines

### Before Opening a PR

1. Rebase your branch on the latest `dev`:

   ```bash
   git fetch origin
   git rebase origin/dev
   ```

2. Run tests locally:

   ```bash
   cargo test --workspace
   ```

3. Check formatting and lints:

   ```bash
   cargo fmt --check
   cargo clippy --workspace
   ```

### PR Requirements

- **Title**: Use the same convention as branch names (`feat:`, `fix:`, etc.)
- **Description**: Explain what and why, link related issues
- **Size**: Keep PRs focused; split large changes into smaller PRs
- **Tests**: Include tests for new functionality

See [Versioning TOC](scripts/versioning/file_versioning/TOC.md) for details.

---

## Code Review Process

1. All PRs require at least one approval before merging.
2. Address reviewer feedback promptly.
3. Re-request review after making changes.
4. Resolve all conversations before merging.

### Review Expectations

- Reviews typically happen within 1-2 business days.
- Be respectful and constructive in feedback.
- Focus on correctness, clarity, and maintainability.

---

## Testing Requirements

- All new features must include tests.
- Bug fixes should include a regression test when possible.
- Run the full test suite before submitting:

  ```bash
  cargo test --workspace
  ```

- Ensure CI passes on your PR.

---

## Code Quality

- Prefer explicit error handling over panics.
- Keep documentation up to date with code changes.
- Avoid breaking public APIs without a clear migration path.
- Follow existing code style and patterns.
- Use `cargo fmt` for formatting and `cargo clippy` for lints.

---

## Documentation Standards

### Overview

To maintain consistency across the project, all `README.md` and `TOC.md` files should follow the standardized patterns below. These patterns use placeholders to make them reusable across different directories.

---

### README.md Pattern

Each `README.md` file should include the following sections:

#### 1. Title

A clear title describing the purpose of the directory.

```markdown
# [Directory Name] Documentation
```

**Example**:
```markdown
# Common JSON Library
```

#### 2. Introduction

A brief description of the directory's purpose.

```markdown
This directory contains [description of the purpose, e.g., libraries, scripts, etc.].
```

**Example**:
```markdown
This directory contains shared utility libraries for JSON processing and manipulation.
```

#### 3. Role in the Project

A high-level explanation of the directory's responsibility and its interactions with other modules.

```markdown
## Role in the Project

This directory is responsible for [high-level responsibility].
It interacts mainly with:

- [module / directory A]
- [module / directory B]
```

**Example**:
```markdown
## Role in the Project

This directory is responsible for providing JSON parsing, serialization, and manipulation utilities.
It interacts mainly with:

- `projects/products/core/engine` - Uses JSON for configuration
- `projects/libraries/ai` - Uses JSON for data serialization
```

#### 4. Directory Structure

A tree representation of the directory's contents.

```markdown
## Directory Structure
```

\`\`\`plaintext
[root_directory]/
├── [file_1]      # [Description of file_1]
├── [file_2]      # [Description of file_2]
└── [sub_directory]/
    ├── [doc_1].md        # Documentation for [file_1]
    └── [doc_2].md        # Documentation for [file_2]
\`\`\`

**Example**:
\`\`\`plaintext
common_json/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── parser.rs           # JSON parsing logic
│   └── serializer.rs       # JSON serialization logic
├── documentation/
│   ├── TOC.md             # Documentation index
│   └── api.md             # API reference
└── README.md              # This file
\`\`\`

#### 5. Files Description

A list of key files with placeholders for their purpose.

```markdown
## Files

- **`[file_1]`**: [Description of file_1].
- **`[file_2]`**: [Description of file_2].
```

**Example**:
```markdown
## Files

- **`src/lib.rs`**: Main entry point for the library, exports public APIs.
- **`src/parser.rs`**: Implements JSON parsing from strings and bytes.
- **`src/serializer.rs`**: Implements JSON serialization to strings.
```

---

### TOC.md Pattern

Each `TOC.md` file should include the following sections:

#### 1. Title

A clear title indicating that the file is a table of contents.

```markdown
# Table of Contents
```

#### 2. Introduction

A brief explanation of the file's purpose.

```markdown
This document provides an overview of all documentation files in this directory.
```

#### 3. Files List

A categorized list of `.md` files in the directory.

```markdown
## Concepts

- [architecture.md](architecture.md): Global design of this module.

## Guides

- [usage.md](usage.md): How to use this module.

## Reference

- [api.md](api.md): API details.
```

**Example**:
```markdown
## Core Documentation

- [Back to README](../README.md)

## API Reference

- [parser.md](parser.md): JSON parsing API documentation.
- [serializer.md](serializer.md): JSON serialization API documentation.

## Internal Design

- [architecture.md](architecture.md): Internal architecture and design decisions.
- [performance.md](performance.md): Performance considerations and benchmarks.
```

---

### Benefits

Following these standardized patterns provides:

- **Consistency**: All documentation follows the same structure, making it easier to navigate.
- **Clarity**: Clear sections help contributors understand the purpose and organization.
- **Reusability**: Placeholders make the patterns adaptable to any directory.
- **Maintainability**: Standardized structure simplifies updates and maintenance.
- **Automation**: Consistent patterns enable future automation of documentation generation.

---

### Implementation Notes

1. Replace specific content with appropriate placeholders when creating new documentation.
2. Use the patterns as guidelines; adapt as needed for specific contexts.
3. Keep descriptions clear and concise.
4. Link to related documentation and central resources.
5. Update documentation when making structural changes to directories.

---

## Questions?

If you have questions about contributing, open an issue or reach out to the maintainers.
