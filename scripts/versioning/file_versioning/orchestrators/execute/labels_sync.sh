#!/usr/bin/env bash
set -euo pipefail

# Sync GitHub labels from a labels.json file
# Usage: ./labels_sync.sh [labels-file]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/core/command.sh
source "$ROOT_DIR/scripts/common_lib/core/command.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"

require_git_repo
require_cmd gh
require_cmd jq

LABELS_FILE="${1:-$ROOT_DIR/.github/labels.json}"

if [[ ! -f "$LABELS_FILE" ]]; then
  die "Labels file not found: $LABELS_FILE"
fi

info "Syncing labels from: $LABELS_FILE"

# Read and process each label
jq -c '.[]' "$LABELS_FILE" | while read -r label; do
  NAME=$(echo "$label" | jq -r '.name')
  COLOR=$(echo "$label" | jq -r '.color')
  DESCRIPTION=$(echo "$label" | jq -r '.description // ""')

  info "Processing label: $NAME"

  # Check if label exists
  if gh label list --json name --jq ".[].name" | grep -qx "$NAME"; then
    info "  Updating existing label..."
    gh label edit "$NAME" --color "$COLOR" --description "$DESCRIPTION" || warn "Failed to update $NAME"
  else
    info "  Creating new label..."
    gh label create "$NAME" --color "$COLOR" --description "$DESCRIPTION" || warn "Failed to create $NAME"
  fi
done

info "✅ Label sync complete."
