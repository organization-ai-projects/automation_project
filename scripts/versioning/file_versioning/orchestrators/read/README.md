# Read-Only Orchestrator Components

These are **internal scripts** called by executable orchestrators. They are not meant to be run directly, but you can if you understand what they do.

## Scripts

### `synch_main_dev.sh`

**Synchronizes dev branch with main using a clean merge PR.**

Called by: `start_work.sh` (Step 1)

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

**Create a pull request from current branch.**

Can be called standalone or by other scripts.

**Usage (if run directly):**

```bash
bash create_pr.sh
```

**What it does:**

- Creates PR from current branch to configured base branch
- Allows title and description customization
- Handles PR creation via GitHub CLI

## When to Run These Directly?

Generally **don't** - use the orchestrators in `execute/` instead. They ensure proper sequencing and all necessary checks.

**Exception:** Debugging or special cases where you need to run one step manually.

## Architecture Note

These scripts:

- Are non-executable (permissions: `-rw-r--r--`)
- Live in `orchestrators/read/` to indicate they're read-only components
- Should only be called via `bash script.sh` (not directly)
- Handle their own validation but expect orchestrators to validate dependencies
