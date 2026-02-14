# Documentation Ownership Map

This map defines ownership responsibilities for major documentation zones.

## Goal

Reduce orphaned documentation and make maintenance expectations explicit.

## Ownership Table

| Documentation Zone | Primary Owner Role | Backup Owner Role | Maintenance Expectation |
| --- | --- | --- | --- |
| `README.md` (repo root) | Repository Maintainer | Release Maintainer | Updated on structure/process changes |
| `CONTRIBUTING.md` | Repository Maintainer | Code Review Maintainer | Updated on workflow/policy changes |
| `documentation/` | Documentation Maintainer | Repository Maintainer | Updated when technical docs move or split |
| `.github/documentation/` | Governance Maintainer | Repository Maintainer | Updated on GitHub process/template policy changes |
| `.github/workflows/documentation/` | CI Maintainer | Repository Maintainer | Updated with workflow behavior changes |
| `scripts/**/README.md` and `scripts/**/TOC.md` | Script Owner | Repository Maintainer | Updated with CLI/contract/behavior changes |
| `projects/**/README.md` and `projects/**/documentation/TOC.md` | Product/Library Owner | Repository Maintainer | Updated with module behavior/interface changes |

## Maintenance Rules

- The author of a behavior change updates impacted documentation in the same PR.
- Missing ownership for a new documentation zone must be defined before merge.
- If ownership is unclear, default to `Repository Maintainer` until explicitly delegated.

## Discoverability

- This file is referenced from `documentation/TOC.md`.
- Governance-specific conventions remain under `.github/documentation/`.
