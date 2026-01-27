# Accounts UI

Dioxus WASM UI bundle for Accounts (setup/admin + login).

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
