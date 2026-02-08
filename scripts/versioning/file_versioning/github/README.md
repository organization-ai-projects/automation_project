# GitHub Scripts Documentation

This directory contains scripts that use **only** the `gh` (GitHub CLI) tool.

## Role in the Project

This directory is reserved for pure GitHub platform operations that don't require git commands.
It interacts mainly with:

- GitHub API (via `gh` CLI)
- Repository settings and configurations
- GitHub Actions workflows
- Organization and team management

## Directory Structure

```
github/
├── README.md (this file)
├── TOC.md
└── generate_pr_description.sh
```

## Files

- `README.md`: This file.
- `TOC.md`: Documentation index for GitHub-only scripts.
- `generate_pr_description.sh`: Generate structured merge PR descriptions from GitHub metadata.

## Scope

Scripts in this directory should:

- Use only `gh` commands (no `git` commands)
- Perform pure GitHub platform operations
- Work exclusively with GitHub's API/features

## Scripts

### `generate_pr_description.sh`

Generates a ready-to-paste PR description (e.g., `dev -> main`) by analyzing child PRs and resolved issues through `gh`.

Usage:

```bash
bash generate_pr_description.sh [--keep-artifacts] [MAIN_PR_NUMBER] [OUTPUT_FILE]
```
