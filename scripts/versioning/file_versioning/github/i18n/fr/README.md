# Documentation des automatisations GitHub

Langue : [English](../../README.md) | **Francais**

Ce repertoire conserve la documentation et les tests de regression GitHub.
La logique d'automatisation est migree en Rust dans `tools/versioning_automation`, executee via `versioning_automation`.

## Structure du repertoire

```text
github/
├── README.md
├── TOC.md
├── i18n/
└── tests/
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

## Tests de regression

- `tests/generate_pr_description_regression.sh`
- `tests/refresh_pr_issue_extraction_regression.sh`
- `tests/auto_add_closes_on_dev_pr_regression.sh`
- `tests/pr_directive_conflict_guard_regression.sh`
- `tests/auto_link_parent_issue_regression.sh`
- `tests/issue_done_in_dev_status_regression.sh`
- `tests/issue_reopen_on_dev_merge_regression.sh`
- `tests/neutralize_closure_refs_regression.sh`
- `tests/parent_issue_guard_regression.sh`
- `tests/closure_hygiene_on_main_merge_regression.sh`
- `tests/manager_issues_regression.sh`
- `tests/shellcheck_regression.sh`
- `tests/enforcer_shell_contract_regression.sh`

## Notes

- Les workflows GitHub Actions appellent `target/debug/versioning_automation ...` directement.
- Aucun entrypoint runtime shell ne reste sous `scripts/versioning/file_versioning/github`.
