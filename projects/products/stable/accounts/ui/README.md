# Accounts UI Documentation

This directory contains the Dioxus WASM UI bundle for Accounts (setup/admin + login).

## Role in the Project

This UI is responsible for providing the web interface for account setup, login, and user management. It communicates with the accounts backend through the central_ui proxy.

It interacts mainly with:

- Central UI - For serving the UI bundle
- Accounts backend - For account operations
- Engine - Through central_ui proxy

## Directory Structure

```
ui/
├── README.md           # This file
├── Cargo.toml          # Package configuration
├── ui_manifest.ron     # UI metadata
└── src/               # Source code
    └── main.rs
```

## Files

- `README.md`: This file.
- `Cargo.toml`: Package configuration.
- `documentation/`: Additional documentation.
- `src/`: Source code.


## Build (WASM bundle)

```bash
# From workspace root
cargo install dioxus-cli

# Build UI bundle
scripts/automation/build_accounts_ui.sh
```

The bundle should include:
- `ui_dist/ui.wasm`
- `ui_dist/ui.js`
- `ui_dist/index.html`
- `ui_dist/ui_manifest.ron`

`ui_manifest.ron` is sourced from `projects/products/accounts/ui/ui_manifest.ron`.

Central UI serves the `ui_dist/` folder at runtime.
