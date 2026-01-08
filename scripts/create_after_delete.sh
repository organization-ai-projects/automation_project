#!/bin/bash
set -euo pipefail

# Usage: ./create_after_delete.sh
# Recrée la branche courante depuis dev après suppression locale + remote.

REMOTE="origin"
BASE_BRANCH="dev"

BRANCH_NAME="$(git branch --show-current || true)"

if [[ -z "$BRANCH_NAME" ]]; then
  echo "Erreur : Aucune branche locale active (detached HEAD). Passe sur une branche et relance." >&2
  exit 1
fi

if [[ "$BRANCH_NAME" == "$BASE_BRANCH" || "$BRANCH_NAME" == "main" ]]; then
  echo "Erreur : Refus de supprimer une branche protégée ($BRANCH_NAME)." >&2
  exit 1
fi

echo "=== Recreate branch: $BRANCH_NAME (base: $BASE_BRANCH, remote: $REMOTE) ==="

# Toujours se mettre sur la base avant de supprimer la branche courante
git fetch "$REMOTE" --prune

echo "-> Checkout $BASE_BRANCH"
git checkout "$BASE_BRANCH"
git pull "$REMOTE" "$BASE_BRANCH"

echo "-> Delete local branch $BRANCH_NAME (safe)"
if git show-ref --verify --quiet "refs/heads/$BRANCH_NAME"; then
  if git branch -d "$BRANCH_NAME"; then
    echo "✓ Branche locale \"$BRANCH_NAME\" supprimée."
  else
    echo "Erreur : Branche locale \"$BRANCH_NAME\" non mergée, suppression refusée." >&2
    echo "Astuce : merge-la ou supprime en force avec: git branch -D \"$BRANCH_NAME\"" >&2
    exit 1
  fi
else
  echo "ℹ Branche locale \"$BRANCH_NAME\" inexistante."
fi

echo "-> Delete remote branch $BRANCH_NAME (if exists)"
if git ls-remote --exit-code --heads "$REMOTE" "$BRANCH_NAME" >/dev/null 2>&1; then
  git push "$REMOTE" --delete "$BRANCH_NAME"
  echo "✓ Branche distante \"$BRANCH_NAME\" supprimée."
else
  echo "ℹ Branche distante \"$BRANCH_NAME\" inexistante."
fi

echo "-> Create branch from $BASE_BRANCH"
git checkout -b "$BRANCH_NAME" "$BASE_BRANCH"

echo "-> Push & set upstream"
git push --set-upstream "$REMOTE" "$BRANCH_NAME"

echo "✓ Branche \"$BRANCH_NAME\" recréée depuis \"$BASE_BRANCH\" et poussée sur \"$REMOTE\"."
