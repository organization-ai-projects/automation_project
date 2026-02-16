# Documentation EN/FR Structure and Migration Policy

## Goal

Define deterministic, maintainable rules for bilingual documentation across the repository.

## Canonical Source

- English (`en`) is the canonical source of truth.
- French (`fr`) content is a translation of canonical English content.
- Translation must preserve technical meaning; no architecture/content refactor is allowed during translation.

## Folder and File Structure Rules

### Root-level documentation

- Keep canonical English files at root:
  - `README.md`
  - `CONTRIBUTING.md`
- Add French equivalents with `.fr.md` suffix:
  - `README.fr.md`
  - `CONTRIBUTING.fr.md`

### Documentation trees

For each documentation tree (`documentation/`, `projects/**/documentation/`, `scripts/**`, `tools/**`):

- Use parallel language trees:
  - `en/` for canonical English docs
  - `fr/` for French translations
- Keep navigation symmetry:
  - English TOCs link to English files
  - French TOCs link to French files

## Link Safety Rules

- Keep all relative links valid after migration.
- Prefer language-consistent links (`en -> en`, `fr -> fr`).
- Shared assets must not be duplicated per language.

## Shared Assets Rules

- Keep a single source for assets per documentation zone.
- Example for project docs:
  - `documentation/assets/**`
- Example for crate-level docs:
  - `projects/**/documentation/assets/**`

## Translation Header Rule

Each French file must include this header line at the top:

`This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.`

## Non-goals

- No code refactor in this policy.
- No translation of source code comments or rustdoc in `.rs` files.
- No duplication of assets for each language.

## Migration Phasing

Recommended execution order:

1. Define and approve structure/policy (this document).
2. Migrate root docs and documentation entrypoints.
3. Expand to per-crate docs and scripts/tools docs.
4. Enforce link validation in CI.
