# Metadata

- [Back to Technical TOC](TOC.md)

## 1 `metadata.ron` File

Each project includes a `metadata.ron` file that describes its capabilities and entry points. This file is used by the global registry and the central UI to discover and display projects.

The `metadata.ron` file is purely declarative. It does not trigger any actions by itself.

### 1.1 Example of a `metadata.ron` File

```ron
(
  schema_version: 1,
  generated_at: "2026-01-04T12:00:00Z",
  id: "dev_forge_app",
  name: "Dev Forge Desktop",
  kind: "product", // product | library
  version: "0.1.0",

  entrypoints: (
    ui: [
      (id: "admin", title: "Admin", role: "admin"),
      (id: "user",  title: "User",  role: "user"),
    ],
  ),

  capabilities: [
    "lint",
    "tests",
    "doc",
    "workflow_runner",
  ],

  domains: [
    (id: "projects",  desc: "multi-project management"),
    (id: "workflows", desc: "orchestration and runners"),
    (id: "logs",      desc: "events & observability"),
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
> - `id`: Stable, unique, and machine-friendly.
> - `name`: Human-readable and intended for user interfaces.
>   Note:
> - The `entrypoints` field may include other types of entry points in the future, beyond `ui`.
> - Similarly, the `ai_hints` field is extensible, but there is no guarantee of support for new keys.
