# Pull Request and Merging

[Back to Git Workflow](TOC.md) | [Back to Versioning](../TOC.md)

---

## Overview

This document describes the pull request (PR) workflow for this project. PRs are the primary mechanism for integrating changes into shared branches.

---

## Before Opening a PR

1. **Ensure your branch is up to date** with `dev`:

   ```bash
   git fetch origin
   git rebase origin/dev
   ```

2. **Run tests locally**:

   ```bash
   cargo test --workspace
   ```

3. **Check formatting and lints**:

   ```bash
   cargo fmt --check
   cargo clippy --workspace
   ```

4. **Verify your commits** are clean and descriptive.

---

## Creating a Pull Request

### Title Convention

Use a clear, descriptive title following this pattern:

```text
<type>: <short description>
```

**Types**:

- `feat` – New feature
- `fix` – Bug fix
- `doc` – Documentation only
- `refactor` – Code refactoring (no behavior change)
- `test` – Adding or updating tests
- `chore` – Maintenance tasks

**Examples**:

- `feat: Add user authentication module`
- `fix: Resolve panic in JSON parser`
- `doc: Update API documentation`

### Description

Include in your PR description:

- **What** the PR does
- **Why** it's needed (link to issue if applicable)
- **How** to test it
- Any **breaking changes**

### Linking Issues

Reference related issues using keywords:

```text
Closes #123
Fixes #456
Resolves #789
```

---

## Review Process

1. **Assign reviewers** – At least one reviewer is required.
2. **Address feedback** – Respond to comments and push fixes.
3. **Re-request review** after making changes.
4. **Approval required** – PRs need at least one approval before merging.

### Review Checklist

Reviewers should verify:

- [ ] Code compiles without warnings
- [ ] Tests pass
- [ ] Documentation is updated if needed
- [ ] No unnecessary changes included
- [ ] Commit history is clean

---

## Merging

### Merge Strategy

This project uses **squash and merge** for most PRs to keep history clean.

### Before Merging

- All CI checks must pass
- At least one approval is required
- No unresolved conversations

### After Merging

1. Delete the feature branch (GitHub can do this automatically)
2. Sync your local branches:

   ```bash
   git checkout dev
   git pull origin dev
   git branch -d <your-branch>
   ```

---

## See Also

- [Branch Creation](branch_creation.md)
- [Commit Guidelines](commit.md)
- [Push Workflow](push.md)
- [Sync After PR](../sync_after_pr.md)
