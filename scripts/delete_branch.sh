#!/bin/bash
# Usage: ./delete_branch.sh <branch-name>

if [ -z "$1" ]; then
  echo "Erreur : Vous devez spécifier un nom de branche."
  exit 1
fi

BRANCH_NAME=$1

# Sauvegarder le nom de la branche supprimée dans un fichier temporaire
echo $BRANCH_NAME > /tmp/last_deleted_branch

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