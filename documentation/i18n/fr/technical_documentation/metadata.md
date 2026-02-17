# Metadonnees

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

- [Retour au TOC technique](TOC.md)

## 1 Fichier `metadata.ron`

Chaque projet inclut un fichier `metadata.ron` qui decrit ses capacites et points d'entree. Ce fichier est utilise par le registre global et l'UI centrale pour decouvrir et afficher les projets. Voir [Registry](registry.md) pour la structure du registre et le format des IDs.

Le fichier `metadata.ron` est purement declaratif. Il ne declenche aucune action a lui seul.

### 1.1 Exemple de fichier `metadata.ron`

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

> Convention :
>
> - `id` : stable, unique et machine-friendly (hex ProtocolId).
> - `generated_at` : timestamp UNIX en millisecondes (`u64`).
> - `name` : lisible humainement et destine aux interfaces utilisateur.
> - `entrypoints.ui[*].id` : ProtocolId (hex).
> - `domains[*].id` : ProtocolId (hex).
>   Note :
> - Le champ `entrypoints` peut inclure d'autres types de points d'entree dans le futur, au-dela de `ui`.
> - Le champ `ai_hints` est extensible, mais il n'y a pas de garantie de support pour les nouvelles cles.
