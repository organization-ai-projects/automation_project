# ci_markdown.yml Documentation

## Purpose

This workflow enforces Markdown formatting and linting standards across the repository using markdownlint-cli2. It ensures consistent documentation quality and catches common Markdown issues.

## Triggers

- **Push**: Triggered on pushes to `main` and `dev` branches.
- **Pull Request**: Triggered on pull requests targeting `main` and `dev` branches.

## Steps

1. **Checkout**:
   - Checks out the repository code.
2. **Setup Node.js**:
   - Installs Node.js version 20 with npm caching for faster subsequent runs.
3. **Install Dependencies**:
   - Installs markdownlint-cli2 and its dependencies using `npm ci`.
4. **Run Markdown Lint**:
   - Executes `npm run lint-md` to check all `*.md` files against configured rules.

## Configuration

- **Config File**: `.markdownlint-cli2.yaml` in the repository root
- **Ignored Paths**: `target/`, `node_modules/`, `ui_dist/`, `code_agent_sandbox/`
- **Rule Adjustments**:
  - Line length relaxed to 150 characters (excluding code blocks and tables)
  - Inline HTML allowed (common in GitHub Markdown)
  - Multiple blank lines allowed for readability
  - First line heading requirement disabled

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

Markdown linting is integrated into the pre-push hook via `scripts/automation/pre_push_check.sh`. It will run automatically before push if npm is available.

## Related Files

- [workflows_overview.md](../../documentation/workflows_overview.md)
- [ci_main.yml](ci_main.md)
- [ci_dev.yml](ci_dev.md)
