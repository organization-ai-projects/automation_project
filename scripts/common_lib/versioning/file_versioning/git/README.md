# Git Utilities Documentation

Reusable utility functions for git operations.

## Role in the Project

This directory is responsible for providing low-level Git operation abstractions that encapsulate common repository tasks, branch management, working tree validation, commit operations, staging operations, and synchronization utilities.
It interacts mainly with:

- Git command-line interface
- Repository state and configuration
- Working tree and staging area
- Local and remote branches
- Commit history

## Directory Structure

```plaintext
git/
├── commands.sh         # Single git CLI entrypoint (vcs_local_* wrappers)
├── branch.sh           # Branch management utilities
├── commit.sh           # Commit operations
├── repo.sh             # Repository validation utilities
├── staging.sh          # Staging/index operations
├── synch.sh            # Synchronization utilities
└── working_tree.sh     # Working tree state validation
```

## Files

- `README.md`: This file.
- `commands.sh`: Single Git CLI backend wrappers (`vcs_local_*`).
- `branch.sh`: Branch management utilities.
- `commit.sh`: Commit operations.
- `repo.sh`: Repository validation utilities.
- `staging.sh`: Staging/index operations.
- `synch.sh`: Synchronization utilities.
- `working_tree.sh`: Working tree state validation.

## Scope

This directory contains foundational git functions used by other scripts:

- Repository and branch operations
- Working tree validation
- Commit operations
- Staging/index operations
- Synchronization utilities
- Single `git` invocation backend (`commands.sh`) for deduplication and consistency

## Invocation Rule

- `commands.sh` is the only file in this module that executes the `git` binary directly.
- Other scripts must call `vcs_local_*` wrappers (or higher-level helpers built on top of them).

## Wrapper API

All wrappers are variadic and forward extra args with `"$@"`.

- Primitive:
  - `vcs_local_run <git-args...>`
- Core:
  - `vcs_local_add`, `vcs_local_branch`, `vcs_local_checkout`, `vcs_local_commit`
  - `vcs_local_diff`, `vcs_local_fetch`, `vcs_local_pull`, `vcs_local_push`
  - `vcs_local_reset`, `vcs_local_status`, `vcs_local_switch`
  - `vcs_local_rev_parse`, `vcs_local_show_ref`, `vcs_local_ls_remote`, `vcs_local_ls_files`
  - `vcs_local_for_each_ref`, `vcs_local_rev_list`, `vcs_local_log`, `vcs_local_merge_base`

## Examples

```bash
vcs_local_branch --show-current
vcs_local_checkout -b feat/my-branch dev
vcs_local_push --set-upstream origin feat/my-branch
vcs_local_diff --cached --name-only
```

## Current Modules

### repo.sh

Repository validation utilities:

- `require_git_repo()` - Ensure running in a git repository

### branch.sh

Branch management utilities:

- `branch_exists_local()` - Check if local branch exists
- `branch_exists_remote()` - Check if remote branch exists
- `is_protected_branch()` - Check if branch is protected
- `get_current_branch()` - Get current branch name
- `require_non_protected_branch()` - Require branch is not protected
- `save_last_deleted_branch()` - Save deleted branch name
- `get_last_deleted_branch()` - Retrieve last deleted branch

### working_tree.sh

Working tree state validation:

- `require_clean_tree()` - Require working tree is clean
- `has_untracked_files()` - Check for untracked files
- `is_working_tree_dirty()` - Check if working tree is dirty

### staging.sh

Staging/index operations:

- `git_add_all()` - Add all changes to staging
- `git_add_files()` - Add specific files to staging
- `git_reset_all()` - Reset all staged changes
- `git_reset_files()` - Reset specific files
- `git_status()` - Get full git status
- `git_status_short()` - Get short git status

### commit.sh

Commit operations:

- `git_commit()` - Create a commit
- `git_commit_amend()` - Amend previous commit
- `git_commit_amend_message()` - Amend commit message only
- `has_staged_changes()` - Check for staged changes
- `has_unstaged_changes()` - Check for unstaged changes

### synch.sh

Synchronization utilities:

- `git_fetch_prune()` - Fetch from remote and prune deleted branches

## Adding New Git Utilities

When adding a git utility:

1. **Identify the category** - Does it fit existing file or need new one?
2. **Keep it focused** - One file = one category
3. **Make it reusable** - Should work across different contexts
4. **Document it** - Add to this README and in the script file

Good candidates for git utilities:

- Frequently used git operations
- Operations that need validation/error handling
- Operations used in multiple scripts
