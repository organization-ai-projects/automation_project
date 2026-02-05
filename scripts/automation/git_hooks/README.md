# Git Hooks Documentation

Custom git hooks to ensure code and commit quality.

## Role in the Project

This directory is responsible for enforcing code quality and commit standards through automated validation at key points in the git workflow.
It interacts mainly with:

- Git commit and push workflow
- Cargo formatting and linting tools
- Test infrastructure
- Commit message conventions
- Changed file detection system

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
├── pre-push            # Runs quality checks before push
└── install_hooks.sh    # Installs git hooks to .git/hooks/
```

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
```

**Bypass (emergency only):**

```bash
SKIP_COMMIT_VALIDATION=1 git commit -m "emergency fix"
```

### `pre-commit`

Runs code formatting before each commit:

1. **Formatting**: `cargo fmt --all`

Automatically adds formatted files to staging.

**Bypass (emergency only):**

```bash
SKIP_PRE_COMMIT=1 git commit -m "message"
```

### `pre-push`

Runs quality checks before each push, **with smart scope detection**:

1. **Formatting**: `cargo fmt --all --check`
2. **Linting**: `cargo clippy` (only on affected crates)
3. **Tests**: `cargo test` (only on affected crates)

#### Scope detection

The hook analyzes changed files and only tests the impacted crates:

```plaintext
projects/products/accounts/backend/src/...  → tests accounts-backend
projects/libraries/security/src/...         → tests security
projects/products/core/engine/src/...       → tests engine
```

If no changes are detected, a full workspace test is run.

**Bypass (emergency only):**

```bash
SKIP_PRE_PUSH=1 git push
```

## Installation

Run the installation script:

```bash
./scripts/git_hooks/install_hooks.sh
```

This script copies the hooks into `.git/hooks/` and makes them executable.

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
./scripts/git_hooks/install_hooks.sh
```

To temporarily disable a hook:

```bash
# Rename the hook in .git/hooks/
mv .git/hooks/pre-push .git/hooks/pre-push.disabled
```

To re-enable it:

```bash
mv .git/hooks/pre-push.disabled .git/hooks/pre-push
```
