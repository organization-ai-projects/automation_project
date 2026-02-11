#!/usr/bin/env bash
set -euo pipefail

# Sync GitHub labels from a labels.json file
# Usage: ./labels_sync.sh [--prune] [labels-file]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../../../.." && pwd)"

# shellcheck source=scripts/common_lib/core/logging.sh
source "$ROOT_DIR/scripts/common_lib/core/logging.sh"
# shellcheck source=scripts/common_lib/core/command.sh
source "$ROOT_DIR/scripts/common_lib/core/command.sh"
# shellcheck source=scripts/common_lib/versioning/file_versioning/git/repo.sh
source "$ROOT_DIR/scripts/common_lib/versioning/file_versioning/git/repo.sh"

require_git_repo
require_cmd gh
require_cmd jq

PRUNE=false
LABELS_FILE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --prune)
      PRUNE=true
      shift
      ;;
    -h|--help)
      echo "Usage: ./labels_sync.sh [--prune] [labels-file]"
      exit 0
      ;;
    *)
      if [[ -n "$LABELS_FILE" ]]; then
        die "Unexpected argument: $1"
      fi
      LABELS_FILE="$1"
      shift
      ;;
  esac
done

LABELS_FILE="${LABELS_FILE:-$ROOT_DIR/.github/labels.json}"

if [[ ! -f "$LABELS_FILE" ]]; then
  die "Labels file not found: $LABELS_FILE"
fi

info "Syncing labels from: $LABELS_FILE"

has_label() {
  local needle="$1"
  shift
  local label
  for label in "$@"; do
    if [[ "$label" == "$needle" ]]; then
      return 0
    fi
  done
  return 1
}

mapfile -t existing_labels < <(gh label list --limit 1000 --json name --jq '.[].name')

# Read and process each label
jq -c '.[]' "$LABELS_FILE" | while read -r label; do
  NAME=$(echo "$label" | jq -r '.name')
  COLOR=$(echo "$label" | jq -r '.color')
  DESCRIPTION=$(echo "$label" | jq -r '.description // ""')

  info "Processing label: $NAME"

  # Check if label exists
  if has_label "$NAME" "${existing_labels[@]}"; then
    info "  Updating existing label..."
    gh label edit "$NAME" --color "$COLOR" --description "$DESCRIPTION" || warn "Failed to update $NAME"
  else
    info "  Creating new label..."
    if gh label create "$NAME" --color "$COLOR" --description "$DESCRIPTION"; then
      existing_labels+=("$NAME")
    else
      warn "Failed to create $NAME"
    fi
  fi
done

if [[ "$PRUNE" == true ]]; then
  info "Pruning labels that are not present in labels.json..."
  desired_labels_file="$(mktemp)"
  trap 'rm -f "$desired_labels_file"' EXIT
  jq -r '.[].name' "$LABELS_FILE" | sort -u > "$desired_labels_file"

  # Built-in labels to keep even if absent from labels.json.
  # Override with LABELS_SYNC_PROTECTED_LABELS (comma-separated),
  # or set LABELS_SYNC_PROTECTED_LABELS="" to disable protection.
  protected_labels=()
  if [[ -n "${LABELS_SYNC_PROTECTED_LABELS+x}" ]]; then
    IFS=',' read -r -a protected_labels <<< "${LABELS_SYNC_PROTECTED_LABELS}"
  else
    protected_labels=(
      "bug"
      "documentation"
      "duplicate"
      "enhancement"
      "good first issue"
      "help wanted"
      "invalid"
      "question"
      "wontfix"
    )
  fi

  mapfile -t repo_labels < <(gh label list --limit 1000 --json name --jq '.[].name')
  for repo_label in "${repo_labels[@]}"; do
    if grep -Fqx "$repo_label" "$desired_labels_file"; then
      continue
    fi
    if has_label "$repo_label" "${protected_labels[@]}"; then
      info "  Keeping protected label: $repo_label"
      continue
    fi
    info "  Deleting obsolete label: $repo_label"
    gh label delete "$repo_label" --yes || warn "Failed to delete $repo_label"
  done
fi

info "✅ Label sync complete."
