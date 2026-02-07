# automation_markdown.yml Documentation

## Purpose

This automation workflow automatically fixes Markdown formatting issues in pull requests using markdownlint-cli2. Similar to `automation_rustfmt.yml`, it detects markdown files modified in a PR, checks for linting issues, applies fixes, and auto-commits the changes.

## Triggers

- **Pull Request**: Triggered on pull requests targeting `dev` and `main` branches.

## Conditions

- Only runs if the PR is from the same repository (not from forks)
- Skips the `sync/main-into-dev` branch to avoid interference with sync operations

## Steps

1. **Checkout**:
   - Checks out the PR branch with write permissions to allow auto-commits.
2. **Ensure local branch**:
   - Creates or switches to the local branch for the PR.
3. **Setup Node.js**:
   - Installs Node.js version 20 with npm caching for faster subsequent runs.
4. **Install Dependencies**:
   - Installs markdownlint-cli2 and its dependencies using `npm ci`.
5. **Check for markdown issues**:
   - Identifies modified `.md` files in the PR
   - Runs `npm run lint-md` to check for linting issues
   - Sets `needs_fixing=true` if issues are found
6. **Run markdownlint-cli2 --fix**:
   - Applies automatic fixes using `npm run lint-md-fix` if issues were detected
7. **Auto-commit markdownlint changes**:
   - Commits and pushes the fixes back to the PR branch with message "chore: apply markdownlint fixes"

## Configuration

- **Config File**: `.markdownlint-cli2.yaml` in the repository root
- **Ignored Paths**: `target/`, `node_modules/`, `ui_dist/`, `code_agent_sandbox/`
- **Rule Adjustments**:
  - Baseline configuration with relaxed rules to accommodate existing documentation
  - Line length, code block language specification, and inline HTML checks are disabled
  - Rules can be progressively tightened as documentation is updated

## Local Usage

Contributors can run markdown linting locally:

```bash
# Install dependencies (first time only)
npm install

# Lint all markdown files
npm run lint-md

# Auto-fix markdown issues
npm run lint-md-fix
```

## Pre-Push Integration

Markdown linting is integrated into the pre-push hook via `scripts/automation/pre_push_check.sh`. It will run automatically before push if npm is available. Can be bypassed with `SKIP_PRE_PUSH=1 git push` if needed.

## Behavior

This workflow automatically fixes formatting issues when documentation is edited directly on GitHub or through PRs, ensuring consistent markdown formatting without manual intervention.

## Related Files

- [workflows_overview.md](../../documentation/workflows_overview.md)
- [automation_rustfmt.yml](automation_rustfmt.md)
- [automation_sync.yml](automation_sync.md)
