# Git Scripts Documentation

This directory contains scripts that use **only** the `git` command-line tool.

## Role in the Project

This directory is responsible for pure git operations that are platform-agnostic and work with any git hosting service.
It interacts mainly with:

- Local git repositories (branch management, commits, working tree)
- Remote git repositories (push, fetch operations)
- Parent orchestrators (called by execute/read scripts)

## Directory Structure

```
git/
├── README.md (this file)
├── TOC.md
├── create_branch.sh           # Create branches with validation
├── delete_branch.sh           # Delete branches
├── push_branch.sh             # Push branches to remote
├── clean_branches.sh          # Clean obsolete branches
├── clean_local_gone.sh        # Remove branches with gone remotes
├── create_work_branch.sh      # Create work branches with conventions
├── finish_branch.sh           # Close work branches
├── add_commit_push.sh         # Add, commit, and push changes
├── create_after_delete.sh     # Recreate branch from base
└── cleanup_after_pr.sh        # Update branches after PR merge
```

## Files

- `README.md`: This file.
- `TOC.md`: Documentation index for git scripts.
- `create_branch.sh`: Create branches with validation.
- `delete_branch.sh`: Delete branches.
- `push_branch.sh`: Push branches to remote.
- `clean_branches.sh`: Clean obsolete branches.
- `clean_local_gone.sh`: Remove branches with gone remotes.
- `create_work_branch.sh`: Create work branches with conventions.
- `finish_branch.sh`: Close work branches.
- `add_commit_push.sh`: Add, commit, and push changes.
- `create_after_delete.sh`: Recreate branch from base.
- `cleanup_after_pr.sh`: Update branches after PR merge.

## Scope

Scripts in this directory should:

- Use only `git` commands (no `gh`, `gitlab-cli`, or other version control platform CLIs)
- Perform pure git operations (branches, commits, working tree, etc.)
- Be platform-agnostic (work with any git hosting: GitHub, GitLab, Gitea, etc.)

## Examples

- Branch management (create, delete, checkout)
- Commit operations
- Working tree state management
- Local repository operations

## Note

If a script needs to interact with GitHub/GitLab APIs or their CLIs, it should be placed at the parent level (`file_versioning/`) as a hybrid script, or in a future platform-specific directory if we create pure platform scripts.
