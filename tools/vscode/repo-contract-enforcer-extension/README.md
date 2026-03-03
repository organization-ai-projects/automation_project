# Repo Contract Enforcer Diagnostics (VSCode Extension)

Local VSCode extension to surface team-rule diagnostics (repo_contract_enforcer) directly inline in editors, similar to clippy squiggles.

## What it does

- Runs `repo_contract_enforcer_backend` over JSON-lines IPC.
- Creates VSCode diagnostics from violations.
- Updates diagnostics on startup, save, edits (debounced), and Rust/TOML file events.
- Cancels superseded runs to avoid overlapping checks.
- Exposes explicit status in VSCode status bar (running/superseded/errors/count).

## Install locally

1. Open VSCode command palette.
2. Run `Extensions: Install from VSIX...` after packaging with `vsce`, or use `Developer: Install Extension from Location...` and select this folder if available in your VSCode build.

## Settings

- `repoContractEnforcer.mode`: `auto` | `strict` | `relaxed`
- `repoContractEnforcer.runOnSave`: `true|false`
- `repoContractEnforcer.runOnChange`: `true|false` (debounced edit-triggered checks)
- `repoContractEnforcer.runOnFileEvents`: `true|false` (create/change/delete/rename)
- `repoContractEnforcer.debounceMs`: debounce delay in milliseconds (`50..5000`)
- `repoContractEnforcer.command`: command used to run UI (default: `cargo`)

## Manual command

- `Repo Contract Enforcer: Run Check`
