# Executable Orchestrators Documentation

These scripts are the **main entry points** you run directly from the command line. Each orchestrates a complete workflow.

## Role in the Project

This directory is responsible for providing interactive, user-facing workflows that guide developers through common version control tasks.
It interacts mainly with:

- Developers (via interactive prompts and guidance)
- Read orchestrators (in `../read/` directory)
- Git utilities (in `../../git/` directory)
- GitHub API (via `gh` CLI)

## Directory Structure

```
execute/
├── README.md (this file)
├── TOC.md
├── start_work.sh              # Primary workflow for starting work
├── ci_watch_pr.sh             # Monitor PR CI status
└── labels_sync.sh             # Sync repository labels
```

## Scripts

### `start_work.sh`

**The primary workflow for starting new development work.**

Orchestrates three steps:

1. **Fetch** latest changes from dev and main branches (Main→dev sync is automated by bot after PR merge)
2. **Check** high priority issues (uses `../read/check_priority_issues.sh`)
3. **Create** feature branch from issue (uses `../read/git/create_branch.sh`)

```bash
./start_work.sh
```

**Features:**

- Validates all dependencies at startup (fail-fast)
- Enforces branch naming conventions
- Allows issue-based branch creation with auto-naming
- Supports custom branch names with validation after confirmation
- Skips `main` and `dev` branches during cleanup
- Safe branch switching on exit (handles detached HEAD)

### `ci_watch_pr.sh`

**Monitor CI status of a pull request.**

Polls GitHub API for PR status checks until all pass or one fails.

```bash
./ci_watch_pr.sh [pr-number]
```

If no PR number provided, automatically finds PR for current branch.

**Environment Variables:**

- `POLL_INTERVAL` - Check interval in seconds (default: 10)
- `MAX_WAIT` - Maximum wait time in seconds (default: 3600 / 1 hour)

### `labels_sync.sh`

**Synchronize repository labels from configuration.**

Manages GitHub labels based on a config file.

```bash
./labels_sync.sh
```

## Common Usage

```bash
# Start a new feature
./start_work.sh

# Watch PR #123
./ci_watch_pr.sh 123

# Watch current branch's PR automatically
./ci_watch_pr.sh

# Sync labels
./labels_sync.sh
```

## Requirements

All orchestrators require:

- `git` - Version control
- `gh` - GitHub CLI
- `jq` - JSON processor

Validation happens at startup in `start_work.sh`.
