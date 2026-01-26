# Overview of Scripts

- [Back to File Versioning Index](TOC.md)
- [Back to File Versioning Scripts Index](scripts/TOC.md)

Use the dedicated indexes for script documentation by category:

- [File Versioning Scripts](scripts/TOC.md)
- [Git Scripts](git/scripts/TOC.md)
- [GitHub Scripts](github/scripts/TOC.md)

## Existing Scripts

### create_branch.sh

- **Description**: Automates the process of creating a new branch from a base branch (default: `dev`) and optionally pushing it to the remote repository.
- **Usage**: `./scripts/versioning/file_versioning/git/create_branch.sh <branch-name>`
- **Details**: See the [full documentation](git/scripts/create_branch.md).

---

### delete_branch.sh

- **Description**: Automates the process of deleting a branch locally and remotely. Stores the name of the deleted branch for potential recreation.
- **Usage**: `./scripts/versioning/file_versioning/git/delete_branch.sh <branch-name> [--force]`
- **Details**: See the [full documentation](git/scripts/delete_branch.md).

---

### push_branch.sh

- **Description**: Pushes the current branch to the remote repository while enforcing rules for protected branches.
- **Usage**: `./scripts/versioning/file_versioning/git/push_branch.sh`
- **Details**: See the [full documentation](git/scripts/push_branch.md).

---

### synch_main_dev.sh

- **Description**: Synchronizes `dev` with `main` via a pull request and auto-merge.
- **Usage**: `./scripts/versioning/file_versioning/synch_main_dev.sh`
- **Details**: See the [full documentation](scripts/synch_main_dev.md).

## Scripts to Create

### pre_push_check.sh

- **Description**: Ensures all necessary checks are performed before pushing changes.
- **Planned Usage**: `./pre_push_check.sh`
- **Steps**:
  - Check dependencies.
  - Check for merge conflicts.
  - Run tests.
- **Purpose**: Prevents issues from being pushed to the remote repository.

### create_pr.sh

- **Description**: Automates the creation of a pull request.
- **Planned Usage**: `./create_pr.sh`
- **Steps**:
  - Gather branch information.
  - Use GitHub CLI to create a PR.
- **Purpose**: Simplifies the PR creation process.

### clean_branches.sh

- **Description**: Cleans up obsolete local and remote branches.
- **Planned Usage**: `./clean_branches.sh`
- **Steps**:
  - Identify stale branches.
  - Remove them locally and remotely.
- **Purpose**: Keeps the repository clean and organized.

### check_dependencies.sh

- **Description**: Checks for outdated or missing dependencies.
- **Planned Usage**: `./check_dependencies.sh`
- **Steps**:
  - Analyze dependency files.
  - Report outdated or missing dependencies.
- **Purpose**: Ensures the project dependencies are up-to-date.

### check_merge_conflicts.sh

- **Description**: Checks for merge conflicts in local branches.
- **Planned Usage**: `./check_merge_conflicts.sh`
- **Steps**:
  - Compare branches.
  - Report conflicts.
- **Purpose**: Identifies potential merge issues early.

### pre_add_review.sh

- **Description**: Acts as an internal reviewer before adding changes to the staging area.
- **Planned Usage**: `./pre_add_review.sh`
- **Steps**:
  - Run `cargo fmt --check`.
  - Run `cargo clippy -D warnings`.
  - Run `cargo test`.
  - Check for regex patterns (e.g., `unwrap`, `expect`, `todo`).
  - Summarize the diff by touched crates.
- **Purpose**: Ensures all necessary checks are performed before staging changes.

### create_work_branch.sh

- **Description**: Creates a clean work branch from `dev` with conventions.
- **Planned Usage**: `./create_work_branch.sh`
- **Steps**:
  - Checkout `dev` and pull the latest changes.
  - Create a branch with a specific naming convention.
  - Set up upstream automatically.
- **Purpose**: Standardizes the process of creating work branches.

### finish_branch.sh

- **Description**: Ensures a clean closure of a work branch (local + remote).
- **Planned Usage**: `./finish_branch.sh`
- **Steps**:
  - Verify the branch is not `main` or `dev`.
  - Delete the local branch.
  - Delete the remote branch.
  - Run `fetch --prune`.
- **Purpose**: Simplifies branch cleanup.

### clean_local_gone.sh

- **Description**: Removes local branches whose remote counterparts have disappeared.
- **Planned Usage**: `./clean_local_gone.sh`
- **Steps**:
  - Identify local branches without remote counterparts.
  - Remove them safely.
- **Purpose**: Keeps the local repository clean.

### labels_sync.sh

- **Description**: Ensures that labels are consistent across the repository.
- **Planned Usage**: `./labels_sync.sh`
- **Steps**:
  - Read a `labels.toml` or `labels.json` file.
  - Create or update labels using GitHub CLI.
- **Purpose**: Prevents label drift and ensures consistency.

### changed_crates.sh

- **Description**: Outputs the crates touched in a diff.
- **Planned Usage**: `./changed_crates.sh`
- **Steps**:
  - Analyze the diff.
  - Identify affected crates.
- **Purpose**: Useful for auto-labeling PRs and future automation.

### ci_watch_pr.sh

- **Description**: Monitors the CI status of a PR.
- **Planned Usage**: `./ci_watch_pr.sh`
- **Steps**:
  - Use GitHub CLI to watch PR checks.
- **Purpose**: Simplifies CI monitoring for pull requests.

### scripts/base_scripts.sh

- **Description**: Provides common functions for all scripts (e.g., `info`, `warn`, `die`, repository verification, `require_clean_tree`, `require_cmd` for `gh`/`jq`, `git_fetch_prune`, etc.).
- **Planned Usage**: Source this library in other scripts to avoid duplication and standardize logging and return codes.
- **Purpose**: Prevents code duplication and ensures consistent behavior across scripts.

## Overview

This document provides an overview of existing scripts and scripts to be developed to improve Git workflow automation. Each script includes suggestions for improvement to ensure better efficiency and ease of use.
