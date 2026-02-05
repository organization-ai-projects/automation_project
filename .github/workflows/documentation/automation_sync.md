# automation_sync.yml Documentation

This workflow syncs the `main` branch into `dev` after merges to keep `dev` up to date.

## Purpose

- Keeps `dev` aligned with `main` after merge.

## Triggers

- Triggered on PRs merged into `main`.

## Steps

1. **Checkout Code**: Checks out the repository code.
2. **Generate GitHub App token**: Creates a token to authenticate the sync.
3. **Run sync script**: Runs the script that creates the `main` → `dev` sync PR and waits for mergeability.

## Contribution

This workflow is event-driven and does not provide a manual trigger.
