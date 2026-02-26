# automation_markdown.yml Documentation

## Purpose

This automation workflow automatically fixes Markdown formatting issues in pull requests using markdownlint-cli2. Similar to `automation_rustfmt.yml`, it detects markdown files modified in a PR, checks for linting issues, applies fixes, and auto-commits the changes.

## Triggers

* **Pull Request**: Triggered on pull requests targeting `dev` and `main` branches.

## Conditions

* Only runs if the PR is from the same repository (not from forks)
* Skips the `sync/main-into-dev` branch to avoid interference with sync operations
* Skips markdown files under `.github/workflows/` in auto-fix mode (GitHub App token lacks `workflows` permission for workflow-file updates)

## Steps

1. **Checkout**:

   * Checks out the PR branch with write permissions to allow auto-commits.
2. **Ensure local branch**:

   * Creates or switches to the local branch for the PR.
3. **Setup pnpm**:

   * Installs pnpm version 9 for dependency management.
4. **Setup Node.js**:

   * Installs Node.js version 20 with pnpm caching for faster subsequent runs.
5. **Install Dependencies**:

   * Installs markdownlint-cli2 and its dependencies using `pnpm install --frozen-lockfile`.
6. **Check for markdown issues**:

   * Identifies modified `.md` files in the PR
   * Runs markdownlint-cli2 directly on the modified files only
   * Sets `needs_fixing=true` if issues are found
7. **Run markdownlint-cli2 --fix**:

   * Applies automatic fixes using markdownlint-cli2 --fix on modified files only
8. **Auto-commit markdownlint changes**:

   * Commits and pushes the fixes back to the PR branch with message "chore: apply markdownlint fixes"

## Configuration

* **Config File**: `.markdownlint-cli2.yaml` in the repository root
* **Ignored Paths**: `target/`, `node_modules/`, `ui_dist/`, `code_agent_sandbox/`
* **Rule Adjustments**:

  * Baseline configuration with relaxed rules to accommodate existing documentation
  * Line length, code block language specification, and inline HTML checks are disabled
  * Rules can be progressively tightened as documentation is updated

## Local Usage

Contributors can run Markdown linting locally.

### Install pnpm (official and recommended)

pnpm must be installed via **Corepack**, which is the official package manager shim shipped with Node.js.

**Requirements**

* Node.js >= 16.13 (Node.js 18+ or 20+ recommended)

```bash
# Enable Corepack
corepack enable

# Install and activate pnpm (project uses pnpm v9)
corepack prepare pnpm@9 --activate

# Verify installation
pnpm --version
```

This is the **officially recommended way** to install pnpm.
It does **not** rely on `npm install -g`.

### Install dependencies (first time only)

```bash
pnpm install --frozen-lockfile
```

### Lint all markdown files

```bash
pnpm run lint-md
```

### Auto-fix markdown issues

```bash
pnpm run lint-md-fix
```

### Lint specific files

```bash
pnpm run lint-md-files file1.md file2.md
```

### Auto-fix specific files

```bash
pnpm run lint-md-fix-files file1.md file2.md
```

### About `pnpm dlx`

`pnpm dlx` is equivalent to `npx`: it **runs a package**. It does **not** install pnpm itself.

```bash
# Run markdownlint-cli2 without a global install
pnpm dlx markdownlint-cli2 "**/*.md"

# Apply fixes
pnpm dlx markdownlint-cli2 --fix "**/*.md"
```

## Pre-Push Integration

Markdown linting is integrated directly into the Git pre-push hook at `scripts/automation/git_hooks/pre-push` (not via `scripts/automation/pre_push_check.sh`).

Behavior:

* if changed files include markdown, pre-push runs `pnpm run lint-md-files -- ...` on those files
* if markdown dependencies are missing, pre-push fails with setup instructions (`pnpm install --frozen-lockfile`)
* `SKIP_PRE_PUSH=1 git push` bypasses all pre-push checks (emergency-only path)

## Pre-Commit Integration

The Git pre-commit hook (`scripts/automation/git_hooks/pre-commit`) also runs markdownlint on staged markdown files.
This gives fast feedback before commit, while pre-push remains a second safety gate.

## Behavior

This workflow automatically fixes formatting issues when documentation is edited directly on GitHub or through PRs, ensuring consistent markdown formatting without manual intervention. The workflow only processes markdown files that were actually modified in the PR, avoiding unintended changes to unrelated files.

## Related Files

* [workflows_overview.md](../../documentation/workflows_overview.md)
* [automation_rustfmt.yml](automation_rustfmt.md)
* [automation_sync.yml](automation_sync.md)
