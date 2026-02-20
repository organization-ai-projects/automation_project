# Git Hooks Documentation

This directory contains custom git hooks to ensure code and commit quality.

## Role in the Project

This directory is responsible for enforcing code quality and commit standards through automated validation at key points in the git workflow.
It interacts mainly with:

- Git commit and push workflow
- Cargo formatting and linting tools
- Test infrastructure
- Commit message conventions
- Changed file detection system

## Directory Structure

```plaintext
git_hooks/
├── commit-msg          # Validates commit message format
├── pre-commit          # Runs code formatting before commit
├── prepare-commit-msg  # Auto-generates commit subject from context
├── pre-push            # Runs quality checks before push
├── install_hooks.sh    # Installs git hooks (worktree-aware)
└── tests/
    ├── convention_guardrails_regression.sh  # Regression tests for issue trailer guardrails
    └── fixtures/                             # Commit message fixtures for allow/block cases
```

## Files

- `README.md`: This file.
- `commit-msg`: Validates commit message format.
- `pre-commit`: Runs formatting before commit.
- `prepare-commit-msg`: Auto-generates commit subject from branch/staged files.
- `pre-push`: Runs quality checks before push.
- `install_hooks.sh`: Installs hooks to the correct git hooks directory (supports standard clones and worktrees).
- `tests/convention_guardrails_regression.sh`: Regression suite for issue trailer guardrails in `commit-msg`, `pre-push`, and `post-checkout`.

## Available hooks

### `commit-msg`

Validates commit message format according to project conventions.

**Expected format:**

```plaintext
<type>(<scope>): <message>
```

or

```plaintext
<type>: <message>
```

**Allowed types:**

- `feature`, `feat` - New feature
- `fix` - Bug fix
- `fixture` - Test data or fixtures
- `doc`, `docs` - Documentation
- `refactor` - Refactoring
- `test`, `tests` - Tests
- `chore` - Maintenance tasks

**Valid examples:**

```bash
feat(auth): add user authentication
fix: resolve null pointer exception
docs(readme): update installation instructions
refactor(api): simplify error handling
docs(.github): add default PR template
```

**Bypass (emergency only):**

```bash
SKIP_COMMIT_VALIDATION=1 git commit -m "emergency fix"
```

### `pre-commit`

Runs code formatting before each commit:

1. **Protected branch guard**: blocks direct commits on `dev` and `main`
2. **Formatting**: `cargo fmt --all`

Automatically adds formatted files to staging.

**Bypass (emergency only):**

```bash
SKIP_PRE_COMMIT=1 git commit -m "message"
ALLOW_PROTECTED_BRANCH_COMMIT=1 git commit -m "message"
```

### `prepare-commit-msg`

Auto-generates a conventional commit subject when the commit message is empty.

Inputs used:

1. Branch naming prefix (`feat/`, `fix/`, `docs/`, etc.) to infer type
2. Staged files to infer required scopes and fallback type
3. Branch slug to derive a readable short description

It does not override:

- Explicit messages provided with `git commit -m`
- Merge/squash/amend commit messages
- Non-empty commit message templates

**Bypass (emergency only):**

```bash
SKIP_PREPARE_COMMIT_MSG=1 git commit
```

### `pre-push`

Runs quality checks before each push, with selective execution:

1. **Formatting**: `cargo fmt --all --check`
2. **Linting**: `cargo clippy` (only on affected crates)
3. **Tests**: `cargo test` (only on affected crates)
4. **Docs/scripts-only mode**: skips Rust checks and runs lightweight shell syntax checks

#### Selection logic

The hook uses two layers:

- If the push only changes docs/scripts/workflow files, Rust checks are skipped.
- Otherwise, commit scopes are used to target specific crates.
- If scopes are invalid/missing, it falls back to full workspace checks.

**Bypass (emergency only):**

```bash
SKIP_PRE_PUSH=1 git push
```

## Installation

Run the installation script:

```bash
./scripts/automation/git_hooks/install_hooks.sh
```

This script copies the hooks into the git hooks directory (resolved via `git rev-parse --git-path hooks`, supporting both standard clones and worktrees) and makes them executable.

## Architecture

The hooks are:

- **Custom bash scripts** - Consistent with the existing infrastructure
- **Autonomous** - No external dependencies (npm, cargo-husky, etc.)
- **Bypassable** - Environment variables for emergencies
- **Informative** - Clear messages about what is checked and how to fix it
- **Smart** - Scope detection to avoid unnecessary tests

## Maintenance

To update the hooks after changes:

```bash
./scripts/automation/git_hooks/install_hooks.sh
```

To run guardrail regression tests:

```bash
./scripts/automation/git_hooks/tests/convention_guardrails_regression.sh
```

To temporarily disable a hook:

```bash
# Rename the hook in the git hooks directory (resolved via git rev-parse --git-path hooks)
GIT_HOOKS_DIR="$(git rev-parse --git-path hooks)"
mv "$GIT_HOOKS_DIR/pre-push" "$GIT_HOOKS_DIR/pre-push.disabled"
```

To re-enable it:

```bash
GIT_HOOKS_DIR="$(git rev-parse --git-path hooks)"
mv "$GIT_HOOKS_DIR/pre-push.disabled" "$GIT_HOOKS_DIR/pre-push"
```
