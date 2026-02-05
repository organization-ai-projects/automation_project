# Workflows Documentation Documentation

This directory contains all documentation related to GitHub Actions workflows used in this repository. The goal is to provide clarity on their purpose, triggers, and steps.

## Role in the Project

This directory is responsible for maintaining comprehensive documentation for all GitHub Actions workflows used in the repository. It ensures that workflow configurations are well-documented, maintainable, and understandable for all contributors.

It interacts mainly with:

- `.github/workflows/`: The actual workflow configuration files
- `.github/`: Parent directory for GitHub-related configuration
- Root repository documentation for overall project context

## Directory Structure

```plaintext
.github/workflows/
├── ci_main.yml           # Handles CI tasks for the main branch
├── ci_dev.yml            # Handles CI tasks for the dev branch
├── ci_reusable.yml       # Reusable workflow for common CI steps
├── automation_rustfmt.yml # Automates code formatting checks
├── automation_sync.yml   # Syncs main into dev after merge
└── documentation/
    ├── TOC.md           # Table of contents for workflow documentation
    ├── ci_main.md       # Documentation for the ci_main.yml workflow
    ├── ci_dev.md        # Documentation for the ci_dev.yml workflow
    ├── ci_reusable.md   # Documentation for the ci_reusable.yml workflow
    ├── automation_rustfmt.md # Documentation for the automation_rustfmt.yml workflow
    ├── automation_sync.md # Documentation for the automation_sync.yml workflow
    ├── bot/
    │   ├── README.md    # Bot workflows docs index
    │   └── TOC.md       # Bot workflows documentation TOC
    └── README.md        # This file
```

## Organization Principle

Workflows are organized into two categories:

- **CI Workflows**:
  - `ci_main.yml`: Handles CI tasks for the `main` branch.
  - `ci_dev.yml`: Handles CI tasks for the `dev` branch.
  - `ci_reusable.yml`: A reusable workflow for common CI steps.

- **Automation Workflows**:
  - `automation_rustfmt.yml`: Automates code formatting checks.
  - `automation_sync.yml`: Syncs main into dev after merge.

## Adding New Workflows

1. **Understand the workflow's purpose** - What task does it automate?
2. **Document it** - Create a new `.md` file in this directory.
3. **Update the TOC** - Add the new workflow to the `TOC.md` in `.github/`.

## Documentation

For a complete overview of workflows:

- See the [GitHub TOC](../../TOC.md)
- Refer to individual workflow documentation files in this directory.
