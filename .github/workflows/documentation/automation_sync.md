# automation_sync.yml Documentation

This workflow syncs the `main` branch into `dev` after merges to keep `dev` up to date.

## Purpose

- Keeps `dev` aligned with `main` after merge.

## Triggers

- Triggered on a schedule or manual dispatch.

## Steps

1. **Checkout Code**: Checks out the repository code.
2. **Sync Automation**: Runs scripts to synchronize automation configurations.
3. **Validate Changes**: Validates that the synchronization was successful.

## Contribution

Contributors can manually trigger this workflow if they make changes to automation configurations.
