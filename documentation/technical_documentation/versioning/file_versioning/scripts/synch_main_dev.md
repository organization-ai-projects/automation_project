# Script: synch_main_dev.sh

- [Back to File Versioning Index](TOC.md)

This document explains the usage of the `synch_main_dev.sh` script, which combines Git and GitHub operations to synchronize the `dev` branch with the `main` branch.

## Purpose

The `synch_main_dev.sh` script automates the process of merging changes from the `main` branch into the `dev` branch, ensuring that `dev` is always up-to-date with the latest changes from `main`. It uses GitHub CLI (`gh`) to create and manage pull requests for this synchronization.

## Usage

Run the script from the root of the repository:

```bash
./scripts/versioning/file_versioning/synch_main_dev.sh
```

## Features

1. **Validation**:
   - Ensures the script is run in a Git repository.
   - Checks for required tools (`gh` and `jq`).
   - Verifies that the working tree is clean before proceeding.

2. **Branch Synchronization**:
   - Updates the local `dev` branch with the remote `dev`.
   - Creates a temporary sync branch to merge `main` into `dev`.

3. **Pull Request Management**:
   - Creates a pull request from the sync branch to `dev`.
   - Enables auto-merge for the pull request.
   - Waits for the pull request to be merged.

4. **Cleanup**:
   - Deletes the temporary sync branch locally and remotely after the pull request is merged.

## Example Output

```bash
2026-01-25 12:00:00 INFO: Fetching remote refs...
2026-01-25 12:00:01 INFO: Updating local 'dev' from origin/dev...
2026-01-25 12:00:02 INFO: Creating sync branch 'sync/dev-with-main' from 'dev'...
2026-01-25 12:00:03 INFO: Merging origin/main into 'sync/dev-with-main'...
2026-01-25 12:00:04 INFO: Pushing 'sync/dev-with-main' to origin...
2026-01-25 12:00:05 INFO: No PR found. Creating PR...
2026-01-25 12:00:06 INFO: Enabling auto-merge for PR #123...
2026-01-25 12:00:07 INFO: Waiting for PR #123 to be merged...
2026-01-25 12:00:10 INFO: Pulling latest 'dev'...
2026-01-25 12:00:11 INFO: Deleting local and remote sync branch 'sync/dev-with-main'...
2026-01-25 12:00:12 INFO: OK Sync complete. Your local 'dev' is up to date. You can continue working.
```

## Notes

- The script assumes the remote is named `origin`. You can override this by setting the `REMOTE` environment variable.
- The merge method for the pull request can be customized using the `MERGE_METHOD` environment variable (`--merge`, `--squash`, or `--rebase`).
