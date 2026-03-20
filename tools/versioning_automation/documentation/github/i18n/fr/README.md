# Documentation des automatisations GitHub

Langue : [English](../../README.md) | **Francais**

Ce repertoire conserve la documentation des automatisations GitHub.
La logique d'automatisation est migree en Rust dans `tools/versioning_automation`, executee via `versioning_automation`.

## Structure du repertoire

```text
github/
├── README.md
├── TOC.md
└── i18n/
```

## Entrees canoniques (Rust)

- `versioning_automation pr generate-description ...`
- `versioning_automation pr refresh-validation ...`
- `versioning_automation pr auto-add-closes ...`
- `versioning_automation pr directive-conflict-guard ...`
- `versioning_automation issue auto-link ...`
- `versioning_automation issue create ...`
- `versioning_automation issue <read|update|close|reopen|delete> ...`
- `versioning_automation issue done-status ...`
- `versioning_automation issue reopen-on-dev ...`
- `versioning_automation issue neutralize ...`
- `versioning_automation issue reevaluate ...`
- `versioning_automation issue parent-guard ...`
- `versioning_automation issue closure-hygiene ...`

## Suites de regression Rust

- `cargo test -q -p versioning_automation pr::tests::generate_description`
- `cargo test -q -p versioning_automation pr::tests::refresh_validation`
- `cargo test -q -p versioning_automation`

Les checks shell transverses restent dans `scripts/automation/tests/`.

## Notes

- Les workflows GitHub Actions appellent `target/debug/versioning_automation ...` directement.
- Aucun entrypoint runtime shell ne reste pour l'automatisation GitHub.
