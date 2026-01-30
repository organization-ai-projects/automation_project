# Automation Scripts

## Overview

To make the team's work easier and more structured, automation scripts were created to handle repetitive tasks and critical project workflows.

These scripts cover several areas:

- **General automation**: Build UI, documentation sync, bundle validation
- **Version management**: Git operations (branches, commits, sync), GitHub workflows (pull requests, issues)
- **Common libraries**: Reusable utilities (logging, file operations, commands, network)

## Scripts Documentation

**The complete, practical documentation for script usage lives in the `scripts/` directory at the project root.**

See:

- **[Main Scripts README](../../scripts/README.md)**: Overview of organization and philosophy
- **[Automation Scripts](../../scripts/automation/README.md)**: Automation script docs (build UI, sync docs, etc.)
- **[Versioning Scripts](../../scripts/versioning/README.md)**: Git and GitHub script docs
- **[Common Libraries](../../scripts/common_lib/README.md)**: Reusable utilities documentation

## Principles and Standards

The scripts follow these principles:

1. **Robustness**: All scripts use `set -euo pipefail` for strict error handling
2. **Modularity**: Common functions live in `scripts/common_lib/`
3. **Documentation**: Each script is documented with usage, parameters, and examples
4. **Maintainability**: Clear code with consistent logging and explicit error messages
5. **Security**: Input validation, credential handling, atomic operations

## Workflows and Conventions

This directory (`technical_documentation/`) contains conceptual guides for team workflows and conventions:

- **[Git Workflows](versioning/file_versioning/git/)**: Conventions and processes for versioning
- **[GitHub Workflows](versioning/file_versioning/github/)**: Conventions for pull requests, issues, etc.
- **[Documentation Automation](automation/)**: Generation and synchronization processes

These documents describe **why** and **how** to work as a team, while `scripts/` describes **how to use** the practical tools.

## Separation of Responsibilities

| Location                   | Content                                                   | Purpose                               |
| -------------------------- | --------------------------------------------------------- | ------------------------------------- |
| `technical_documentation/` | Workflows, conventions, philosophy, conceptual guides     | Understand team processes             |
| `scripts/`                 | Practical script docs, usage, examples                    | Use the automation tools              |

## Important Note

**Single Source of Truth**: For any practical usage info (parameters, options, examples), always consult the documentation in `scripts/`. That is the only maintained source for practical script usage.

Documents in `technical_documentation/` may reference scripts but must never duplicate their usage documentation.
