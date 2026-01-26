# Script: create_branch.sh

- [Back to Git Scripts Index](TOC.md)

This document explains the usage of the `create_branch.sh` script, which is dedicated to Git operations.

## Purpose

The `create_branch.sh` script automates the process of creating a new branch from a base branch (default: `dev`) and optionally pushing it to the remote repository.

## Usage

Run the script from the root of the repository:

```bash
./scripts/versioning/file_versioning/git/create_branch.sh <branch-name>
```

If `<branch-name>` is omitted, the script attempts to recreate the last deleted branch using the `/tmp/last_deleted_branch` file.

## Features

1. **Branch Validation**:
   - Ensures the branch name is valid and not protected (`dev` or `main`).
   - Refuses branch names containing spaces.

2. **Branch Creation**:
   - Creates a new branch locally from the base branch (`dev` by default).
   - If the branch already exists locally, performs a `checkout`.

3. **Push to Remote**:
   - Pushes the branch to the remote repository and sets the upstream.

## Example Output

```bash
=== Create branch: feature/new-feature (base: dev) ===
INFO The local branch 'feature/new-feature' already exists. Checkout.
OK Branch 'feature/new-feature' pushed to 'origin' with upstream.
```

## Notes

- Ensure your working directory is clean before running the script.
- The script assumes the remote is named `origin`. You can override this by setting the `REMOTE` environment variable.
