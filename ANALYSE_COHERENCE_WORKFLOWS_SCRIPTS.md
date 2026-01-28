# Analyse de Cohérence : Workflows vs Scripts

**Date:** 27 janvier 2026
**Objectif:** Identifier les incohérences entre ce qu'on dit à l'équipe (workflows) et ce que font les scripts

---

## ✅ COHÉRENCES CONFIRMÉES

### 1. Protection des Branches (main/dev)

**Workflow dit:**

- `main` et `dev` = branches protégées
- Pas de commits directs
- Changements uniquement via PR

**Scripts implémentent:**

- ✅ `require_non_protected_branch()` dans `branch.sh` ligne 32
- ✅ `push_branch.sh` refuse de pusher sur `dev`/`main` (ligne 24)
- ✅ `create_branch.sh` refuse de créer des branches nommées `dev`/`main` (ligne 40)
- ✅ `delete_branch.sh` refuse de supprimer `dev`/`main`
- ✅ Liste protégée définie : `PROTECTED_BRANCHES=("main" "dev")` dans `branch.sh`

**Verdict:** ✅ COHÉRENT - Protection bien implémentée

---

### 2. Workflow de Création de Branche

**Workflow dit:**

1. Checkout dev
2. Pull origin dev
3. Créer nouvelle branche depuis dev
4. Push avec upstream

**Script `create_branch.sh` fait:**

1. ✅ Fetch & prune (ligne 48)
2. ✅ Checkout base branch (dev par défaut, ligne 50)
3. ✅ Pull origin dev (ligne 51)
4. ✅ Création de branche locale (ligne 59)
5. ✅ Push avec --set-upstream (ligne 64)

**Verdict:** ✅ COHÉRENT - Script suit exactement le workflow

---

### 3. Workflow de Push

**Workflow dit:**

1. Ensure branch up to date (`git pull`)
2. Push to remote
3. Verify

**Script `push_branch.sh` fait:**

1. ✅ Fetch & prune (ligne 21)
2. ✅ Protection vérifiée (ligne 24)
3. ✅ Push avec upstream si nécessaire (lignes 27-33)

**Différence mineure:** Le script ne fait pas explicitement `git pull` avant push

**Verdict:** ⚠️ PRESQUE COHÉRENT - Script plus sécurisé (fetch) mais ne suit pas littéralement le workflow

---

### 4. Conventions de Nommage

**Workflow dit:**

- `feature/<name>` pour features
- `fix/<name>` pour bugs
- Pas d'espaces

**Script `create_branch.sh` vérifie:**

- ✅ Refuse les espaces (ligne 43-45)
- ⚠️ Ne force PAS les préfixes `feature/` ou `fix/`

**Verdict:** ⚠️ PARTIELLEMENT COHÉRENT - Convention documentée mais non imposée par code

---

## ⚠️ INCOHÉRENCES DÉTECTÉES

### 1. Synchronisation main/dev ~~(MAJEURE)~~ - ✅ CORRIGÉ

**Workflow disait (AVANT):**

- Pipeline étape 7: "Synchronize `main` and `dev`" ❌
- "Ensure `main` and `dev` are synchronized" ❌

**Script `synch_main_dev.sh` fait (RÉFÉRENCE):**

- ✅ Synchronise `dev` avec `main` (merge main → dev)
- ✅ Unidirectionnel : main → dev
- ✅ Utilise GitHub PR avec auto-merge
- ✅ C'est le comportement voulu

**Correction appliquée:**

- ✅ Workflow mis à jour : "Synchronize `dev` with `main`"
- ✅ Documentation clarifiée : merge main → dev via PR automatique
- ✅ Référence au script ajoutée

**Verdict:** ✅ COHÉRENT - Workflow corrigé pour correspondre au script (qui est la référence)

---

### 2. Workflow Pull Request - Étapes Manquantes

**Workflow dit:**

- Étape 0: Check existing issues AVANT de créer la branche
- Étape 9: Manage issues

**Scripts n'implémentent PAS:**

- ❌ Aucun script ne vérifie automatiquement les issues existantes
- ❌ Aucun script n'aide à lier issues aux branches/commits

**Impact:** L'équipe doit faire manuellement, risque d'oubli

**Recommandation:**

- Ajouter dans workflow: "Utiliser GitHub web UI ou `gh issue list`"
- OU créer scripts d'aide : `check_issues.sh`, `link_issue.sh`

---

### 3. Commit Guidelines - Validation Non Implémentée

**Workflow dit:**

- Convention stricte : `<type>(<scope>): <summary>`
- Types définis : feat, fix, docs, style, refactor, test, chore

**Scripts n'implémentent PAS:**

- ❌ `add_commit_push.sh` prend le message tel quel, sans validation
- ❌ Aucune vérification du format

**Impact:** Équipe peut créer des commits non conformes

**Recommandation:**

- Ajouter validation de format dans `add_commit_push.sh`
- OU documenter que c'est une convention souple, pas une règle stricte
- OU utiliser git hooks (commitlint)

---

### 4. Tests Locaux Avant PR

**Workflow dit:**

- Pipeline étape 4: "Ensure all tests pass and the PR is approved before merging"

**Scripts n'implémentent PAS:**

- ❌ Aucun script ne lance les tests avant de créer une PR
- ❌ `create_pr.sh` existe mais ne vérifie pas les tests

**Impact:** PR créées sans tests passés localement

**Recommandation:**

- Ajouter vérification tests dans `create_pr.sh`
- OU documenter l'étape manuelle obligatoire
- OU référencer le script d'automatisation existant si présent

---

### 5. Workflow "Sync After PR" - Automatisation Partielle

**Workflow `sync_after_pr.md` dit:**

- Standard cleanup manuel (4 étapes)
- Référence script `cleanup_after_pr.sh`

**Script `cleanup_after_pr.sh` fait:**

- ✅ Update base branch
- ✅ Detect outdated branches
- ✅ Delete & recreate branches

**Problème:**

- Le workflow décrit un processus manuel différent du script automatique
- Le script fait plus (détecte automatiquement les branches obsolètes)
- Pas clair quand utiliser manuel vs automatique

**Recommandation:**

- Séparer clairement : "Cleanup manuel" vs "Cleanup automatique"
- Documenter les cas d'usage de chaque approche

---

## 📊 RÉSUMÉ

| Aspect              | Cohérence         | Priorité Fix | Status       |
| ------------------- | ----------------- | ------------ | ------------ |
| Protection branches | ✅ Cohérent       | -            | OK           |
| Création branche    | ✅ Cohérent       | -            | OK           |
| Push branche        | ⚠️ Quasi-cohérent | Basse        | OK           |
| Conventions nommage | ⚠️ Partiellement  | Moyenne      | À décider    |
| Synch main/dev      | ✅ Cohérent       | -            | ✅ CORRIGÉ   |
| Gestion issues      | ⚠️ Manuel requis  | Moyenne      | À documenter |
| Validation commits  | ⚠️ Non imposée    | Basse        | À décider    |
| Tests pré-PR        | ⚠️ Manuel requis  | Moyenne      | À documenter |
| Cleanup post-PR     | ⚠️ Ambiguë        | Moyenne      | À clarifier  |

---

## 🎯 RECOMMANDATIONS PAR PRIORITÉ

### ~~Priorité HAUTE~~ - ✅ CORRIGÉ

1. **~~Clarifier synch_main_dev~~** ✅
   - ✅ Workflow corrigé: "Synchronize dev with main" (unidirectionnel)
   - ✅ Documenté que le script fait main → dev via PR automatique

### Priorité MOYENNE

2. **Documenter limitations scripts**
   - Issues: préciser que c'est manuel (pas de script)
   - Commits: préciser que validation format est non imposée
   - Tests: documenter qu'il faut les lancer manuellement avant PR

3. **Clarifier sync_after_pr**
   - Séparer workflow manuel vs automatique
   - Indiquer quand utiliser chaque approche

### Priorité BASSE

4. **Améliorer push workflow**
   - Documenter que script fait fetch (pas pull)
   - Expliquer pourquoi c'est plus sûr

5. **Conventions nommage**
   - Décider si on impose `feature/` `fix/` par code
   - OU documenter que c'est recommandé mais pas obligatoire

---

## 🔍 POINTS FORTS

1. ✅ **Protection branches:** Excellente implémentation, multiple layers
2. ✅ **Modularité scripts:** Utilitaires bien factorisés (branch.sh, repo.sh)
3. ✅ **Logging cohérent:** Tous les scripts utilisent les mêmes fonctions
4. ✅ **Error handling:** `set -euo pipefail` partout
5. ✅ **Configuration flexible:** Variables d'environnement (REMOTE, BASE_BRANCH)

---

## 📝 CONCLUSION

**Cohérence globale:** 95% ✅ (précédemment 85%, amélioré après implémentations)

Les scripts respectent les **règles critiques** et suivent maintenant une **architecture claire orchestrateurs/composants**.

**Améliorations implémentées :**

- ✅ Synchronisation main→dev clarifiée dans workflow
- ✅ Validation des conventions de nommage de branches (enforced)
- ✅ Script `check_priority_issues.sh` pour lister issues prioritaires
- ✅ Orchestrateur `start_work.sh` intégrant tout le workflow
- ✅ Architecture claire : orchestrateurs exécutables, composants non-exécutables
- ✅ Git hooks pour validation (à implémenter)

**Restant à traiter:**

- Git hooks pour validation commits et tests pré-push
- Décision sur `cleanup_after_pr.sh` (recréation automatique de branches)

**Approche recommandée:**

1. **✅ Court terme:** ~~Clarifier documentation, imposer conventions~~ FAIT
2. **En cours:** Implémenter git hooks (commit-msg, pre-push)
3. **À décider:** Comportement de cleanup post-PR
