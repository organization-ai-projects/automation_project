# File Versioning Scripts

This directory contains scripts for version control workflows, branch management, and GitHub operations.

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
- `create_pr.sh` - Create pull request
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

Currently empty. Reserved for scripts using only `gh` commands.

### Hybrid Components (root level)

- `check_priority_issues.sh` - List high priority/security issues
- `synch_main_dev_ci.sh` - Synchronize main→dev via automated PR (bot-only, called by GitHub Actions)
- `create_pr.sh` - Create pull requests with auto-generated content

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
