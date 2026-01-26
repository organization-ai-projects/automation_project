# Synchronization After a PR

- [Back to File Versioning Index](TOC.md)

This guide describes how to synchronize your local repository after a PR is
merged into `dev`.

## Standard Cleanup

1. **Update the local `dev` branch**:

   ```bash
   git checkout dev
   git pull origin dev
   ```

2. **Delete the local working branch if no longer needed**:

   ```bash
   git branch -d feature/<name>
   ```

3. **Delete the remote working branch if no longer needed**:

   ```bash
   git push origin --delete feature/<name>
   ```

4. **Create a new working branch if necessary**:

   ```bash
   git checkout -b feature/<new-task>
   ```

## Managing Persistent Working Branches

If you want to keep a working branch for later:

1. **Update the working branch with `dev`**:

   ```bash
   git checkout feature/<name>
   git pull origin dev
   git merge dev
   ```

2. **Push updates to the remote branch**:

   ```bash
   git push origin feature/<name>
   ```

3. **Resume work**:
   Continue working on the branch. Once finished, create a new PR to merge the
   changes into `dev`.

4. **Delete the branch if no longer useful**:
   If the branch is no longer useful, follow the deletion steps above.

## Automate with Scripts

For automated cleanup of outdated branches after a PR, see:

- [Script: cleanup_after_pr.sh](git/scripts/cleanup_after_pr.md)
