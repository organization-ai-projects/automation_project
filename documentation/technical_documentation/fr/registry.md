# Registre

- [Retour à TOC Technique](TOC.md)

## Purpose

The registry (`.automation_project/registry.json`) is the **source of truth** for:

- the list of products
- UI bundle locations
- backend identities
- versions and schema compatibility

It is **explicit**, versioned, and updated by the Engine when it reads each product's `metadata.ron`.
The registry is not handwritten configuration. It is a compiled, normalized view of product metadata, produced by the Engine.

## Relationship to `metadata.ron`

- Each product ships a `metadata.ron` file.
- Engine loads `metadata.ron`, validates it, and writes/updates the registry.
- Central UI reads the registry to display products and UIs.

This means **discovery is always metadata-driven**, not filesystem-scanned.

## ID Format (ProtocolId)

All identifiers stored in the registry are **ProtocolId hex strings** (32 hex chars):

- product IDs
- UI entrypoint IDs
- domain IDs
- backend IDs (where applicable)

This does **not** change the registry structure, only the **format** of the values.

## Registre Fields (high level)

The registry includes (at minimum):

- product list
- UI bundle paths
- backend identities
- schema/version metadata

Exact shape can evolve, but IDs remain ProtocolId hex strings.

## Rules

- Do not infer products by scanning the workspace.
- Update the registry only through the Engine's metadata loading.
- Registre is authoritative for Central UI.

## Related Docs

- [Métadonnées](metadata.md)
- [Architecture](ARCHITECTURE.md)
- [Produits et Composants de l'Espace de Travail](projects/projects_products.md)
