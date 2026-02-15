# ci_reusable.yml Documentation

## Purpose

This reusable workflow centralizes common CI steps such as linting, formatting, and testing. It is used by other workflows like `ci_main.yml` and `ci_dev.yml`.

## Triggers

- **Workflow Call**: Triggered by other workflows using `workflow_call`.

## Steps

1. **Checkout**:
   - Checks out the repository code.
2. **Install Rust**:
   - Installs the repository toolchain policy defined in `rust-toolchain.toml`.
   - Ensures `rustfmt` and `clippy` are available.
3. **Cache Cargo**:
   - Caches dependencies to speed up builds.
4. **Run Checks**:
   - Runs `cargo fmt` and `cargo clippy` with locked dependency resolution.
5. **Run Tests**:
   - Executes `cargo test` for the entire workspace with locked dependency resolution.

## Policy Decisions

### Job Structure Decision

- Current policy keeps `fmt`, `clippy`, and `test` in a single job.
- Rationale:
  - preserves simple ordering and failure interpretation during stabilization,
  - avoids extra orchestration complexity while selective checks logic is still evolving.
- Revisit trigger:
  - splitting into parallel jobs can be re-evaluated once CI signal quality and flaky-test rate are stable.

### Workflow-Only Change Policy

- Changes limited to `.github/workflows/*` are intentionally treated as non-Rust changes in `ci_reusable.yml`.
- Expected validation path for workflow-only changes:
  - workflow review in pull request,
  - at least one manual workflow run (`workflow_dispatch`) when behavior is materially changed.

## Related Files

- [ci_main.yml](ci_main.md)
- [ci_dev.yml](ci_dev.md)
