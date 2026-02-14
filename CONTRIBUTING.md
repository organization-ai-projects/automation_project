# Contributing

## Overview

Contributions are welcome. Keep changes focused, follow existing structure, and link your work to an issue when possible.

For detailed workflow documentation, see the [scripts TOC](scripts/TOC.md).

---

## Getting Started

1. Fork the repository (external contributors) or clone directly (team members).
2. Create a branch from `dev` following the naming convention below.
3. Make your changes with clear, focused commits.
4. Open a pull request to `dev`.

---

## Branch Naming

Use descriptive branch names with a type prefix:

```text
<type>/<short-description>
```

**Types** (the following prefixes are accepted, including aliases and variants):

- `feature/` or `feat/` – New feature
- `fix/` – Bug fix
- `fixture/` – Test fixtures or data
- `doc/` or `docs/` – Documentation changes
- `refactor/` – Code refactoring
- `test/` or `tests/` – Adding or updating tests
- `chore/` – Maintenance tasks

**Examples**:

- `feat/user-authentication`
- `feature/user-dashboard`
- `fix/json-parser-panic`
- `fixture/test-data`
- `doc/update-api-docs`
- `docs/add-examples`
- `refactor/simplify-error-handling`
- `test/add-integration-tests`
- `tests/unit-coverage`
- `chore/update-dependencies`

**Note**: Branch naming is enforced by the `create_branch.sh` script. Invalid branch names will be rejected with a clear error message.

**Source of truth**: `documentation/technical_documentation/branch_naming_convention.md`

---

## Commit Guidelines

### Commit Message Format (Enforced)

All commit messages **must** follow the conventional commit format:

```text
<type>(<scope>): <summary>
```

or

```text
<type>: <summary>
```

**Required Types**:

- `feature`, `feat` – New feature
- `fix` – Bug fix
- `fixture` – Test data or fixtures
- `doc`, `docs` – Documentation changes
- `refactor` – Code refactoring
- `test`, `tests` – Adding or updating tests
- `chore` – Maintenance tasks

**Examples**:

- `feat(auth): add user authentication`
- `feat(ci,scripts): add workflows and sync script`
- `fix: resolve null pointer exception`
- `docs(readme): update installation instructions`
- `refactor(api): simplify error handling`
- `test: add unit tests for validator`
- `chore: update dependencies`

**Scope** (optional): Component or module affected (e.g., `auth`, `api`, `ci`)

**Summary**: Clear, concise description of the change

**Enforcement**:

- The `add_commit_push.sh` script validates commit messages
- Git commit hooks validate commit messages (when installed via `scripts/automation/git_hooks/install_hooks.sh`)
- Non-conforming messages are rejected with clear error messages
- Bypass only for emergencies:
  - Use `--no-verify` flag with `add_commit_push.sh`
  - Use `SKIP_COMMIT_VALIDATION=1 git commit -m "message"` with git directly

### Additional Guidelines

- Keep commits small and focused on a single change.
- Reference issues when applicable: `fix: resolve panic in parser (#42)`

See [Git scripts TOC](scripts/versioning/file_versioning/git/TOC.md) for details.

---

## Pull Request Guidelines

### Before Opening a PR

1. Rebase your branch on the latest `dev`:

   ```bash
   git fetch origin
   git rebase origin/dev
   ```

2. Run tests locally:

   ```bash
   cargo test --workspace
   ```

3. Check formatting and lints:

   ```bash
   cargo fmt --check
   cargo clippy --workspace
   pnpm run lint-md  # Markdown linting (requires pnpm install)
   ```

### Creating a PR

The `create_pr.sh` script automates PR creation and **automatically runs tests** before creating the PR to ensure code quality:

```bash
bash scripts/versioning/file_versioning/orchestrators/read/create_pr.sh
```

**Test enforcement:**

- By default, `create_pr.sh` runs `cargo test --workspace` before creating the PR
- If tests fail, the PR will not be created
- To skip tests (not recommended), use the `--skip-tests` flag:

  ```bash
  bash scripts/versioning/file_versioning/orchestrators/read/create_pr.sh --skip-tests
  ```

- Skipping tests will display a warning reminder to ensure proper testing before merging

**Additional options:**

- `--base <branch>`: Specify the base branch (default: `dev`)
- `--title <title>`: Custom PR title
- `--body <body>`: Custom PR description
- `--draft`: Create as draft PR

### PR Requirements

- **Title**: Use the same convention as branch names (`feat:`, `fix:`, etc.)
- **Description**: Explain what and why, link related issues
- **Size**: Keep PRs focused; split large changes into smaller PRs
- **Tests**: Include tests for new functionality

See [Versioning TOC](scripts/versioning/file_versioning/TOC.md) for details.

---

## Code Review Process

1. All PRs require at least one approval before merging.
2. Address reviewer feedback promptly.
3. Re-request review after making changes.
4. Resolve all conversations before merging.

### Review Expectations

- Reviews typically happen within 1-2 business days.
- Be respectful and constructive in feedback.
- Focus on correctness, clarity, and maintainability.

---

## Testing Requirements

- All new features must include tests.
- Bug fixes should include a regression test when possible.
- Run the full test suite before submitting:

  ```bash
  cargo test --workspace
  ```

- Ensure CI passes on your PR.

### Test Import Style

**Use explicit imports in test modules.** Avoid `use super::*` in favor of explicit `use crate::` or `use super::` imports for specific items.

**Good:**

```rust
#[cfg(test)]
mod tests {
    use crate::MyStruct;
    use crate::MyEnum;
    // or
    use super::MyStruct;
    use super::MyEnum;

    #[test]
    fn test_something() {
        let s = MyStruct::new();
        // ...
    }
}
```

**Avoid:**

```rust
#[cfg(test)]
mod tests {
    use super::*;  // Avoid this

    #[test]
    fn test_something() {
        // ...
    }
}
```

**Rationale:** Explicit imports improve code clarity, make dependencies obvious, and reduce ambiguity during code review.

---

## Code Quality

- Prefer explicit error handling over panics.
- Keep documentation up to date with code changes.
- Avoid breaking public APIs without a clear migration path.
- Follow existing code style and patterns.
- Use `cargo fmt` for formatting and `cargo clippy` for lints.
- Use `pnpm run lint-md` for markdown linting and `pnpm run lint-md-fix` for auto-fixing markdown issues.

---

## Questions?

If you have questions about contributing, open an issue or reach out to the maintainers.
