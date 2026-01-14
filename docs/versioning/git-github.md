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

## 4. Synchronisation après une PR

Une fois qu'une PR de votre branche de travail a été fusionnée dans `dev`, vous devez synchroniser votre dépôt local pour rester à jour :

1. **Mettre à jour la branche `dev` locale** :

   ```bash
   git checkout dev
   git pull origin dev
   ```

2. **Supprimer la branche de travail locale si elle n'est plus nécessaire** :

   ```bash
   git branch -d feature/<nom>
   ```

3. **Supprimer la branche de travail distante si elle n'est plus nécessaire** :

   ```bash
   git push origin --delete feature/<nom>
   ```

4. **Créer une nouvelle branche de travail si nécessaire** :
   Si vous commencez une nouvelle tâche, créez une nouvelle branche à partir de la version mise à jour de `dev` :

   ```bash
   git checkout -b feature/<nouvelle-tâche>
   ```

### Gestion des branches de travail persistantes

Si vous souhaitez conserver une branche de travail pour y revenir plus tard :

1. **Mettre à jour la branche de travail avec `dev`** :
   Avant de reprendre le travail sur une branche existante, assurez-vous qu'elle est synchronisée avec les derniers changements de `dev` :

   ```bash
   git checkout feature/<nom>
   git pull origin dev
   git merge dev
   ```

2. **Pousser les mises à jour vers la branche distante** :
   Si vous avez fusionné ou ajouté des modifications, poussez-les vers la branche distante pour éviter les divergences :

   ```bash
   git push origin feature/<nom>
   ```

3. **Reprendre le travail** :
   Continuez à travailler sur la branche comme d'habitude. Une fois terminé, créez une nouvelle PR pour fusionner les modifications dans `dev`.

4. **Supprimer la branche si elle n'est plus nécessaire** :
   Si la branche n'est plus utile, suivez les étapes de suppression mentionnées ci-dessus.

---

## 5. Règles générales

- **Tests** : Toute modification doit être accompagnée de tests (unitaires, intégration, etc.). Les tests peuvent être absents temporairement lors des phases exploratoires, mais sont requis avant toute fusion de `dev` vers `main`. Les tests peuvent être exécutés localement ou via CI lorsqu’elle est disponible.
- **Commits** : Les messages de commit doivent être clairs et suivre une convention (ex. : `fix: correct bug in X`, `feat: add new feature Y`).
- **Fusion** : Le merge doit être strictement utilisé pour intégrer les modifications, afin de préserver l'intégrité et l'historique complet des commits.

---

## 6. Commandes utiles

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

### Automatisation avec des scripts

Pour simplifier certaines tâches répétitives, voici des scripts que vous pouvez utiliser :

#### Script : Créer une nouvelle branche de travail

```bash
#!/bin/bash
# Usage: ./create_branch.sh <branch-name>

if [ -z "$1" ]; then
  echo "Erreur : Vous devez spécifier un nom de branche."
  exit 1
fi

BRANCH_NAME=$1

git checkout dev
if [ $? -ne 0 ]; then
  echo "Erreur : Impossible de basculer sur la branche dev."
  exit 1
fi

git pull origin dev
if [ $? -ne 0 ]; then
  echo "Erreur : Impossible de mettre à jour dev."
  exit 1
fi

git checkout -b $BRANCH_NAME
if [ $? -eq 0 ]; then
  echo "Branche $BRANCH_NAME créée avec succès."
else
  echo "Erreur : Impossible de créer la branche $BRANCH_NAME."
fi
```

#### Script : Supprimer une branche locale et distante

```bash
#!/bin/bash
# Usage: ./delete_branch.sh <branch-name>

if [ -z "$1" ]; then
  echo "Erreur : Vous devez spécifier un nom de branche."
  exit 1
fi

BRANCH_NAME=$1

git branch -d $BRANCH_NAME
if [ $? -eq 0 ]; then
  echo "Branche locale $BRANCH_NAME supprimée."
else
  echo "Erreur : Impossible de supprimer la branche locale $BRANCH_NAME."
  exit 1
fi

git push origin --delete $BRANCH_NAME
if [ $? -eq 0 ]; then
  echo "Branche distante $BRANCH_NAME supprimée."
else
  echo "Erreur : Impossible de supprimer la branche distante $BRANCH_NAME."
fi
```

---

**Ce workflow garantit une gestion propre et collaborative du code.**

le commit doit être conforme à semver, il faut également utiliser la convention suivante pour le scope :
libraries/[nom de la librairie]
products/[nom du produit]
