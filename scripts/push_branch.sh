#!/bin/bash
set -euo pipefail  # Fail fast sur erreur, variable non définie ou pipe qui échoue

# Usage: ./push_branch.sh
# Description: Pousse la branche courante vers origin avec protection dev/main

# Récupérer la branche courante
BRANCH_NAME=$(git branch --show-current)

# Vérifier si une branche est active
if [ -z "$BRANCH_NAME" ]; then
    echo "Erreur : Aucune branche locale active. Vous devez être sur une branche pour l'utiliser." >&2
    exit 1
fi

# Interdire le push direct vers dev et main
if [[ "$BRANCH_NAME" == "dev" || "$BRANCH_NAME" == "main" ]]; then
    echo "Erreur : Le push direct vers 'dev' ou 'main' est interdit." >&2
    exit 1
fi

# Vérifier si la branche existe déjà sur le distant
if git ls-remote --heads origin "$BRANCH_NAME" 2>/dev/null | grep -q "$BRANCH_NAME"; then
    # Branche existe déjà, push simple
    echo "Push de la branche existante '$BRANCH_NAME'..."
    if git push origin "$BRANCH_NAME"; then
        echo "✓ Branche $BRANCH_NAME poussée avec succès."
    else
        echo "Erreur : Impossible de pousser la branche $BRANCH_NAME." >&2
        exit 1
    fi
else
    # Première fois, configurer le tracking
    echo "Première poussée de la branche '$BRANCH_NAME', configuration du tracking..."
    if git push --set-upstream origin "$BRANCH_NAME"; then
        echo "✓ Branche $BRANCH_NAME poussée avec succès et tracking configuré."
    else
        echo "Erreur : Impossible de pousser la branche $BRANCH_NAME." >&2
        exit 1
    fi
fi