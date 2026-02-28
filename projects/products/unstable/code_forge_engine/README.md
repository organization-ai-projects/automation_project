# code_forge_engine

A workspace product that generates Rust workspace scaffolding from declarative contracts.

## Crates

### `backend` (`code_forge_engine_backend`)
The headless code-generation engine. Reads a contract (JSON/TOML), validates it, renders files, and computes a deterministic manifest hash. Communicates exclusively via a JSON-lines IPC protocol on **stdout**; all diagnostics go to **stderr**.

### `ui` (`code_forge_engine_ui`)
The command-line front-end. Spawns the backend as a child process and drives it through the IPC protocol. Provides contract loading, preview, generation, and report screens.

### `tooling` (`code_forge_engine_tooling`)
Developer utilities: golden-file management and byte-stability / structural validation of generated output.

## IPC Protocol

The backend speaks a line-delimited JSON protocol. Each line sent to **stdin** is a `Message` object wrapping a `Request`; each line emitted on **stdout** is a `Response`. All variants use `snake_case` tags.

**Request variants** (`type` field):
- `load_contract` – `{ "type": "load_contract", "path": "<path>" }`
- `validate_contract` – `{ "type": "validate_contract" }`
- `preview_layout` – `{ "type": "preview_layout" }`
- `generate` – `{ "type": "generate", "out_dir": "<dir>", "mode": "<mode>" }`
- `get_manifest` – `{ "type": "get_manifest" }`
- `shutdown` – `{ "type": "shutdown" }`

**Response variants** (`type` field):
- `ok`
- `error` – `{ "code": <u32>, "message": "…", "details": "…" }`
- `preview` – `{ "files": ["…"] }`
- `manifest` – `{ "manifest_json": "…", "manifest_hash": "…" }`
