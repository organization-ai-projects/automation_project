# Repo Contract Enforcer Diagnostics (VSCode Extension)

Local VSCode extension to surface team-rule diagnostics (repo_contract_enforcer) directly inline in editors, similar to clippy squiggles.

## What it does

- Runs `repo_contract_enforcer_backend` over JSON-lines IPC.
- Creates VSCode diagnostics from violations.
- Updates diagnostics on startup, save, edits (debounced), and Rust/TOML file events.
- Cancels superseded runs to avoid overlapping checks.
- Exposes explicit status in VSCode status bar (running/superseded/errors/count).

## Install locally

From this folder:

```bash
pnpm run vsix
```

This command runs watch mode: it packages + reinstalls immediately, then auto-repeats on every extension file change.

One-shot (without watch):

```bash
pnpm run vsix:reinstall
```

Release-like local smoke pipeline (tests + checks + package):

```bash
pnpm run vsix:smoke
```

Release-like local smoke + install generated VSIX:

```bash
pnpm run vsix:release-local
```

If you want automatic reinstall while editing the extension:

```bash
pnpm run vsix:watch
```

It watches key files and repackages/reinstalls on every change.

By default, reinstall does not reload VS Code automatically (to avoid opening an
extra window in some setups).
To enable auto-reload:

```bash
RELOAD_WINDOW_AFTER_INSTALL=true pnpm run vsix:reinstall
```

## Settings

- `repoContractEnforcer.mode`: `auto` | `strict` | `relaxed`
- `repoContractEnforcer.runOnSave`: `true|false`
- `repoContractEnforcer.runOnChange`: `true|false` (debounced edit-triggered checks)
- `repoContractEnforcer.runOnFileEvents`: `true|false` (create/change/delete/rename)
- `repoContractEnforcer.debounceMs`: debounce delay in milliseconds (`50..5000`)
- `repoContractEnforcer.command`: command used to run UI (default: `cargo`)

## Manual command

- `Repo Contract Enforcer: Run Check`
