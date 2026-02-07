# Git Scripts Documentation

This directory contains scripts that use **only** the `git` command-line tool.

## Workflows

For complete workflow documentation, see:

- **[Sync After PR Merge](sync_after_pr.md)** - Learn how to update your local branches after a PR merges, with both manual steps and automated cleanup via `cleanup_after_pr.sh`

## Role in the Project

This directory is responsible for pure git operations that are platform-agnostic and work with any git hosting service.
It interacts mainly with:

- Local git repositories (branch management, commits, working tree)
- Remote git repositories (push, fetch operations)
- Parent orchestrators (called by execute/read scripts)
- Commit message validation (enforces conventional commit format)

## Directory Structure

```
git/
├── README.md (this file)
├── TOC.md
├── sync_after_pr.md           # Workflow: sync branches after PR merge
├── create_branch.sh           # Create branches with validation
├── delete_branch.sh           # Delete branches
├── push_branch.sh             # Push branches to remote
├── clean_branches.sh          # Clean obsolete branches
├── clean_local_gone.sh        # Remove branches with gone remotes
├── create_work_branch.sh      # Create work branches with conventions
├── finish_branch.sh           # Close work branches
├── add_commit_push.sh         # Add, commit, and push with validation
├── create_after_delete.sh     # Recreate branch from base
└── cleanup_after_pr.sh        # Update branches after PR merge
```

## Files

- `README.md`: This file.
- `TOC.md`: Documentation index for git scripts.
- `sync_after_pr.md`: Workflow documentation for syncing branches after PR merge.
- `create_branch.sh`: Create branches with validation.
- `delete_branch.sh`: Delete branches.
- `push_branch.sh`: Push branches to remote.
- `clean_branches.sh`: Clean obsolete branches.
- `clean_local_gone.sh`: Remove branches with gone remotes.
- `create_work_branch.sh`: Create work branches with conventions.
- `finish_branch.sh`: Close work branches.
- `add_commit_push.sh`: Add, commit, and push with message validation.
- `create_after_delete.sh`: Recreate branch from base.
- `cleanup_after_pr.sh`: Update branches after PR merge.

## Commit Message Validation

The `add_commit_push.sh` script enforces conventional commit message format:

**Format**: `<type>(<scope>): <message>` or `<type>: <message>`

**Allowed types**: `feature`, `feat`, `fix`, `fixture`, `doc`, `docs`, `refactor`, `test`, `tests`, `chore`

**Examples**:
- `feat(auth): add user authentication`
- `fix: resolve null pointer exception`
- `docs(readme): update installation instructions`

**Bypass** (not recommended):
- Use `--no-verify` flag with the script: `./add_commit_push.sh "message" --no-verify`
- Use `SKIP_COMMIT_VALIDATION=1` with git directly: `SKIP_COMMIT_VALIDATION=1 git commit -m "message"`

See [CONTRIBUTING.md](../../../../CONTRIBUTING.md) and [commit-msg hook](../../../automation/git_hooks/commit-msg) for full details.

## Scope

Scripts in this directory should:

- Use only `git` commands (no `gh`, `gitlab-cli`, or other version control platform CLIs)
- Perform pure git operations (branches, commits, working tree, etc.)
- Be platform-agnostic (work with any git hosting: GitHub, GitLab, Gitea, etc.)
- Enforce project conventions (branch naming, commit messages)

## Examples

- Branch management (create, delete, checkout)
- Commit operations with validation
- Working tree state management
- Local repository operations

## Note

If a script needs to interact with GitHub/GitLab APIs or their CLIs, it should be placed at the parent level (`file_versioning/`) as a hybrid script, or in a future platform-specific directory if we create pure platform scripts.
