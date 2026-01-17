# Git/GitHub Workflow

This document describes the Git/GitHub workflow used for the `automation_project`.

---

## 1. Main Branches

### `main`

- **Description**: Stable branch containing validated and production-ready versions.
- **Rule**: No direct commits. Changes come only from the `dev` branch after stabilization.

### `dev`

- **Description**: Development branch containing features under testing.
- **Rule**: No direct commits. Changes come only from working branches via PRs.

---

## 2. Working Branches

### Creation

- A working branch is created for each new feature or bug fix.
- **Naming Convention**: `feature/<name>` or `fix/<name>`.
  - Example: `feature/ui-improvements`, `fix/bug-123`.

### Merging

- Working branches are merged into `dev` via a Pull Request (PR).
- **Rule**:
  - The PR must be approved before merging.
  - Tests must pass before merging.

---

## 3. Merging Process

### From `dev` to `main`

1. Stabilize the `dev` branch.
2. Perform thorough testing.
3. Create a PR from `dev` to `main`.
4. Once approved, merge into `main`.

### From Working Branch to `dev`

1. Create a PR from the working branch to `dev`.
2. Wait for approval and ensure tests pass.
3. Merge into `dev`.

---

## 4. Synchronization after a PR

Once a PR from your working branch has been merged into `dev`, you need to synchronize your local repository to stay up to date:

1. **Update the local `dev` branch** :

   ```bash
   git checkout dev
   git pull origin dev
   ```

2. **Delete the local working branch if no longer needed** :

   ```bash
   git branch -d feature/<name>
   ```

3. **Delete the remote working branch if no longer needed** :

   ```bash
   git push origin --delete feature/<name>
   ```

4. **Create a new working branch if necessary** :
   If you are starting a new task, create a new branch from the updated version of `dev`:

   ```bash
   git checkout -b feature/<new-task>
   ```

### Managing Persistent Working Branches

If you want to keep a working branch for later:

1. **Update the working branch with `dev`** :
   Before resuming work on an existing branch, ensure it is synchronized with the latest changes from `dev`:

   ```bash
   git checkout feature/<name>
   git pull origin dev
   git merge dev
   ```

2. **Push updates to the remote branch** :
   If you have merged or added changes, push them to the remote branch to avoid divergences:

   ```bash
   git push origin feature/<name>
   ```

3. **Resume work** :
   Continue working on the branch as usual. Once finished, create a new PR to merge the changes into `dev`.

4. **Delete the branch if no longer needed** :
   If the branch is no longer useful, follow the deletion steps mentioned above.

---

## 5. General Rules

- **Tests**: Any changes must be accompanied by tests (unit, integration, etc.). Tests may be temporarily absent during exploratory phases but are required before any merge from `dev` to `main`. Tests can be run locally or via CI when available.
- **Commits**: Commit messages must be clear and follow a convention (e.g.: `fix: correct bug in X`, `feat: add new feature Y`).
- **Merge**: Merge should be strictly used to integrate changes, in order to preserve the integrity and complete history of commits.

---

## 6. Useful Commands

### Initializing the local repository

```bash
git clone https://github.com/organization-ai-projects/automation_project.git
cd automation_project
git checkout -b dev origin/dev
```

### Creating a working branch

```bash
git checkout dev
git pull origin dev
git checkout -b feature/<name>
```

### Merging a working branch into `dev`

```bash
git checkout dev
git pull origin dev
git merge feature/<name>
git push origin dev
```

### Merging `dev` into `main`

```bash
git checkout main
git pull origin main
git merge dev
git push origin main
```

### Automation with scripts

To simplify certain repetitive tasks, here are some scripts you can use:

#### Script: Create a new working branch

```bash
#!/bin/bash
# Usage: ./create_branch.sh <branch-name>

if [ -z "$1" ]; then
  echo "Error: You must specify a branch name."
  exit 1
fi

BRANCH_NAME=$1

git checkout dev
if [ $? -ne 0 ]; then
  echo "Error: Unable to switch to dev branch."
  exit 1
fi

git pull origin dev
if [ $? -ne 0 ]; then
  echo "Error: Unable to update dev."
  exit 1
fi

git checkout -b $BRANCH_NAME
if [ $? -eq 0 ]; then
  echo "Branch $BRANCH_NAME created successfully."
else
  echo "Error: Unable to create branch $BRANCH_NAME."
fi
```

#### Script: Delete a local and remote branch

```bash
#!/bin/bash
# Usage: ./delete_branch.sh <branch-name>

if [ -z "$1" ]; then
  echo "Error: You must specify a branch name."
  exit 1
fi

BRANCH_NAME=$1

git branch -d $BRANCH_NAME
if [ $? -eq 0 ]; then
  echo "Local branch $BRANCH_NAME deleted."
else
  echo "Error: Unable to delete local branch $BRANCH_NAME."
  exit 1
fi

git push origin --delete $BRANCH_NAME
if [ $? -eq 0 ]; then
  echo "Remote branch $BRANCH_NAME deleted."
else
  echo "Error: Unable to delete remote branch $BRANCH_NAME."
fi
```

---

## Before making a commit

Before creating a commit, follow these steps to ensure compliance with project conventions:

1. **Check the repository status** :
   - Use `git status` to see modified and added files.
   - Use `git diff` to review unstaged changes.
   - Use `git branch` to confirm you are on a working branch and identify which one.

2. **Follow commit conventions** :
   - The commit message must comply with [SemVer](https://semver.org/).
   - Use the following convention for the scope :
     - `libraries/[library name]` for libraries.
     - `products/[product name]` for products.
   - The message must be properly structured, detailed, and written in English for internationalization.

Example of a commit message :

```text
feat(libraries/hybrid_arena): add new library with SlotArena and BumpArena implementations

- Implemented SlotArena with allocation, removal, and generation tracking.
- Added BumpArena for efficient memory allocation.
- Included comprehensive tests for both arenas.
- Added benchmarks for allocation, access, and iteration performance.
- Fixed Clippy warnings and ensured code adheres to best practices.
```

---

**This workflow ensures clean and collaborative code management.**
