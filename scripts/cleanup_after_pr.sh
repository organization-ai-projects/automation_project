#!/bin/bash
set -euo pipefail

# Usage: ./cleanup_after_pr.sh
# Description: Nettoie les branches mergées et les recrée depuis dev à jour

echo "=== Mise à jour de la branche dev ==="
# Sauvegarder la branche courante
CURRENT_BRANCH=$(git branch --show-current)

# Mettre à jour la branche dev
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
echo "=== Détection des branches locales non à jour ==="
# Récupérer les informations distantes
git fetch origin

# Trouver les branches locales non à jour
OUTDATED_BRANCHES=$(git for-each-ref --format='%(refname:short)' refs/heads | while read BRANCH; do
    STATUS=$(git status -sb --branch "$BRANCH" 2>/dev/null || true)
    if [[ "$STATUS" == *"behind"* ]]; then
        echo "$BRANCH"
    fi
done)

if [ -z "$OUTDATED_BRANCHES" ]; then
    echo "Aucune branche locale non à jour détectée."
    exit 0
fi

echo "Branches locales non à jour détectées :"
echo "$OUTDATED_BRANCHES"

echo ""
echo "=== Suppression des branches locales et distantes ==="
for BRANCH in $OUTDATED_BRANCHES; do
    # Ignorer dev et main par sécurité
    if [[ "$BRANCH" == "dev" || "$BRANCH" == "main" ]]; then
        echo "⚠ Branche protégée '$BRANCH' ignorée."
        continue
    fi

    echo "Traitement de la branche: $BRANCH"

    # Supprimer la branche locale (force si non mergée)
    if git branch -d "$BRANCH" 2>/dev/null; then
        echo "  ✓ Branche locale $BRANCH supprimée."
    elif git branch -D "$BRANCH" 2>/dev/null; then
        echo "  ⚠ Branche locale $BRANCH supprimée (force - non mergée)."
    else
        echo "  ℹ Branche locale $BRANCH n'existe pas ou déjà supprimée."
    fi

    # Supprimer la branche distante
    if git push origin --delete "$BRANCH" 2>/dev/null; then
        echo "  ✓ Branche distante $BRANCH supprimée."
    else
        echo "  ℹ Branche distante $BRANCH n'existe pas ou déjà supprimée."
    fi
done

echo ""
echo "=== Recréation des branches depuis dev ==="
for BRANCH in $OUTDATED_BRANCHES; do
    # Ignorer dev et main
    if [[ "$BRANCH" == "dev" || "$BRANCH" == "main" ]]; then
        continue
    fi

    echo "Recréation de la branche: $BRANCH"

    if ! git checkout -b "$BRANCH" dev; then
        echo "  Erreur : Impossible de créer la branche $BRANCH." >&2
        continue
    fi

    if git push --set-upstream origin "$BRANCH"; then
        echo "  ✓ Branche $BRANCH recréée et poussée avec succès."
    else
        echo "  Erreur : Impossible de pousser la branche recréée $BRANCH." >&2
    fi
done

# Retourner sur la branche d'origine ou dev
if [ -n "$CURRENT_BRANCH" ] && git show-ref --verify --quiet "refs/heads/$CURRENT_BRANCH"; then
    git checkout "$CURRENT_BRANCH"
    echo "✓ Retour sur la branche $CURRENT_BRANCH"
else
    echo "✓ Resté sur la branche dev"
fi

echo ""
echo "=== Nettoyage terminé ==="