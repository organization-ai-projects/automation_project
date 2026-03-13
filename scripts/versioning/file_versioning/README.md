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
├── git/                        # Documentation/contracts for git workflows
└── github/                     # GitHub-only operations
    └── ...                     # Invoked via `versioning_automation` Rust CLI
```

## Files

- `README.md`: This file.
- `TOC.md`: Documentation index for file versioning scripts.
- `git/`: Pure git operation scripts.
- `github/`: GitHub-only operations.

## Runtime Entry Points

Automation runtime entrypoints are now the Rust CLI:

- `versioning_automation automation ...`
- `versioning_automation git ...`
- `versioning_automation pr ...`
- `versioning_automation issue ...`

Legacy orchestrator shell entrypoints under `orchestrators/**` have been removed.

## After PR Merge: cleanup-after-pr

After your PR merges, clean up your local branches:

```bash
versioning_automation git cleanup-after-pr
```

⚠️ **Warning:** This script may force-delete local branches (using `git branch -D`) when safe deletion fails. Before running it, ensure the target branches are fully merged or no longer needed, or use the manual workflow for selective/safer cleanup.

**See [Sync After PR Workflow](git/sync_after_pr.md)** for complete documentation on manual vs automated cleanup.

## Current Components

### Git Components (Rust CLI)

Git operations are now exposed via `versioning_automation git ...`:

- `create-branch` - Create branches with naming validation
- `delete-branch` - Delete branches (local/remote)
- `push-branch` - Push current branch
- `clean-branches` - Clean obsolete branches
- `clean-local-gone` - Remove branches with gone remotes
- `create-work-branch` - Create work branches with conventions
- `finish-branch` - Close work branches
- `add-commit-push` - Add, commit, and push
- `create-after-delete` - Recreate current branch from base
- `cleanup-after-pr` - Update/recreate outdated branches after PR merge

### GitHub Components (`github/`)

- `versioning_automation pr generate-description ...` - Generate merge PR description from PR/issue metadata
- `versioning_automation issue create ...` - Canonical direct issue creation contract entrypoint (Rust CLI)
- `versioning_automation issue <read|update|close|reopen|delete> ...` - Canonical issue lifecycle entrypoint (Rust CLI)

Issue creation modes:

- Direct issue flow uses `.github/ISSUE_TEMPLATE/direct_issue.md` + default issue contract.
- Review follow-up flow uses `.github/ISSUE_TEMPLATE/review_followup.md` + `review` label + review issue contract.
- Managed issue flow is handled by `versioning_automation issue ...` and enforces default `issue` label on create unless explicitly disabled.
- User workflows should use Rust CLI entrypoints (`versioning_automation ...`) directly.

### Automation Components (Rust CLI)

- `versioning_automation automation check-priority-issues ...`
- `versioning_automation automation labels-sync ...`
- `versioning_automation automation ci-watch-pr ...`
- `versioning_automation automation sync-main-dev-ci ...`

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
