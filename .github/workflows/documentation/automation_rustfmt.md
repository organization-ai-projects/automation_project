# automation_rustfmt.yml Documentation

This workflow automates code formatting checks to ensure consistency across the codebase. It runs `rustfmt` only on Rust files modified in the PR and reports any formatting issues.

## Purpose

- Enforces consistent code formatting.
- Prevents unformatted code from being merged.

## Triggers

- Triggered on pull requests and pushes to specific branches.

## Steps

1. **Checkout Code**: Checks out the repository code.
2. **Install Rust**: Installs the Rust toolchain with `rustfmt`.
3. **Check formatting**: Checks modified Rust files for formatting issues.
4. **Run rustfmt**: Formats modified Rust files that need changes.
5. **Auto-commit**: Commits formatting changes back to the PR branch.

## Contribution

Note: This workflow intentionally formats only modified files, not the entire repository. Existing unformatted code outside the PR remains unchanged.

Contributors should ensure their code is formatted before pushing changes to avoid workflow failures.
