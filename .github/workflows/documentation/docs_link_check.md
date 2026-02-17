# docs_link_check.yml

## Purpose

Validate Markdown links for the EN/FR documentation rollout scope.

## Trigger

- Pull requests touching:
  - `README.md`, `README.fr.md`
  - `CONTRIBUTING.md`, `CONTRIBUTING.fr.md`
  - `documentation/**`
  - this workflow documentation
- Manual run (`workflow_dispatch`)

## Coverage

Current checked files:

- `README.md`
- `README.fr.md`
- `CONTRIBUTING.md`
- `CONTRIBUTING.fr.md`
- `documentation/README.md`
- `documentation/TOC.md`
- `documentation/i18n/fr/TOC.md`

## Notes

- The workflow uses `lycheeverse/lychee-action`.
- Scope can be expanded as EN/FR migration progresses.
