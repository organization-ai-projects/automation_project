# Table des matieres

Langue : [English](../../TOC.md) | **Francais**

## Documentation

- [README.md](../../README.md) : vue d'ensemble des automatisations GitHub et des entrees Rust canoniques

## Tests de regression

- [tests/generate_pr_description_regression.sh](../../tests/generate_pr_description_regression.sh) : matrice de regression de generation de description PR
- [tests/refresh_pr_issue_extraction_regression.sh](../../tests/refresh_pr_issue_extraction_regression.sh) : matrice de regression de rafraichissement PR/issues
- [tests/auto_add_closes_on_dev_pr_regression.sh](../../tests/auto_add_closes_on_dev_pr_regression.sh) : verifications de l'enrichissement auto `Closes #...`
- [tests/pr_directive_conflict_guard_regression.sh](../../tests/pr_directive_conflict_guard_regression.sh) : verifications du garde de conflits de directives
- [tests/auto_link_parent_issue_regression.sh](../../tests/auto_link_parent_issue_regression.sh) : verifications d'auto-liaison parent
- [tests/issue_done_in_dev_status_regression.sh](../../tests/issue_done_in_dev_status_regression.sh) : verifications du cycle `done-in-dev`
- [tests/issue_reopen_on_dev_merge_regression.sh](../../tests/issue_reopen_on_dev_merge_regression.sh) : verifications de synchronisation `Reopen #...`
- [tests/neutralize_closure_refs_regression.sh](../../tests/neutralize_closure_refs_regression.sh) : verifications neutralisation/reevaluate
- [tests/parent_issue_guard_regression.sh](../../tests/parent_issue_guard_regression.sh) : verifications du garde parent
- [tests/closure_hygiene_on_main_merge_regression.sh](../../tests/closure_hygiene_on_main_merge_regression.sh) : verifications closure hygiene sur merge main
- [tests/manager_issues_regression.sh](../../tests/manager_issues_regression.sh) : verifications des commandes cycle de vie issues
- [tests/shellcheck_regression.sh](../../tests/shellcheck_regression.sh) : lint shell des harness restants
- [tests/enforcer_shell_contract_regression.sh](../../tests/enforcer_shell_contract_regression.sh) : verifications du contrat shell via enforcer

## Navigation

- [Retour au TOC file_versioning](../../TOC.md)
