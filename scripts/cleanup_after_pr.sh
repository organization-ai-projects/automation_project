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
echo "=== Détection des branches locales en retard vs upstream ==="

git fetch "$REMOTE" --prune

OUTDATED_BRANCHES=()

while IFS=$'\t' read -r branch track; do
  # ignore protected
  for p in "${PROTECTED_BRANCHES[@]}"; do
    [[ "$branch" == "$p" ]] && continue 2
  done

  # ignore branches without upstream (safe default)
  if [[ -z "$track" ]]; then
    continue
  fi

  # mark as outdated if behind or gone
  if [[ "$track" == *"behind"* || "$track" == *"gone"* ]]; then
    OUTDATED_BRANCHES+=("$branch")
  fi
done < <(git for-each-ref --format='%(refname:short)%09%(upstream:track)' refs/heads)

if (( ${#OUTDATED_BRANCHES[@]} == 0 )); then
  echo "Aucune branche locale en retard détectée."
  exit 0
fi

echo "Branches ciblées :"
printf ' - %s\n' "${OUTDATED_BRANCHES[@]}"

echo ""
echo "=== Suppression des branches locales et distantes ==="
for branch in "${OUTDATED_BRANCHES[@]}"; do
  echo "Traitement: $branch"

  # delete local (safe then force)
  if git branch -d "$branch" 2>/dev/null; then
    echo "  ✓ Locale supprimée."
  elif git branch -D "$branch" 2>/dev/null; then
    echo "  ⚠ Locale supprimée (force)."
  else
    echo "  ℹ Locale inexistante."
  fi

  # delete remote only if it exists
  if git ls-remote --exit-code --heads "$REMOTE" "$branch" >/dev/null 2>&1; then
    if git push "$REMOTE" --delete "$branch" >/dev/null 2>&1; then
      echo "  ✓ Distante supprimée."
    else
      echo "  ℹ Distante non supprimée (droits/protection?)."
    fi
  else
    echo "  ℹ Distante inexistante."
  fi
done

echo ""
echo "=== Recréation depuis $BASE_BRANCH ==="
for branch in "${OUTDATED_BRANCHES[@]}"; do
  echo "Recréation: $branch"
  git checkout -b "$branch" "$BASE_BRANCH"
  git push --set-upstream "$REMOTE" "$branch"
  echo "  ✓ OK."
done

echo ""
echo "=== Retour sur branche d'origine ==="
if [[ -n "$CURRENT_BRANCH" ]] && git show-ref --verify --quiet "refs/heads/$CURRENT_BRANCH"; then
  git checkout "$CURRENT_BRANCH"
  echo "✓ Retour sur $CURRENT_BRANCH"
else
  echo "✓ Resté sur $BASE_BRANCH"
fi

echo ""
echo "=== Nettoyage terminé ==="
