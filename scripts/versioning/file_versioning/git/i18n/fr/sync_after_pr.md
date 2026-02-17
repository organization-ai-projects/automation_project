# Workflow: synchroniser apres merge PR

Langue : [English](../../sync_after_pr.md) | **Francais**

Ce document explique comment synchroniser les branches locales apres un merge PR, en manuel ou via `cleanup_after_pr.sh`.

## Vue d'ensemble

Apres merge vers la branche de base (souvent `dev`), les branches locales peuvent devenir en retard.
Il faut:

1. Mettre a jour la branche de base
2. Identifier les branches locales en retard
3. Nettoyer les branches obsoletes (et optionnellement les recreer depuis la base)

## Quand choisir chaque approche

### Cas d'usage du nettoyage manuel

- Pour apprendre le workflow pas a pas
- Pour une ou deux branches seulement
- Pour garder un controle selectif branche par branche
- Pour diagnostiquer un probleme precis

### Nettoyage automatise

- Pour beaucoup de branches en retard
- Pour la maintenance reguliere apres merge
- Pour un processus repetable et rapide

Recommandation: faire une premiere passe manuelle, puis utiliser le script pour la routine.

## Nettoyage manuel

### 1) Mettre a jour la branche de base

```bash
git checkout dev
git pull origin dev
```

### 2) Identifier les branches en retard

```bash
for branch in $(git for-each-ref --format='%(refname:short)' refs/heads | grep -vE '^(dev|main)$'); do
  BEHIND=$(git rev-list --count "$branch..dev" 2>/dev/null || echo "0")
  if [ "$BEHIND" -gt 0 ]; then
    echo "Branch $branch is behind by $BEHIND commit(s)"
  fi
done
```

### 3) Nettoyer les branches en retard

Option A - supprimer seulement (travail deja merge/abandonne):

```bash
git branch -d <branch-name>
# ou
git branch -D <branch-name>

git push origin --delete <branch-name>
```

Option B - supprimer puis recreer (travail a poursuivre):

```bash
git branch -D <branch-name>
git push origin --delete <branch-name>

git checkout -b <branch-name> dev
git push --set-upstream origin <branch-name>
```

### 4) Revenir a la branche de travail

```bash
git checkout <your-current-branch>
```

## Nettoyage automatise avec cleanup_after_pr.sh

### Demarrage rapide

Comportement par defaut (supprime + recree):

```bash
cd scripts/versioning/file_versioning/git
./cleanup_after_pr.sh
```

Le script:

1. Met a jour la base (defaut `dev`)
2. Detecte les branches en retard
3. Supprime local + distant si necessaire
4. Recree depuis la base mise a jour
5. Revient sur la branche initiale

Mode suppression seule:

```bash
./cleanup_after_pr.sh --delete-only
```

## Configuration

```bash
# Remote different
REMOTE=upstream ./cleanup_after_pr.sh

# Base differente
BASE_BRANCH=main ./cleanup_after_pr.sh

# Combinaison
REMOTE=upstream BASE_BRANCH=main ./cleanup_after_pr.sh --delete-only
```

## Options

```bash
./cleanup_after_pr.sh [OPTIONS]

Options:
  --delete-only    Supprime sans recreer
  --help, -h       Affiche l'aide

Variables d'environnement:
  REMOTE           Remote Git (defaut: origin)
  BASE_BRANCH      Branche de base comparee (defaut: dev)
```

## Bonnes pratiques de securite

- Le script ignore automatiquement `dev` et `main`.
- Il tente d'abord `git branch -d`, puis bascule vers `-D` si necessaire.
- `git branch -D` peut supprimer une branche avec commits non merges.

Verifier avant execution que les branches cibles sont bien mergees ou abandonnees.

## Depannage rapide

Erreur:

```text
error: The branch '<branch-name>' is not fully merged.
```

Signifie que des commits ne sont pas dans la base.
Options:

- Verifier le statut du merge/PR
- Forcer la suppression uniquement si voulu: `git branch -D <branch-name>`

Si une branche distante ne se supprime pas, c'est souvent une protection/permission.
Le nettoyage local peut tout de meme etre valide.

## Resume

| Aspect | Manuel | Script |
|--------|--------|--------|
| Ideal pour | apprentissage, cas ponctuel | maintenance reguliere |
| Vitesse | plus lente | rapide |
| Controle | maximal | moyen |
| Risque d'erreur | faible si attentif | reduit par checks automatiques |

Point de depart conseille: tester une fois en manuel, puis automatiser avec `cleanup_after_pr.sh`.
