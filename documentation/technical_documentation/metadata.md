# Metadata

- [Back to Technical TOC](TOC.md)

## 1 `metadata.ron` File

Each project includes a `metadata.ron` file that describes its capabilities and entry points. This file is used by the global registry and the central UI to discover and display projects. See [Registry](registry.md) for registry structure and ID format.

The `metadata.ron` file is purely declarative. It does not trigger any actions by itself.

### 1.1 Example of a `metadata.ron` File

```ron
(
  schema_version: 1,
  generated_at: "2026-01-04T12:00:00Z",
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
> - `name`: Human-readable and intended for user interfaces.
> - `entrypoints.ui[*].id`: ProtocolId (hex).
> - `domains[*].id`: ProtocolId (hex).
>   Note:
> - The `entrypoints` field may include other types of entry points in the future, beyond `ui`.
> - Similarly, the `ai_hints` field is extensible, but there is no guarantee of support for new keys.
