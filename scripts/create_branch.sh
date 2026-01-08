#!/bin/bash
set -euo pipefail

# Usage: ./create_branch.sh <branch-name>
# If branch-name is omitted, tries to reuse /tmp/last_deleted_branch.
# Creates branch from dev and optionally pushes it.

REMOTE="origin"
BASE_BRANCH="dev"
LAST_DELETED_FILE="/tmp/last_deleted_branch"

BRANCH_NAME="${1:-}"

if [[ -z "$BRANCH_NAME" ]]; then
  if [[ -f "$LAST_DELETED_FILE" ]]; then
    BRANCH_NAME="$(tr -d '\r' < "$LAST_DELETED_FILE" | head -n 1 | xargs || true)"
    if [[ -z "$BRANCH_NAME" ]]; then
      echo "Erreur : $LAST_DELETED_FILE est vide ou invalide." >&2
      exit 1
    fi
    echo "Aucun nom fourni. Recréation de la dernière branche supprimée : $BRANCH_NAME"
  else
    echo "Erreur : vous devez spécifier un nom de branche (ou $LAST_DELETED_FILE n'existe pas)." >&2
    exit 1
  fi
fi

# Protections basiques
if [[ "$BRANCH_NAME" == "$BASE_BRANCH" || "$BRANCH_NAME" == "main" ]]; then
  echo "Erreur : nom de branche protégé/refusé: $BRANCH_NAME" >&2
  exit 1
fi

# Refuse spaces (optional but sane)
if [[ "$BRANCH_NAME" == *" "* ]]; then
  echo "Erreur : nom de branche invalide (contient des espaces): '$BRANCH_NAME'" >&2
  exit 1
fi

echo "=== Create branch: $BRANCH_NAME (base: $BASE_BRANCH) ==="

git fetch "$REMOTE" --prune

git checkout "$BASE_BRANCH"
git pull "$REMOTE" "$BASE_BRANCH"

# If branch already exists locally, just checkout it (or refuse; your choice)
if git show-ref --verify --quiet "refs/heads/$BRANCH_NAME"; then
  echo "ℹ La branche locale '$BRANCH_NAME' existe déjà. Checkout."
  git checkout "$BRANCH_NAME"
else
  git checkout -b "$BRANCH_NAME" "$BASE_BRANCH"
  echo "✓ Branche '$BRANCH_NAME' créée depuis '$BASE_BRANCH'."
fi

# Optional: push + upstream (highly recommended)
git push --set-upstream "$REMOTE" "$BRANCH_NAME"
echo "✓ Branche '$BRANCH_NAME' poussée sur '$REMOTE' avec upstream."
