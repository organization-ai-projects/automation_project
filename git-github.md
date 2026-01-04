# Workflow Git/GitHub

Ce document décrit le workflow Git/GitHub utilisé pour le projet `automation_project`.

---

## 1. Branches principales

### `main`

- **Description** : Branche stable contenant les versions validées et prêtes pour la production.
- **Règle** : Aucun commit direct. Les modifications proviennent uniquement de la branche `dev` après stabilisation.

### `dev`

- **Description** : Branche de développement contenant les fonctionnalités en cours de test.
- **Règle** : Aucun commit direct. Les modifications proviennent uniquement des branches de travail via des PRs.

---

## 2. Branches de travail

### Création

- Une branche de travail est créée pour chaque nouvelle fonctionnalité ou correction de bug.
- **Convention de nommage** : `feature/<nom>` ou `fix/<nom>`.
  - Exemple : `feature/ui-improvements`, `fix/bug-123`.

### Fusion

- Les branches de travail sont fusionnées dans `dev` via une Pull Request (PR).
- **Règle** :
  - La PR doit être approuvée avant la fusion.
  - Les tests doivent passer avant la fusion.

---

## 3. Processus de fusion

### De `dev` vers `main`

1. Stabiliser la branche `dev`.
2. Effectuer des tests approfondis.
3. Créer une PR de `dev` vers `main`.
4. Une fois approuvée, fusionner dans `main`.

### De branche de travail vers `dev`

1. Créer une PR de la branche de travail vers `dev`.
2. Attendre l’approbation et s’assurer que les tests passent.
3. Fusionner dans `dev`.

---

## 4. Règles générales

- **Tests** : Toute modification doit être accompagnée de tests (unitaires, intégration, etc.).
- **Commits** : Les messages de commit doivent être clairs et suivre une convention (ex. : `fix: correct bug in X`, `feat: add new feature Y`).
- **Rebase** : Rebaser régulièrement les branches de travail sur `dev` pour éviter les conflits.

---

## 5. Commandes utiles

### Initialisation du dépôt local

```bash
git clone https://github.com/organization-ai-projects/automation_project.git
cd automation_project
git checkout -b dev origin/dev
```

### Création d’une branche de travail

```bash
git checkout dev
git pull origin dev
git checkout -b feature/<nom>
```

### Fusion d’une branche de travail dans `dev`

```bash
git checkout dev
git pull origin dev
git merge feature/<nom>
git push origin dev
```

### Fusion de `dev` dans `main`

```bash
git checkout main
git pull origin main
git merge dev
git push origin main
```

---

**Ce workflow garantit une gestion propre et collaborative du code.**
