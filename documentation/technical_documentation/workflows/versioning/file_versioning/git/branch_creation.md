# Branch Creation Guidelines

- [Back to Git Index](TOC.md)

This document explains how to create and manage branches in the `automation_project`.

## Main Branches

### `main`

- **Description**: Stable branch containing validated and production-ready versions.
- **Rule**: No direct commits. Changes come only from the `dev` branch after stabilization via PR.

### `dev`

- **Description**: Development branch containing features under testing.
- **Rule**: No direct commits. Changes come only from working branches via PRs.

## Steps to Create a New Branch

1. **Switch to the `dev` branch**:

   ```bash
   git checkout dev
   ```

2. **Update the `dev` branch**:

   ```bash
   git pull origin dev
   ```

3. **Create a new branch**:

   ```bash
   git checkout -b <branch-name>
   ```

   Replace `<branch-name>` with a descriptive name, such as:
   - `feature/<name>` for new features.
   - `fix/<name>` for bug fixes.

4. **Push the new branch to the remote repository**:

   ```bash
   git push -u origin <branch-name>
   ```

## Naming Conventions

- Use `feature/<name>` for new features.
- Use `fix/<name>` for bug fixes.
- Use lowercase letters and hyphens to separate words.

## Additional Naming Notes

- Prefer descriptive, scoped names (example: `feature/ui-improvements`,
  `fix/bug-123`).

## Automation Available

To automate branch creation, see the [scripts documentation](../../../../../../scripts/versioning/file_versioning/git/README.md).
