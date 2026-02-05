# automation_rustfmt.yml Documentation

This workflow automates code formatting checks to ensure consistency across the codebase. It runs `rustfmt` on the code and reports any formatting issues.

## Purpose

- Enforces consistent code formatting.
- Prevents unformatted code from being merged.

## Triggers

- Triggered on pull requests and pushes to specific branches.

## Steps

1. **Checkout Code**: Checks out the repository code.
2. **Run rustfmt**: Executes the `rustfmt` tool to check for formatting issues.
3. **Report Issues**: Reports any formatting issues found.

## Contribution

Contributors should ensure their code is formatted before pushing changes to avoid workflow failures.
