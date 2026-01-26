# Script: cleanup_after_pr.sh

- [Back to Git Scripts Index](TOC.md)

This document explains the usage of the `cleanup_after_pr.sh` script, which is dedicated to Git operations.

## Purpose

The `cleanup_after_pr.sh` script automates the process of cleaning up outdated branches after a pull request (PR) has been merged. It ensures that local and remote branches are synchronized with the base branch.

## Usage

Run the script from the root of the repository:

```bash
./scripts/versioning/file_versioning/git/cleanup_after_pr.sh
```

## Features

1. **Update Base Branch**:
   - Updates the base branch (default: `dev`) by pulling the latest changes from the remote repository.

2. **Detect Outdated Branches**:
   - Identifies local branches that are behind the base branch.
   - Ignores protected branches (`dev` and `main` by default).

3. **Delete and Recreate Branches**:
   - Deletes outdated local and remote branches.
   - Recreates the branches from the updated base branch.

4. **Branch Protection**:
   - Ensures that protected branches are not modified.

## Example Output

```bash
=== Updating branch dev ===
OK Branch dev updated.

=== Detecting local branches behind dev ===
  -> feature/old-feature is behind by 3 commit(s) on dev
Target branches:
 - feature/old-feature

=== Deleting and recreating branches ===
Processing: feature/old-feature
  OK Local branch deleted.
  OK Remote branch deleted.
  OK Branch recreated.

OK Cleanup complete.
```

## Notes

- Ensure your working directory is clean before running the script.
- The script assumes the remote is named `origin`. You can override this by setting the `REMOTE` environment variable.
