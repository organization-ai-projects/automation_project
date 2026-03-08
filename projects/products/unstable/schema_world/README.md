# schema_world

Schema-driven data world with deterministic storage, canonical encoding, migrations, diffs, and snapshot hashes.

## Binaries

- `schema_world_backend`: deterministic engine + IPC server.
- `schema_world_ui`: thin controller/UI over backend IPC.

## Determinism Contract

- Same logical data maps to same canonical bytes.
- Snapshot hash is SHA-256 over canonical snapshot bytes.
- Storage iteration order and diff ordering are deterministic.
