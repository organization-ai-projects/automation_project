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

## Related Files

- [ci_main.yml](ci_main.md)
- [ci_dev.yml](ci_dev.md)
