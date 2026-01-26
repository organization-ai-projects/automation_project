# Script: create_after_delete.sh

- [Back to Git Scripts Index](TOC.md)

This document explains the usage of the `create_after_delete.sh` script, which is dedicated to Git operations.

## Purpose

The `create_after_delete.sh` script automates the process of deleting a branch (both locally and remotely) and recreating it from the base branch (default: `dev`).

## Usage

Run the script from the root of the repository:

```bash
./scripts/versioning/file_versioning/git/create_after_delete.sh
```

## Features

1. **Branch Validation**:
   - Ensures the user is on a valid branch (not in detached HEAD state).
   - Prevents deletion of protected branches (`dev` and `main` by default).

2. **Delete Branches**:
   - Deletes the local branch if it exists.
   - Deletes the remote branch if it exists.

3. **Recreate Branch**:
   - Recreates the branch locally from the base branch.
   - Pushes the branch to the remote repository and sets the upstream.

## Example Output

```bash
=== Recreate branch: feature/new-feature (base: dev, remote: origin) ===
-> Checkout dev
-> Delete local branch feature/new-feature (safe)
OK Local branch "feature/new-feature" deleted.
-> Delete remote branch feature/new-feature (if exists)
OK Remote branch "feature/new-feature" deleted.
-> Create branch from dev
-> Push & set upstream
OK Branch "feature/new-feature" recreated from "dev" and pushed to "origin".
```

## Notes

- Ensure your working directory is clean before running the script.
- The script assumes the remote is named `origin`. You can override this by setting the `REMOTE` environment variable.
