# Multi-Language Documentation Structure

This document describes the multi-language documentation structure implemented in this repository.

## Overview

The repository now supports documentation in multiple languages:
- **English (en)**: Canonical source of truth
- **French (fr)**: Translation

## Structure by Documentation Type

### 1. Root-Level Documentation

Root-level documentation files have language variants with `.fr.md` suffix:

```
README.md           # English (canonical)
README.fr.md        # French translation
CONTRIBUTING.md     # English (canonical)
CONTRIBUTING.fr.md  # French translation
```

Each file includes a link to its language variant at the top.

### 2. Main Technical Documentation (`/documentation/`)

The main documentation folder uses a hierarchical structure with language selectors:

```
documentation/
├── TOC.md              # English index
├── TOC.fr.md           # French index
└── technical_documentation/
    ├── assets/         # Shared assets (not duplicated per language)
    │   └── architecture_bootstrap.png
    ├── en/             # English documentation (canonical)
    │   ├── TOC.md
    │   ├── ARCHITECTURE.md
    │   ├── documentation.md
    │   ├── metadata.md
    │   ├── registry.md
    │   ├── system_processes.md
    │   └── projects/
    │       ├── TOC.md
    │       ├── projects_libraries.md
    │       └── projects_products.md
    └── fr/             # French translations
        ├── TOC.md
        ├── ARCHITECTURE.md
        ├── documentation.md
        ├── metadata.md
        ├── registry.md
        ├── system_processes.md
        └── projects/
            ├── TOC.md
            ├── projects_libraries.md
            └── projects_products.md
```

**Key points:**
- Assets are shared at `technical_documentation/assets/` level
- No asset duplication per language
- Links to assets use `../assets/` from within language folders

### 3. Library Documentation (`/projects/libraries/*/documentation/`)

Each library's documentation follows the same pattern:

```
projects/libraries/<library_name>/
├── README.md
├── README.fr.md
└── documentation/
    ├── en/
    │   ├── TOC.md
    │   └── *.md (documentation files)
    └── fr/
        ├── TOC.md
        └── *.md (translations)
```

**Example**: `common_json` library:
```
projects/libraries/common_json/
├── README.md
├── README.fr.md
└── documentation/
    ├── en/
    │   ├── TOC.md
    │   ├── access.md
    │   ├── deserialize.md
    │   ├── error.md
    │   ├── json_array_builder.md
    │   ├── json_object_builder.md
    │   ├── macros.md
    │   ├── merge.md
    │   ├── serialize.md
    │   └── value.md
    └── fr/
        ├── TOC.md
        ├── access.md
        ├── deserialize.md
        ├── error.md
        ├── json_array_builder.md
        ├── json_object_builder.md
        ├── macros.md
        ├── merge.md
        ├── serialize.md
        └── value.md
```

### 4. Product Documentation (`/projects/products/*/documentation/`)

Products follow the same structure as libraries:

```
projects/products/<product_name>/
├── README.md
├── README.fr.md
└── documentation/
    ├── en/
    │   ├── TOC.md
    │   └── *.md
    └── fr/
        ├── TOC.md
        └── *.md
```

### 5. Scripts Documentation (`/scripts/`)

Scripts use a flat structure with `.fr.md` suffix for French variants:

```
scripts/
├── README.md
├── README.fr.md
├── TOC.md
├── TOC.fr.md
├── automation/
│   ├── README.md
│   ├── README.fr.md
│   ├── TOC.md
│   └── TOC.fr.md
└── versioning/
    ├── README.md
    ├── README.fr.md
    ├── TOC.md
    └── TOC.fr.md
```

**Rationale**: Scripts documentation is mostly README files, so a flat structure with language suffixes is more practical than subdirectories.

### 6. Tools Documentation (`/tools/`)

Tools follow the same flat structure as scripts:

```
tools/bot_ci_harness/
├── README.md
└── README.fr.md
```

## Navigation Rules

### Cross-References

1. **English documents** link to other English documents
2. **French documents** link to other French documents
3. Language selectors at the top of major documents allow switching between languages

### Link Patterns

- Root docs → Documentation: `documentation/TOC.md` (EN) or `documentation/TOC.fr.md` (FR)
- Documentation → Root: `../README.md` (EN) or `../README.fr.md` (FR)
- Within doc folders: Relative links within same language folder
- Assets: Always use relative path to shared `assets/` folder

### Examples

From `documentation/technical_documentation/en/ARCHITECTURE.md`:
```markdown
![Bootstrap](../assets/architecture_bootstrap.png)
[Back to Technical TOC](TOC.md)
```

From `projects/libraries/common_json/documentation/en/TOC.md`:
```markdown
[Back to common_json README](../../README.md)
```

From `projects/libraries/common_json/documentation/fr/TOC.md`:
```markdown
[Retour à common_json README](../../README.fr.md)
```

## Adding New Documentation

### For a new document in an existing location:

1. Create the English version in the `en/` folder
2. Create the French translation in the `fr/` folder
3. Update the corresponding TOC.md file in both languages

### For a new library/product:

1. Create `README.md` and `README.fr.md` in the root
2. Create `documentation/en/` and `documentation/fr/` folders
3. Add `TOC.md` in both language folders
4. Add documentation files following the pattern above

### For scripts:

1. Create `README.md` and `README.fr.md` in the same directory
2. Create `TOC.md` and `TOC.fr.md` if needed
3. Ensure links in French files point to French variants

## Translation Guidelines

1. **English is canonical**: All content originates in English
2. **No content changes in translation**: French versions should translate, not rewrite
3. **Maintain structure**: Keep the same sections, headers, and organization
4. **Update links**: Ensure French docs link to French variants
5. **Don't duplicate assets**: Use shared asset folders

## Summary

- **200+ documentation files** now support both English and French
- **Zero asset duplication** - all images/assets are shared
- **Consistent structure** across all documentation types
- **Working cross-references** between all documents
- **Easy to extend** for additional languages in the future
