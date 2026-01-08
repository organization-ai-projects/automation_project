#!/bin/bash
set -euo pipefail

# Usage: ./cleanup_after_pr.sh
# Description: Nettoie les branches locales en retard vs leur upstream, les supprime (local+remote),
#              puis les recrée depuis dev (et push upstream).

echo "=== Mise à jour de la branche dev ==="
CURRENT_BRANCH="$(git branch --show-current || true)"

if ! git checkout dev; then
  echo "Erreur : Impossible de basculer sur la branche dev." >&2
  exit 1
fi

if ! git pull origin dev; then
  echo "Erreur : Impossible de mettre à jour la branche dev." >&2
  exit 1
fi

echo "✓ Branche dev mise à jour avec succès."
echo ""
echo "=== Détection des branches locales en retard vs upstream ==="

git fetch origin --prune

OUTDATED_BRANCHES=()

# Liste toutes les branches locales + leur "track" upstream (ex: "[behind 2]" / "[ahead 1]" / "[gone]" / "")
# Note: %(upstream:track) est vide si pas d'upstream configuré.
while IFS= read -r line; do
  # line = "<branch>\t<track>"
  branch="${line%%$'\t'*}"
  track="${line#*$'\t'}"

  # Sécurité: ignore dev/main
  if [[ "$branch" == "dev" || "$branch" == "main" ]]; then
    continue
  fi

  # Track examples:
  #   "[behind 2]"
  #   "[ahead 1]"
  #   "[ahead 1, behind 2]"
  #   "[gone]"
  #   "" (no upstream)
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
  echo "Traitement de la branche: $branch"

  if git branch -d "$branch" 2>/dev/null; then
    echo "  ✓ Branche locale $branch supprimée."
  elif git branch -D "$branch" 2>/dev/null; then
    echo "  ⚠ Branche locale $branch supprimée (force - non mergée)."
  else
    echo "  ℹ Branche locale $branch n'existe pas ou déjà supprimée."
  fi

  if git push origin --delete "$branch" 2>/dev/null; then
    echo "  ✓ Branche distante $branch supprimée."
  else
    echo "  ℹ Branche distante $branch n'existe pas, déjà supprimée, ou droits insuffisants."
  fi
done

echo ""
echo "=== Recréation des branches depuis dev ==="
for branch in "${OUTDATED_BRANCHES[@]}"; do
  echo "Recréation de la branche: $branch"

  if ! git checkout -b "$branch" dev; then
    echo "  Erreur : Impossible de créer la branche $branch." >&2
    continue
  fi

  if git push --set-upstream origin "$branch"; then
    echo "  ✓ Branche $branch recréée et poussée avec succès."
  else
    echo "  Erreur : Impossible de pousser la branche recréée $branch." >&2
  fi
done

echo ""
echo "=== Retour sur branche d'origine ==="
if [[ -n "$CURRENT_BRANCH" ]] && git show-ref --verify --quiet "refs/heads/$CURRENT_BRANCH"; then
  git checkout "$CURRENT_BRANCH"
  echo "✓ Retour sur la branche $CURRENT_BRANCH"
else
  echo "✓ Resté sur la branche dev"
fi

echo ""
echo "=== Nettoyage terminé ==="
