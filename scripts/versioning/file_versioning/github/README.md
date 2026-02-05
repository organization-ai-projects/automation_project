# GitHub Scripts Documentation

This directory is reserved for scripts that use **only** the `gh` (GitHub CLI) tool.

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
└── TOC.md
```

## Scope

Scripts in this directory should:

- Use only `gh` commands (no `git` commands)
- Perform pure GitHub platform operations
- Work exclusively with GitHub's API/features

## Examples (Future)

- Repository settings management
- GitHub Actions workflows management
- Organization/team management
- Security/policy configuration

## Current Status

Currently **empty**. All existing GitHub-related scripts also use `git` commands, so they remain as hybrid scripts at the parent level (`file_versioning/`).
