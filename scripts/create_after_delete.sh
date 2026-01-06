#!/bin/bash
# Usage: ./create_after_delete.sh

# Récupérer le nom de la branche locale actuelle
BRANCH_NAME=$(git branch --show-current)

# Vérifier si une branche est active
if [ -z "$BRANCH_NAME" ]; then
  echo "Erreur : Aucune branche locale active. Vous devez être sur une branche pour l'utiliser."
  exit 1
fi

# Supprimer la branche locale
if git branch --list $BRANCH_NAME > /dev/null 2>&1; then
  git branch -d $BRANCH_NAME
  if [ $? -eq 0 ]; then
    echo "Branche locale $BRANCH_NAME supprimée."
  else
    echo "Erreur : Impossible de supprimer la branche locale $BRANCH_NAME."
    exit 1
  fi
else
  echo "La branche locale $BRANCH_NAME n'existe pas."
fi

# Supprimer la branche distante si elle existe
if git ls-remote --exit-code origin $BRANCH_NAME > /dev/null 2>&1; then
  git push origin --delete $BRANCH_NAME
  if [ $? -eq 0 ]; then
    echo "Branche distante $BRANCH_NAME supprimée."
  else
    echo "Erreur : Impossible de supprimer la branche distante $BRANCH_NAME."
  fi
else
  echo "La branche distante $BRANCH_NAME n'existe pas."
fi

# Créer la branche à partir de dev
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
  echo "Branche $BRANCH_NAME recréée avec succès."
else
  echo "Erreur : Impossible de recréer la branche $BRANCH_NAME."
fi