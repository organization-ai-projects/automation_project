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
  - Baseline configuration currently has relaxed rules to accommodate existing documentation
  - Line length, code block language specification, and inline HTML checks are disabled
  - Rules can be progressively tightened as documentation is updated
  - Multiple blank lines and emphasis as headings are allowed for readability

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

## Related Files

- [workflows_overview.md](../../documentation/workflows_overview.md)
- [ci_main.yml](ci_main.md)
- [ci_dev.yml](ci_dev.md)
