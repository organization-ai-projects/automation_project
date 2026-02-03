# Scripts Organization

This directory contains all automation scripts for the project, organized by **responsibility domain**.

## Directory Structure

```plaintext
scripts/
├── automation/       # Workspace automation (builds, checks, CI, security, releases)
│   └── git_hooks/    # Git hooks for commit validation and pre-push checks
├── common_lib/       # Reusable utility libraries sourced by other scripts
├── versioning/       # Version control workflows (branches, PRs, releases)
└── README.md         # This file
```

## Organization Principle

Scripts are organized by **what they do** (responsibility), not by the tools they use:

- **`automation/`** - Scripts that automate project-wide tasks (builds, tests, audits, releases, quality checks)
  - **`automation/git_hooks/`** - Git hooks for automatic validation (commit messages, pre-push checks)
- **`common_lib/`** - Reusable function libraries sourced by other scripts
- **`versioning/`** - Scripts for version control and git workflows
  - **`versioning/file_versioning/orchestrators/execute/`** - Interactive scripts run by humans (may prompt, guide workflows)
  - **`versioning/file_versioning/orchestrators/read/`** - Non-interactive scripts composed by other scripts (no prompts, script-friendly)

## Adding New Scripts

1. **Understand what the script does** - What task does it automate?
2. **Find the right domain** - Does it belong in automation, versioning, or as a utility library in common_lib?
3. **Check existing organization** - Look at similar scripts in that domain
4. **Document it** - Add description to the domain's README and main scripts_overview.md

## Documentation

For complete details about all scripts:

- See the README in each domain directory
- See the scripts TOC: `scripts/TOC.md`

## Quick Reference

| Goal                             | Directory                                   | Example                                    |
| -------------------------------- | ------------------------------------------- | ------------------------------------------ |
| Validate commits and quality     | `automation/git_hooks/`                     | `commit-msg`, `pre-push`                   |
| Automate builds, tests, checks   | `automation/`                               | `build_ui_bundles.sh`, `pre_add_review.sh` |
| Manage branches, PRs, sync repos | `versioning/file_versioning/orchestrators/` | `execute/start_work.sh`, bot automation    |
| Reusable utility libraries       | `common_lib/`                               | `logging.sh`, `command.sh`                 |
