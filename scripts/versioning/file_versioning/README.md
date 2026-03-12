# File Versioning Documentation

This directory contains scripts for version control workflows, branch management, and GitHub operations.

## Role in the Project

This directory is responsible for managing version control workflows including branch operations, pull request automation, and repository synchronization.
It interacts mainly with:

- Git repositories (local branch management, commits, pushes)
- GitHub API (via `gh` CLI for PRs, issues, labels)
- CI/CD workflows (automated dev/main synchronization)
- Repository developers (interactive workflow orchestration)

## Directory Structure

```
file_versioning/
├── README.md (this file)
├── TOC.md
├── orchestrators/              # Workflow orchestration
│   ├── execute/                # Interactive entry points (user-facing)
│   │   ├── start_work.sh       # Main workflow: sync, issues, branch
│   │   ├── ci_watch_pr.sh      # Monitor PR CI status
│   │   └── labels_sync.sh      # Sync repository labels
│   └── read/                   # Non-interactive components (API layer)
│       ├── synch_main_dev_ci.sh  # Bot automation for dev sync
│       ├── check_priority_issues.sh  # List priority issues
│       └── create_pr.sh        # Internal PR helper (guarded)
├── git/                        # Pure git operations (10 scripts)
│   ├── create_branch.sh        # Create branches with validation
│   ├── delete_branch.sh        # Delete branches
│   ├── push_branch.sh          # Push branches
│   ├── clean_branches.sh       # Clean obsolete branches
│   └── ...                     # Additional git utilities
└── github/                     # GitHub-only operations
    └── generate_pr_description.sh # Generate merge PR description
```

## Files

- `README.md`: This file.
- `TOC.md`: Documentation index for file versioning scripts.
- `orchestrators/`: Workflow orchestration scripts.
- `git/`: Pure git operation scripts.
- `github/`: GitHub-only operations.

## Architecture: Execute vs Read

Scripts are organized into two clear categories for maximum clarity:

### 📁 `orchestrators/execute/` - Executable Entry Points

Complete workflows that users run directly:

- **`start_work.sh`** ⭐ - Main workflow: sync dev, show priority issues, create branch
- **`ci_watch_pr.sh`** - Monitor PR CI status
- **`labels_sync.sh`** - Sync repository labels from config

**Usage:** Run directly

```bash
./scripts/versioning/file_versioning/orchestrators/execute/start_work.sh
```

### 📁 `orchestrators/read/` - Read-Only Components

Specialized scripts called by execute scripts or bot automation:

- `synch_main_dev_ci.sh` - Synchronize dev with main (bot automation only)
- `create_pr.sh` - Internal PR helper (direct invocation blocked)
- `check_priority_issues.sh` - List high priority/security issues

**Usage:** Called internally by orchestrators (not meant to be run directly)

### 📁 `git/` - Git Utility Scripts

Low-level scripts using only `git` commands:

- `create_branch.sh` - Create feature/fix/doc branches
- And more specialized git operations

## Why This Architecture?

1. **Crystal Clear Structure**: `execute/` contains what you run, `read/` contains what executes it
2. **Forces Best Practices**: Users follow complete workflows, not isolated operations
3. **Prevents Errors**: Can't skip critical steps (sync before branch creation)
4. **Easy to Navigate**: Obvious where to find executable scripts vs internal helpers

## Primary Workflow: start_work.sh

**Recommended entry point for starting new work:**

```bash
./scripts/versioning/file_versioning/orchestrators/execute/start_work.sh
```

This orchestrates:

1. **Fetch** latest from dev and main branches
2. **Check** high priority issues (via `check_priority_issues.sh`)
3. **Create** feature branch from issue (via `git/create_branch.sh`)

Note: Main→dev synchronization is now automated by bot after PR merge

This orchestrator:

1. ✅ Synchronizes `dev` with `main` (ensures you're up-to-date)
2. 🔥 Shows high priority & security issues
3. 🌿 Creates branch (from issue or custom name)

**Interactive**: Guides you through the complete process

## After PR Merge: cleanup_after_pr.sh

After your PR merges, clean up your local branches:

```bash
./scripts/versioning/file_versioning/git/cleanup_after_pr.sh
```

⚠️ **Warning:** This script may force-delete local branches (using `git branch -D`) when safe deletion fails. Before running it, ensure the target branches are fully merged or no longer needed, or use the manual workflow for selective/safer cleanup.

**See [Sync After PR Workflow](git/sync_after_pr.md)** for complete documentation on manual vs automated cleanup.

## Current Components

### Git-only Components (`git/`)

Pure git operations (10 components):

- `create_branch.sh` - Create branches with naming validation
- `delete_branch.sh` - Delete branches
- `push_branch.sh` - Push branches
- `clean_branches.sh` - Clean obsolete branches
- `clean_local_gone.sh` - Remove branches with gone remotes
- `create_work_branch.sh` - Create work branches with conventions
- `finish_branch.sh` - Close work branches
- `add_commit_push.sh` - Add, commit, and push
- `create_after_delete.sh` - Recreate branch from base
- `cleanup_after_pr.sh` - Update branches after PR merge

### GitHub Components (`github/`)

- `generate_pr_description.sh` - Generate merge PR description from PR/issue metadata
- `versioning_automation issue create ...` - Canonical direct issue creation contract entrypoint (Rust CLI)
- `versioning_automation issue <read|update|close|reopen|delete> ...` - Canonical issue lifecycle entrypoint (Rust CLI)

Issue creation modes:

- Direct issue flow uses `.github/ISSUE_TEMPLATE/direct_issue.md` + default issue contract.
- Review follow-up flow uses `.github/ISSUE_TEMPLATE/review_followup.md` + `review` label + review issue contract.
- Managed issue flow is handled by `versioning_automation issue ...` and enforces default `issue` label on create unless explicitly disabled.
- Direct shell entrypoints under `github/issues/*` are compatibility wrappers; user workflows should use the Rust CLI.

### Hybrid Components (orchestrators/read)

- `check_priority_issues.sh` - List high priority/security issues
- `synch_main_dev_ci.sh` - Synchronize main→dev via automated PR (bot-only, called by GitHub Actions)
- `create_pr.sh` - Internal helper used by canonical PR flow

## Branch Naming Conventions

Enforced by `create_branch.sh`:

- `feature/` or `feat/` - New features
- `fix/` or `fixture/` - Bug fixes
- `doc/` or `docs/` - Documentation
- `refactor/` - Code refactoring
- `test/` or `tests/` - Tests
- `chore/` - Maintenance tasks

Example: `feature/user-authentication`, `fix/null-pointer-bug`

## Adding New Scripts

**Decision tree:**

1. **Is it a complete workflow?** → Make it executable orchestrator at root level
2. **Is it a specialized component?** → Make it non-executable component in appropriate directory
3. **Does it use only `git`?** → Place in `git/`
4. **Does it use only `gh`?** → Place in `github/`
5. **Does it use both?** → Place at root level

## Documentation

For detailed workflow documentation, see:

- [Scripts TOC](../../TOC.md)
