# Push Guidelines

- [Back to Git Index](TOC.md)

This document explains how to push changes to the remote repository in the `automation_project`.

## Steps to Push

1. **Ensure your branch is up to date**:

   ```bash
   git pull origin <branch>
   ```

   This ensures your branch is synchronized with the remote branch.

2. **Push your changes**:

   ```bash
   git push origin <branch>
   ```

   Replace `<branch>` with the name of your current branch.

3. **Verify the push**:
   Check the remote repository to ensure your changes have been pushed successfully.

## Automate with Scripts

To simplify pushing changes, you can use the `push_branch.sh` script:

```bash
./scripts/versioning/file_versioning/git/push_branch.sh <branch>
```

This script ensures your branch is up to date and pushes changes to the remote repository.

See the script documentation for details: [Script: push_branch.sh](scripts/push_branch.md).
