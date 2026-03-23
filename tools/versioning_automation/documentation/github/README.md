# GitHub Automation Documentation

This directory now keeps GitHub automation documentation.
The automation logic is implemented in Rust in `tools/versioning_automation` and executed via `versioning_automation`.

## Directory Structure

```text
github/
├── README.md
├── TOC.md
└── i18n/
```

## Canonical Entrypoints (Rust)

- `versioning_automation pr generate-description ...`
- `versioning_automation pr refresh-validation ...`
- `versioning_automation pr auto-add-closes ...`
- `versioning_automation pr directive-conflict-guard ...`
- `versioning_automation issue auto-link ...`
- `versioning_automation issue create ...`
- `versioning_automation issue <read|update|close|reopen|delete> ...`
- `versioning_automation issue done-status ...`
- `versioning_automation issue reopen-on-dev ...`
- `versioning_automation issue neutralize ...`
- `versioning_automation issue reevaluate ...`
- `versioning_automation issue parent-guard ...`
- `versioning_automation issue closure-hygiene ...`

## Rust Regression Suites

- `cargo test -q -p versioning_automation pr::tests::generate_description`
- `cargo test -q -p versioning_automation pr::tests::refresh_validation`
- `cargo test -q -p versioning_automation`

Shell-level cross-workflow checks are under `scripts/automation/tests/`.

## Notes

- GitHub Actions workflows call `target/debug/versioning_automation ...` directly.
- No shell runtime entrypoint remains for GitHub automation.
- Troubleshooting: `.github/documentation/pr_generator_troubleshooting.md`.
