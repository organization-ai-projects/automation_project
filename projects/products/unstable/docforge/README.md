# docforge

Deterministic document editing engine with typed AST, replayable operations, and stable rendering output.

## Binaries

- `docforge_backend`: document engine and persistence.
- `docforge_ui`: UI entrypoint crate.

## UI Runtime

- `docforge_ui` uses a Dioxus launch path for `wasm32`.
- Native execution remains a minimal entrypoint for local workflows.
