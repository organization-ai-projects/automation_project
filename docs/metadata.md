# Metadata

## 1 Fichier `metadata.ron`

Chaque projet inclut un fichier `metadata.ron` qui décrit ses capacités et ses points d’entrée. Ce fichier est utilisé par le registry global et l’UI centrale pour découvrir et afficher les projets.

Le fichier `metadata.ron` est purement déclaratif. Il ne déclenche aucune action par lui-même.

### 1.1 Exemple de fichier `metadata.ron`

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
    (id: "projects",  desc: "gestion multi-projets"),
    (id: "workflows", desc: "orchestration et runners"),
    (id: "logs",      desc: "événements & observabilité"),
  ],

  ai_hints: (
    primary_language: "rust",
    important_paths: ["src/", "README.md", "ARCHITECTURE.md"],
    config_files: ["Cargo.toml", "project.toml"],
  ),
)
```

> Convention :
>
> - `id` : Stable, unique, et adapté aux machines.
> - `name` : Lisible par un humain et destiné aux interfaces utilisateur.
> Note :
> - Le champ `entrypoints` pourra inclure d'autres types de points d'entrée à l'avenir, au-delà de `ui`.
> - De même, le champ `ai_hints` est extensible, mais sans garantie de support pour les nouvelles clés.
