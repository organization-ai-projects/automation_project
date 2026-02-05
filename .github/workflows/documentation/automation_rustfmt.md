# automation_rustfmt.yml Documentation

This workflow automates code formatting checks to ensure consistency across the codebase. It runs `rustfmt` only on Rust files modified in the PR and reports any formatting issues.

## Purpose

- Enforces consistent code formatting.
- Prevents unformatted code from being merged.

## Triggers

- Triggered on pull requests and pushes to specific branches.

## Steps

1. **Checkout Code**: Checks out the repository code.
2. **Run rustfmt on modified files**: Executes `rustfmt` only for Rust files changed in the PR.
3. **Report Issues**: Reports any formatting issues found.

## Contribution

Note: This workflow intentionally formats only modified files, not the entire repository. Existing unformatted code outside the PR remains unchanged.

Contributors should ensure their code is formatted before pushing changes to avoid workflow failures.
