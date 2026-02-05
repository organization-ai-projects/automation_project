# ci_dev.yml Documentation

## Purpose

This workflow handles CI tasks for the `dev` branch. It runs the centralized CI steps defined in `ci_reusable.yml`.

## Triggers

- **Push**: Triggered on pushes to the `dev` branch.
- **Pull Request**: Triggered on pull requests targeting the `dev` branch.

## Steps

1. **Run CI**:
   - Executes the reusable workflow [ci_reusable.yml](ci_reusable.md).

## Related Files

- [ci_reusable.yml](ci_reusable.md)
- [ci_main.yml](ci_main.md)
