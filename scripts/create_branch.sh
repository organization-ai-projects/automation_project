#!/bin/bash
# Usage: ./create_branch.sh <branch-name>

if [ -z "$1" ]; then
  # Si aucun nom n'est fourni, vérifier si un nom de branche supprimée est disponible
  if [ -f /tmp/last_deleted_branch ]; then
    BRANCH_NAME=$(cat /tmp/last_deleted_branch)
    echo "Aucun nom fourni. Recréation de la dernière branche supprimée : $BRANCH_NAME"
  else
    echo "Erreur : Vous devez spécifier un nom de branche ou aucune branche supprimée récemment n'est disponible."
    exit 1
  fi
else
  BRANCH_NAME=$1
fi

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