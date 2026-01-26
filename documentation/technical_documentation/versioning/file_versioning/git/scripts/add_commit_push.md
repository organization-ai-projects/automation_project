# Script: add_commit_push.sh

- [Back to Git Scripts Index](TOC.md)

This document explains the usage of the `add_commit_push.sh` script, which is dedicated to Git operations.

## Purpose

The `add_commit_push.sh` script automates the process of adding changes, committing them, and pushing the changes to the remote repository.

## Usage

Run the script from the root of the repository, providing a commit message as an argument:

```bash
./scripts/versioning/file_versioning/git/add_commit_push.sh "Your commit message"
```

## Features

1. **Add Changes**:
   - Stages all changes in the working directory using `git add .`.

2. **Commit Changes**:
   - Creates a commit with the provided message using `git commit -m`.

3. **Push Changes**:
   - Calls the `push_branch.sh` script to push the changes to the remote repository.

## Example Output

```bash
=== git add ===
=== git commit ===
[feature/new-feature 1234567] Your commit message
=== git push (via push_branch.sh) ===
OK Commit and push completed successfully.
```

## Notes

- Ensure your working directory is clean before running the script.
- The script depends on `push_branch.sh` to handle the push operation.
