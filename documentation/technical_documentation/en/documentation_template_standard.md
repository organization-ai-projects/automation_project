# Documentation Template Standard

This file is the canonical source of truth for documentation template structure
in this repository.

## Goal

Define consistent roles for `README.md` and `TOC.md`, and list required vs
optional sections so new documentation can be added without ambiguity.

## File Roles

### `README.md`

- Purpose: explain a directory/module as an entry point for humans.
- Focus: context, usage, and conventions.
- Should not duplicate full file indexes already maintained in `TOC.md`.

Required sections:

- `Purpose` or `Role`
- `Scope`
- `Key Components` (or equivalent summary)
- `Navigation` (or links to relevant TOC)

Optional sections:

- `Conventions`
- `Usage`
- `Troubleshooting`
- `References`

### `TOC.md`

- Purpose: provide a navigable index of documentation files for a directory.
- Focus: links + one-line descriptions.
- Should not duplicate explanatory narrative already in `README.md`.

Required sections:

- `Documentation` (or `Documentation Files`) with links and short descriptions
- `Navigation` (back-link to parent index)

Optional sections:

- `Related Documentation`
- `Related Governance Docs`
- `Templates`
- `Workflows`

## Canonical Location Rule

- This file (`documentation/technical_documentation/en/documentation_template_standard.md`)
  is the single source of truth for template structure.
- Other docs may summarize or reference this standard, but must not redefine
  conflicting rules.

## Adoption Rule

- New documentation zones should include both:
  - a `README.md` (context and usage),
  - a `TOC.md` (index and navigation),
  unless the zone contains only one file and no sub-structure.

- Existing zones should converge to this structure incrementally when touched.
