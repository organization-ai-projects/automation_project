# Contributing

**Version française** : [CONTRIBUTING.fr.md](CONTRIBUTING.fr.md)

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

**Types**:

- `feat/` – New feature
- `fix/` – Bug fix
- `doc/` – Documentation changes
- `refactor/` – Code refactoring
- `test/` – Adding or updating tests
- `chore/` – Maintenance tasks

**Examples**:

- `feat/user-authentication`
- `fix/json-parser-panic`
- `doc/update-api-docs`

---

## Commit Guidelines

- Keep commits small and focused on a single change.
- Use clear, descriptive commit messages.
- Reference issues when applicable: `fix: Resolve panic in parser (#42)`

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
   ```

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

---

## Code Quality

- Prefer explicit error handling over panics.
- Keep documentation up to date with code changes.
- Avoid breaking public APIs without a clear migration path.
- Follow existing code style and patterns.
- Use `cargo fmt` for formatting and `cargo clippy` for lints.

---

## Questions?

If you have questions about contributing, open an issue or reach out to the maintainers.
