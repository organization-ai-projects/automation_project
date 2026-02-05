# Orchestrators Documentation

This directory contains orchestrator scripts organized by execution mode and interactivity.

## Role in the Project

This directory is responsible for orchestrating complete workflows by coordinating git operations, GitHub API calls, and user interactions.
It interacts mainly with:

- Git utility scripts (in `git/` directory)
- GitHub API (via `gh` CLI)
- Developers (interactive prompts and guidance)
- CI/CD automation (bot-triggered synchronization)

## Directory Structure

```
orchestrators/
├── README.md (this file)
├── TOC.md
├── execute/                    # Interactive orchestrators (UI layer)
│   ├── start_work.sh           # Main workflow for starting work
│   ├── ci_watch_pr.sh          # Monitor PR CI status
│   └── labels_sync.sh          # Sync repository labels
└── read/                       # Non-interactive orchestrators (API layer)
    ├── synch_main_dev_ci.sh    # Automated dev/main sync (bot-only)
    ├── check_priority_issues.sh # List priority/security issues
    └── create_pr.sh            # Create pull requests
```

## Architecture: Execute vs Read

We split orchestrators into two categories based on **how they're used**:

### 📍 `execute/` - Interactive Orchestrators

**For humans to run directly from the command line.**

**Characteristics:**

- ✅ May prompt the user (`read -rp`)
- ✅ Human-friendly output with colors, emojis, formatting
- ✅ Guide users through workflows step-by-step
- ✅ Make decisions on behalf of the user
- ✅ Call `read/` orchestrators and `git/` utilities

**Rule:** These scripts are the "UI layer". They orchestrate workflows, not implement logic.

**Examples:**

- `start_work.sh` - Guides through sync → issues → branch creation
- `ci_watch_pr.sh` - Monitors PR with human-readable status
- `labels_sync.sh` - Interactive label management

### 🔧 `read/` - Composable Orchestrators

**For other scripts to call (non-interactive, script-friendly).**

**Characteristics:**

- ❌ No prompts (`read -rp` forbidden)
- ❌ No user interaction
- ✅ Stable exit codes (0 = success, non-0 = failure)
- ✅ Script-friendly output (can be piped/parsed)
- ✅ Accept parameters via environment variables or arguments
- ✅ Implement actual business logic

**Rule:** These scripts are the "API layer". They do real work with sensible defaults.

**Examples:**

- `synch_main_dev_ci.sh` - Sync dev with main (called by bot/CI only)
- `check_priority_issues.sh` - List priority issues (called by `start_work.sh`)
- `create_pr.sh` - Create PR with defaults (called by other scripts)

## Technical Contract

### Scripts in `read/`

**Must follow:**

```bash
# ✅ Accept config via environment
BRANCH="${BRANCH:-main}"

# ✅ Accept arguments for flexibility
script.sh --option value

# ❌ NEVER do this:
read -rp "Enter something: " VAR    # No prompts!

# ✅ Use defaults instead:
VAR="${VAR:-default_value}"

# ✅ Exit with clear codes
exit 0   # Success
exit 1   # Failure
```

### Scripts in `execute/`

**May do:**

```bash
# ✅ Prompt the user
read -rp "Choose an option: " choice

# ✅ Human-friendly output
echo "🎉 All done!"
echo "❌ Something failed"

# ✅ Call read/ scripts (bot automation handles sync)
# bash "../read/synch_main_dev_ci.sh"  (No longer called by start_work.sh - bot-only)

# ✅ Guide workflows
echo "Step 1 of 3: Preparing dev..."
```

## Execution Flow

```plaintext
User runs:  ./execute/start_work.sh
            ↓
            Prompts & orchestrates
            ↓
            Fetches latest from dev/main
            ↓
            Calls read/check_priority_issues.sh (API call)
            ↓
            Calls git/create_branch.sh (API call)
            ↓
            [Bot automation handles main→dev sync via GitHub Actions]
            ↓
            Reports success to user
```

## When to Add Scripts

**Ask yourself:**

1. **Is this interactive?** (prompts, menus, user decisions)
   → Goes in `execute/`

2. **Is this reusable by other scripts?** (no prompts, stable output)
   → Goes in `read/`

3. **Is this pure git operations?** (no orchestration)
   → Goes in `git/`

4. **Is this a utility function?** (sourced, not executed)
   → Goes in `common_lib/`

## Documentation

- See [execute/README.md](execute/README.md) for orchestrator details
- See [read/README.md](read/README.md) for component details
- See [git/README.md](../git/README.md) for git utilities
