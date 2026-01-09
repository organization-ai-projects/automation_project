#!/bin/bash
set -euo pipefail

REMOTE="${REMOTE:-origin}"
BASE_BRANCH="${BASE_BRANCH:-dev}"
PROTECTED_BRANCHES=("dev" "main")

echo "=== Mise à jour de la branche $BASE_BRANCH ==="
CURRENT_BRANCH="$(git branch --show-current || true)"

git checkout "$BASE_BRANCH"
git pull "$REMOTE" "$BASE_BRANCH"

echo "✓ Branche $BASE_BRANCH mise à jour."
echo ""
echo "=== Détection des branches locales en retard sur $BASE_BRANCH ==="

git fetch "$REMOTE" --prune

OUTDATED_BRANCHES=()

# Parcourir toutes les branches locales
for branch in $(git for-each-ref --format='%(refname:short)' refs/heads); do
  # Ignorer les branches protégées
  for p in "${PROTECTED_BRANCHES[@]}"; do
    [[ "$branch" == "$p" ]] && continue 2
  done

  # Vérifier si BASE_BRANCH a des commits que la branche n'a pas (branche en retard)
  BEHIND_COUNT=$(git rev-list --count "$branch..$BASE_BRANCH" 2>/dev/null || echo "0")

  if (( BEHIND_COUNT > 0 )); then
    echo "  → $branch est en retard de $BEHIND_COUNT commit(s) sur $BASE_BRANCH"
    OUTDATED_BRANCHES+=("$branch")
  fi
done

if (( ${#OUTDATED_BRANCHES[@]} == 0 )); then
  echo "Aucune branche locale en retard détectée."
  exit 0
fi

echo "Branches ciblées :"
printf ' - %s\n' "${OUTDATED_BRANCHES[@]}"

echo ""
echo "=== Suppression et recréation des branches ==="
for branch in "${OUTDATED_BRANCHES[@]}"; do
  echo "Traitement: $branch"

  # Supprimer la branche locale
  if git branch -d "$branch" 2>/dev/null; then
    echo "  ✓ Locale supprimée."
  elif git branch -D "$branch" 2>/dev/null; then
    echo "  ⚠ Locale supprimée (force)."
  else
    echo "  ℹ Locale inexistante."
  fi

  # Supprimer la branche distante si elle existe
  if git ls-remote --exit-code --heads "$REMOTE" "$branch" >/dev/null 2>&1; then
    if git push "$REMOTE" --delete "$branch" >/dev/null 2>&1; then
      echo "  ✓ Distante supprimée."
    else
      echo "  ℹ Distante non supprimée (droits/protection?)."
    fi
  else
    echo "  ℹ Distante inexistante."
  fi

  # Recréer la branche à partir de BASE_BRANCH
  git checkout -b "$branch" "$BASE_BRANCH"
  git push --set-upstream "$REMOTE" "$branch"
  echo "  ✓ Branche recréée."
done

if [[ -n "$CURRENT_BRANCH" ]] && git show-ref --verify --quiet "refs/heads/$CURRENT_BRANCH"; then
  git checkout "$CURRENT_BRANCH"
  echo "✓ Retour sur $CURRENT_BRANCH"
else
  echo "✓ Resté sur $BASE_BRANCH"
fi

echo ""
echo "=== Nettoyage terminé ==="
