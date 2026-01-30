# Commit Guidelines

- [Back to Git Index](TOC.md)

This document explains how to properly create a commit in the `automation_project`.

## Steps to Commit

1. **Check the repository status**:

   ```bash
   git status
   ```

   This shows the files that have been modified, added, or deleted.

2. **Review changes**:

   ```bash
   git diff
   ```

   Use this command to review unstaged changes.

3. **Stage changes**:

   ```bash
   git add <file>
   ```

   Add the files you want to include in the commit.

4. **Review staged changes**:

   ```bash
   git diff --cached
   ```

   Ensure the staged changes are correct.

5. **Write a clear commit message**:
   Follow the convention:

   ```text
   <type>(<scope>): <summary>

   - Detail 1
   - Detail 2
   ```

   Example:

   ```text
   feat(libraries/hybrid_arena): add SlotArena implementation

   - Added SlotArena with allocation and removal.
   - Included tests and benchmarks.
   ```

6. **Create the commit**:

   ```bash
   git commit -m "<commit message>"
   ```

## Automation Available

To automate add + commit + push, see the [scripts documentation](../../../../../../scripts/versioning/file_versioning/git/README.md).

## Commit Message Types

- `feat`: A new feature.
- `fix`: A bug fix.
- `docs`: Documentation changes.
- `style`: Code style changes (formatting, etc.).
- `refactor`: Code refactoring without changing functionality.
- `test`: Adding or updating tests.
- `chore`: Maintenance tasks (e.g., dependency updates).

## Additional Commit Conventions

- **Structure**: Commit messages should be clear and follow a convention (for
  example: `fix: correct bug in X`, `feat: add new feature Y`).
- **Example**:

```text
feat(libraries/hybrid_arena): add SlotArena and BumpArena implementations

- Implement SlotArena with allocation, removal, and generation tracking.
- Add BumpArena for efficient memory allocation.
- Add tests for both arenas.
- Add benchmarks for allocation, access, and iteration performance.
```
