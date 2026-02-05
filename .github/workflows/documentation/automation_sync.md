# automation_sync.yml Documentation

This workflow syncs the `main` branch into `dev` after merges to keep `dev` up to date.

## Purpose

- Keeps `dev` aligned with `main` after merge.

## Triggers

- Triggered on a schedule or manual dispatch.

## Steps

1. **Checkout Code**: Checks out the repository code.
2. **Create sync PR**: Creates a PR that merges `main` into `dev`.
3. **Validate Changes**: Validates that the synchronization was successful.

## Contribution

Contributors can manually trigger this workflow if they need to sync `main` into `dev`.
