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
├── branch.sh           # Branch management utilities
├── commit.sh           # Commit operations
├── repo.sh             # Repository validation utilities
├── staging.sh          # Staging/index operations
├── synch.sh            # Synchronization utilities
└── working_tree.sh     # Working tree state validation
```

## Scope

This directory contains foundational git functions used by other scripts:

- Repository and branch operations
- Working tree validation
- Commit operations
- Staging/index operations
- Synchronization utilities

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
