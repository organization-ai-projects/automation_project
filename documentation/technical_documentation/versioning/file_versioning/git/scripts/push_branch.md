# Script: push_branch.sh

- [Back to Git Scripts Index](TOC.md)

This document explains the usage of the `push_branch.sh` script, which is dedicated to Git operations.

## Purpose

The `push_branch.sh` script is used to push the current branch to the remote repository while enforcing the following rules:

- **Protected Branches**: Direct pushes to `dev` and `main` are forbidden.
- **Upstream Configuration**: Automatically sets the upstream branch if it is not already configured.

## Usage

Run the script from the root of the repository:

```bash
./scripts/versioning/file_versioning/git/push_branch.sh
```

## Features

1. **Branch Validation**:
   - Ensures the user is on a valid branch (not in detached HEAD state).
   - Prevents pushes to protected branches (`dev` and `main`).

2. **Remote Synchronization**:
   - Fetches the latest references from the remote repository.

3. **Push Logic**:
   - If the upstream branch exists, performs a simple push.
   - If no upstream branch exists, configures it automatically with `--set-upstream`.

## Example Output

```bash
=== Push branch: feature/new-feature -> origin ===
OK Branch 'feature/new-feature' pushed to 'origin'.
```

## Notes

- Ensure your working directory is clean before running the script.
- The script assumes the remote is named `origin`. You can override this by setting the `REMOTE` environment variable.
