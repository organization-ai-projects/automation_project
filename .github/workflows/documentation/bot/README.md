# Bot Documentation

This directory contains index docs for bot-related workflows. The detailed workflow docs live in the parent directory.

## Role in the Project

This directory is responsible for organizing and indexing bot-related workflow documentation. It serves as a focused entry point for understanding automation bot workflows.

It interacts mainly with:

- `.github/workflows/documentation/`: Parent directory containing detailed workflow documentation
- `.github/workflows/`: The actual bot workflow configuration files
- Root GitHub configuration for CI/CD automation

## Directory Structure

```plaintext
.github/workflows/
├── automation_rustfmt.yml      # Automates code formatting checks
├── automation_sync.yml         # Syncs main into dev after merge
├── issue_done_in_dev_status.yml # Maintains done-in-dev issue status labels
└── documentation/bot/
    ├── README.md               # This file
    └── TOC.md                  # Bot workflows documentation index
```

## Files

- `README.md`: Bot workflows docs index.
- `TOC.md`: Bot workflows documentation TOC.

## Workflows

- **`automation_rustfmt.yml`**: Automates code formatting checks.
- **`automation_sync.yml`**: Syncs main into dev after merge.
- **`issue_done_in_dev_status.yml`**: Adds/removes `done-in-dev` label through issue lifecycle events.

For detailed documentation, see:

- [automation_rustfmt.yml Documentation](../automation_rustfmt.md)
- [automation_sync.yml Documentation](../automation_sync.md)
- [issue_done_in_dev_status.yml Documentation](../issue_done_in_dev_status.md)
