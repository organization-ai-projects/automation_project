# City Builder UI

Dioxus UI bundle (WASM target) with a native CLI entrypoint for local command execution.

## Runtime Modes

- `wasm32`: launches Dioxus UI (`src/web_app.rs`)
- non-wasm: runs CLI flow (`src/main.rs`) and invokes backend binary

## API Base

The WASM UI sends JSON commands to:

- `${CITY_BUILDER_UI_API_BASE}/run`
- `${CITY_BUILDER_UI_API_BASE}/replay`
- `${CITY_BUILDER_UI_API_BASE}/snapshot`
- `${CITY_BUILDER_UI_API_BASE}/validate`

Default:

- `CITY_BUILDER_UI_API_BASE=/api/city_builder`

## Bundle Metadata

`ui_manifest.ron` is the UI bundle descriptor consumed by central UI.
