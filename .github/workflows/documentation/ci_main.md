# ci_main.yml Documentation

## Purpose

This workflow handles CI tasks for the `main` branch. It ensures that only allowed branches (`dev` or `hotfix/*`) can be merged into `main` and runs the centralized CI steps defined in `ci_reusable.yml`.

## Triggers

- **Push**: Triggered on pushes to the `main` branch.
- **Pull Request**: Triggered on pull requests targeting the `main` branch.

## Steps

1. **Gate PR Sources**:
   - Verifies that only `dev` or `hotfix/*` branches can merge into `main`.
2. **Run CI**:
   - Executes the reusable workflow [ci_reusable.yml](ci_reusable.md).

## Related Files

- [ci_reusable.yml](ci_reusable.md)
- [ci_dev.yml](ci_dev.md)
