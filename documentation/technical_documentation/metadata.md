# Metadata

- [Back to Technical TOC](TOC.md)

## 1 `metadata.ron` File

Each project includes a `metadata.ron` file that describes its capabilities and entry points. This file is used by the global registry and the central UI to discover and display projects. See [Registry](registry.md) for registry structure and ID format.

The `metadata.ron` file is purely declarative. It does not trigger any actions by itself.

## 1.2 Product Manifest Convention

The product manifest contract is split into:

- Root product manifest: `metadata.ron` (source of truth for product identity/capabilities/entrypoints)
- Crate deployable manifests:
  - `backend/backend_manifest.ron`
  - `ui/ui_manifest.ron`

The root `metadata.ron` remains authoritative for product-level identity. Crate manifests define deployable artifact metadata for packaging/runtime wiring.

### 1.3 `backend_manifest.ron` Contract

`backend/backend_manifest.ron` describes the deployable backend artifact.

```ron
(
  schema_version: 1,
  id: "example_backend",
  title: "Example Backend",
  entrypoint: "example_backend",
  kind: "backend_service",
  transport: "stdio_cli",
  version: "0.1.0",
)
```

Field semantics:

- `schema_version`: manifest schema version (`u64`)
- `id`: stable backend artifact id (string, machine-friendly)
- `title`: human-readable backend label
- `entrypoint`: executable command/binary name
- `kind`: fixed to `backend_service`
- `transport`: runtime transport contract (for example `stdio_cli`)
- `version`: artifact version

### 1.4 `ui_manifest.ron` Contract

`ui/ui_manifest.ron` describes the UI bundle artifact.

```ron
(
  schema_version: 1,
  id: "example_ui",
  title: "Example UI",
  entrypoint: "index.html",
  kind: "ui_bundle",
  version: "0.1.0",
)
```

### 1.5 Migration Strategy

Rollout is progressive to keep compatibility:

1. Introduce manifests on unstable products first (`metadata.ron`, `backend_manifest.ron`, `ui_manifest.ron` when UI exists).
2. Enforce missing-manifest checks as warnings on unstable products.
3. Backfill stable products with crate manifests.
4. Enforce missing-manifest checks as blocking errors in strict mode for stable products.
5. Keep unstable/relaxed checks as warnings to allow progressive adoption.

### 1.1 Example of a `metadata.ron` File

```ron
(
  schema_version: 1,
  generated_at: 1767528000000,
  id: "4fb1f3633c504d8d82accf829a854ea4",
  name: "Dev Forge Desktop",
  kind: "product", // product | library
  version: "0.1.0",

  entrypoints: (
    ui: [
      (id: "c0a8010f4d8f4fd4b4b4f3e2a9a1b7c1", title: "Admin", role: "admin"),
      (id: "d1f0a9b24c3d4e9fb2a8b0a1c2d3e4f5", title: "User",  role: "user"),
    ],
  ),

  capabilities: [
    "lint",
    "tests",
    "doc",
    "workflow_runner",
  ],

  domains: [
    (id: "0f1e2d3c4b5a69788796a5b4c3d2e1f0", desc: "multi-project management"),
    (id: "11223344556677889900aabbccddeeff", desc: "orchestration and runners"),
    (id: "ffeeddccbbaa00998877665544332211", desc: "events & observability"),
  ],

  ai_hints: (
    primary_language: "rust",
    important_paths: ["src/", "README.md", "ARCHITECTURE.md"],
    config_files: ["Cargo.toml", "project.toml"],
  ),
)
```

> Convention:
>
> - `id`: Stable, unique, and machine-friendly (hex ProtocolId).
> - `generated_at`: UNIX timestamp in milliseconds (`u64`).
> - `name`: Human-readable and intended for user interfaces.
> - `entrypoints.ui[*].id`: ProtocolId (hex).
> - `domains[*].id`: ProtocolId (hex).
>   Note:
> - The `entrypoints` field may include other types of entry points in the future, beyond `ui`.
> - Similarly, the `ai_hints` field is extensible, but there is no guarantee of support for new keys.
