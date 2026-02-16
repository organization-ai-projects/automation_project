# Guide de stabilisation des tests flaky

This file is a French translation of the English documentation. If it becomes outdated, refer to the English version.

Ce document definit le workflow workspace pour le triage et la stabilisation des tests flaky.

## Objectif

- Suivre les tests flaky connus dans un seul endroit.
- Appliquer d'abord des corrections deterministes.
- Garder un comportement CI et pre-push predictible.

## Inventaire actuel

| Test | Zone | Classification | Statut | Mitigation |
| --- | --- | --- | --- | --- |
| `store::tests::audit_buffer::test_manual_flush` | `projects/products/stable/accounts/backend` | Timing / visibilite async I/O | Stabilise | Poll de visibilite disque apres flush manuel |

## Classification

- Conditions de timing/race
- Collisions d'etat partage
- Visibilite/latence I/O
- Hypotheses de polling async
- Dependances d'environnement

## Workflow de remediation

1. Reproduire le test flaky localement avec des executions repetees.
2. Classifier le mode d'echec.
3. Stabiliser avec setup/assertions deterministes (polling, isolation, ordering).
4. Ajouter ou mettre a jour une assertion de test de regression.
5. Mettre a jour la ligne d'inventaire avec statut et mitigation.

## Recommandations CI et pre-push

- Ne pas bypass silencieusement les echec flaky.
- Si la stabilisation n'est pas immediate, isoler avec une issue de suivi tracee.
- Mettre a jour cet inventaire dans la meme PR que le travail de remediation.
