# Script: delete_branch.sh

- [Back to Git Scripts Index](TOC.md)

This document explains the usage of the `delete_branch.sh` script, which is dedicated to Git operations.

## Purpose

The `delete_branch.sh` script automates the process of deleting a branch locally and remotely. It also stores the name of the deleted branch for potential recreation.

## Usage

Run the script from the root of the repository:

```bash
./scripts/versioning/file_versioning/git/delete_branch.sh <branch-name> [--force]
```

- `<branch-name>`: The name of the branch to delete.
- `--force` (optional): Force deletion of the local branch.

## Features

1. **Branch Validation**:
   - Ensures the branch name is valid and not protected (`dev` or `main`).
   - Prevents deletion of the branch currently checked out.

2. **Delete Branches**:
   - Deletes the local branch (safe by default, forced with `--force`).
   - Deletes the remote branch if it exists.

3. **Store Deleted Branch Name**:
   - Saves the name of the deleted branch in `/tmp/last_deleted_branch` for potential recreation using the `create_branch.sh` script.

## Example Output

```bash
=== Delete branch: feature/old-feature (remote: origin) ===
OK Local branch 'feature/old-feature' deleted.
OK Remote branch 'feature/old-feature' deleted.
```

## Notes

- Ensure your working directory is clean before running the script.
- The script assumes the remote is named `origin`. You can override this by setting the `REMOTE` environment variable.
