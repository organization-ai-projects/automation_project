#!/bin/bash
set -euo pipefail

# Usage: ./push_branch.sh
# Description: Push la branche courante vers le remote, refuse dev/main.

REMOTE="${REMOTE:-origin}"
PROTECTED_BRANCHES=("dev" "main")

BRANCH_NAME="$(git branch --show-current || true)"

if [[ -z "$BRANCH_NAME" ]]; then
  echo "Erreur : Aucune branche locale active (detached HEAD). Passe sur une branche et relance." >&2
  exit 1
fi

for b in "${PROTECTED_BRANCHES[@]}"; do
  if [[ "$BRANCH_NAME" == "$b" ]]; then
    echo "Erreur : push direct interdit vers '$b'." >&2
    exit 1
  fi
done

git fetch "$REMOTE" --prune

echo "=== Push branch: $BRANCH_NAME -> $REMOTE ==="

# Si upstream existe, push simple, sinon push -u
if git rev-parse --abbrev-ref --symbolic-full-name "@{u}" >/dev/null 2>&1; then
  git push "$REMOTE" "$BRANCH_NAME"
  echo "✓ Branche '$BRANCH_NAME' poussée sur '$REMOTE'."
else
  git push --set-upstream "$REMOTE" "$BRANCH_NAME"
  echo "✓ Branche '$BRANCH_NAME' poussée sur '$REMOTE' (upstream configuré)."
fi
