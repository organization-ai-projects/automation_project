# Workflows Overview

This document provides a general overview of the GitHub Actions workflows used in this repository. It explains their purpose, structure, and how they interact with the development process.

## Workflows

- **ci_main.yml**: Handles CI tasks for the `main` branch.
- **ci_dev.yml**: Handles CI tasks for the `dev` branch.
- **ci_reusable.yml**: A reusable workflow that centralizes common CI steps.
- **automation_rustfmt.yml**: Applies rustfmt to modified Rust files in PRs.
- **automation_markdown.yml**: Applies markdownlint fixes to modified Markdown files in PRs.
- **automation_sync.yml**: Syncs `main` into `dev` after merge.

## Purpose

The workflows are designed to ensure code quality, automate testing, and streamline the development process. Each workflow is tailored to specific branches or tasks, while the reusable workflow reduces duplication.

## Structure

- `.github/workflows/`: Contains the YAML files for the workflows.
- `.github/workflows/documentation/`: Contains detailed documentation for each workflow.

## Related Documentation

- [ci_main.yml Documentation](../workflows/documentation/ci_main.md)
- [ci_dev.yml Documentation](../workflows/documentation/ci_dev.md)
- [ci_reusable.yml Documentation](../workflows/documentation/ci_reusable.md)
- [automation_rustfmt.yml Documentation](../workflows/documentation/automation_rustfmt.md)
- [automation_markdown.yml Documentation](../workflows/documentation/automation_markdown.md)
- [automation_sync.yml Documentation](../workflows/documentation/automation_sync.md)
