# Overview of Scripts

> **⚠️ This page is a high-level overview. For comprehensive details about all scripts, their organization, and how to add new ones, see the README files in the `scripts/` directory.**

## Quick Navigation

- **[Full Scripts Documentation](../../../../../scripts/README.md)** - Complete guide with all details and principles
- **[Orchestrators](../../../../../scripts/versioning/file_versioning/orchestrators/README.md)** - Execute and Read orchestrators architecture
- **[Git Scripts](../../../../../scripts/versioning/file_versioning/git/README.md)** - Pure git operations
- **[GitHub Workflows](../../../../../scripts/versioning/file_versioning/README.md)** - GitHub-specific operations
- **[Common Libraries](../../../../../scripts/common_lib/README.md)** - Reusable utilities
- **[Automation Scripts](../../../../../scripts/automation/README.md)** - Project-wide automation
- **[Git Hooks](../../../../../scripts/automation/git_hooks/README.md)** - Automatic validation hooks

## Script Organization

Scripts are organized by **responsibility domain** and **execution mode**:

### Domains

- **`scripts/automation/`** - Workspace automation (builds, checks, CI/CD, security, releases)
  - **`automation/git_hooks/`** - Git hooks (commit-msg, pre-push)
- **`scripts/versioning/file_versioning/`** - Version control workflows (branches, PRs, syncing)
  - **`orchestrators/execute/`** - Interactive orchestrators (human-run, may prompt)
  - **`orchestrators/read/`** - Non-interactive orchestrators (composable, script-friendly)
  - **`git/`** - Pure git utilities (platform-agnostic)
- **`scripts/common_lib/`** - Reusable utility libraries

### Orchestrators Architecture

**Execute orchestrators** (`orchestrators/execute/`):

- Interactive scripts for humans
- May prompt users with menus and choices
- Human-friendly output
- Examples: `start_work.sh`, `ci_watch_pr.sh`

**Read orchestrators** (`orchestrators/read/`):

- Non-interactive, composable
- No prompts, stable exit codes
- Called by execute orchestrators
- Examples: `synch_main_dev.sh`, `create_pr.sh`

## Key Principles

- **Git Hooks**: 3 files (commit-msg, pre-push, install)
- **Versioning Orchestrators (Execute)**: 3 scripts
- **Versioning Orchestrators (Read)**: 3 scripts
- **Versioning (Git Pure)**: 10- Each script belongs in one clear location

1. **Modular Design** - Utility libraries in `common_lib/` support all scripts
1. **Self-Documenting** - Each directory has a README explaining its purpose

## Adding New Scripts

See **[scripts/README.md](../../../../../scripts/README.md)** for the complete decision tree and guidelines.

## Current Script Count

- **Automation**: 14 scripts
- **Versioning (Git Pure)**: 10 scripts
- **Versioning (Hybrid)**: 4 scripts
- **Common Libraries**: 11 modules across core and git utilities
