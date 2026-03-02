# Repo Contract Enforcer Diagnostics (VSCode Extension)

Local VSCode extension to surface team-rule diagnostics (repo_contract_enforcer) directly inline in editors, similar to clippy squiggles.

## What it does

- Runs `repo_contract_enforcer_ui` in JSON mode.
- Creates VSCode diagnostics from violations.
- Updates diagnostics on save (`.rs`, `.toml`) and on startup.

## Install locally

1. Open VSCode command palette.
2. Run `Extensions: Install from VSIX...` after packaging with `vsce`, or use `Developer: Install Extension from Location...` and select this folder if available in your VSCode build.

## Settings

- `repoContractEnforcer.mode`: `auto` | `strict` | `relaxed`
- `repoContractEnforcer.runOnSave`: `true|false`
- `repoContractEnforcer.command`: command used to run UI (default: `cargo`)

## Manual command

- `Repo Contract Enforcer: Run Check`
