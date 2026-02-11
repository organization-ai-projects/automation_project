# Read-Only Orchestrator Components Documentation

This directory contains **internal scripts** called by executable orchestrators. They are not meant to be run directly, but you can if you understand what they do.

## Role in the Project

This directory is responsible for providing non-interactive, composable components that implement business logic for version control workflows.
It interacts mainly with:

- Execute orchestrators (called by scripts in `../execute/`)
- Git utilities (in `../../git/` directory)
- GitHub API (via `gh` CLI for PRs, issues)
- CI/CD workflows (bot automation for synchronization)

## Directory Structure

```
read/
├── README.md (this file)
├── TOC.md
├── check_priority_issues.sh   # List priority/security issues
├── create_pr.sh               # Create pull requests
└── synch_main_dev_ci.sh       # Automated dev/main sync (bot-only)
```

## Files

- `README.md`: This file.
- `TOC.md`: Documentation index for read orchestrators.
- `check_priority_issues.sh`: Lists priority/security issues.
- `create_pr.sh`: Creates pull requests.
- `synch_main_dev_ci.sh`: Automates dev/main sync (bot-only).

## Scripts

### `synch_main_dev_ci.sh`

**Synchronizes dev branch with main via automated PR (CI-only, bot automation).**

Called by: GitHub Actions workflow `ci_automation_sync.yml` (after PR merge into main)

⚠️ **Note:** This script is for CI/bot automation only. Do NOT run manually.

**What it does:**

1. Creates a temporary sync branch from `main`
2. Creates a PR from sync branch into `dev`
3. Enables auto-merge with CI checks
4. Waits for merge completion
5. Cleans up temporary branch

**Environment Variables:**

- `MAIN` - Main branch name (default: main)
- `DEV` - Dev branch name (default: dev)
- `REMOTE` - Remote name (default: origin)
- `MAX_RETRIES` - Retries if main advances during sync (default: 1)
- `STRICT_MAIN_SHA` - Enforce main SHA doesn't change (default: true)
- `MAX_WAIT_SECONDS` - Timeout for merge (default: 600)

**Safety Features:**

- Validates clean working tree
- Detects main branch advancement and retries
- Auto-merge with status checks
- Handles detached HEAD safely

### `check_priority_issues.sh`

**List GitHub issues with high priority or security labels.**

Called by: `start_work.sh` (Step 2)

**What it does:**

1. Fetches issues with "high priority" label
2. Fetches issues with "security" label
3. Displays them with clickable links

**Output:**
Shows issue number, title, and GitHub URL for each priority issue.

### `create_pr.sh`

**Create a pull request from current branch with automated test validation.**

Can be called standalone or by other scripts.

**Usage (if run directly):**

```bash
bash create_pr.sh [--base <branch>] [--title <title>] [--body <body>] [--draft] [--skip-tests]
```

**What it does:**

1. Validates current branch is not protected
2. **Runs workspace tests** (`cargo test --workspace`) by default
3. Creates PR from current branch to configured base branch (default: `dev`)
4. Auto-generates title from branch name if not provided
5. Auto-generates body with commit list and checklist
6. Adds labels based on branch type prefix
7. Handles PR creation via GitHub CLI

**Options:**

- `--base <branch>`: Target base branch (default: `dev`)
- `--title <title>`: Custom PR title (auto-generated if not provided)
- `--body <body>`: Custom PR description (auto-generated if not provided)
- `--draft`: Create as draft PR
- `--skip-tests`: Skip test execution (not recommended, shows warning)

**Label mapping (auto-apply):**

- `feat/*` -> `feature`
- `docs/*` -> `documentation`
- `test/*` and `tests/*` -> `testing`
- other supported prefixes map to themselves (`fix`, `chore`, `refactor`)

If label application fails, the script now warns and continues (does not abort PR creation).

**Test Enforcement:**

By default, this script runs `cargo test --workspace` before creating the PR. If tests fail, the PR creation is aborted. Use `--skip-tests` to bypass this check, but a warning will be displayed reminding you to ensure proper testing before merging.

## When to Run These Directly?

Generally **don't** - use the orchestrators in `execute/` instead. They ensure proper sequencing and all necessary checks.

**Exception:** Debugging or special cases where you need to run one step manually.

## Architecture Note

These scripts:

- Are non-executable (permissions: `-rw-r--r--`)
- Live in `orchestrators/read/` to indicate they're read-only components
- Should only be called via `bash script.sh` (not directly)
- Handle their own validation but expect orchestrators to validate dependencies
