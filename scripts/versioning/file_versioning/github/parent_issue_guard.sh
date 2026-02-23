#!/usr/bin/env bash

set -euo pipefail

# shellcheck source=scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh"

usage() {
  cat <<USAGE
Usage:
  $0 --issue ISSUE_NUMBER [--strict-guard true|false]
  $0 --child ISSUE_NUMBER [--strict-guard true|false]

Notes:
  - --issue: evaluate one parent issue candidate directly.
  - --child: search and evaluate parent candidates referencing the child issue.
  - strict guard: when true, a closed parent with open required children is reopened.
USAGE
}

require_number() {
  local name="$1"
  local value="${2:-}"
  if [[ ! "$value" =~ ^[0-9]+$ ]]; then
    echo "Erreur: ${name} doit être un numéro d'issue." >&2
    exit 2
  fi
}

issue_arg=""
child_arg=""
strict_guard="true"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --issue)
      issue_arg="${2:-}"
      shift 2
      ;;
    --child)
      child_arg="${2:-}"
      shift 2
      ;;
    --strict-guard)
      strict_guard="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Erreur: option inconnue: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$issue_arg" && -z "$child_arg" ]]; then
  echo "Erreur: --issue ou --child est requis." >&2
  usage >&2
  exit 2
fi

if [[ -n "$issue_arg" && -n "$child_arg" ]]; then
  echo "Erreur: utiliser --issue ou --child, pas les deux en même temps." >&2
  exit 2
fi

if [[ "$strict_guard" != "true" && "$strict_guard" != "false" ]]; then
  echo "Erreur: --strict-guard doit être true ou false." >&2
  exit 2
fi

if ! command -v gh >/dev/null 2>&1; then
  echo "Erreur: gh est requis." >&2
  exit 3
fi
if ! command -v jq >/dev/null 2>&1; then
  echo "Erreur: jq est requis." >&2
  exit 3
fi

REPO_NAME="${GH_REPO:-}"
if [[ -z "$REPO_NAME" ]]; then
  REPO_NAME="$(gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true)"
fi
if [[ -z "$REPO_NAME" ]]; then
  echo "Erreur: impossible de déterminer le repository (GH_REPO)." >&2
  exit 3
fi
REPO_OWNER="${REPO_NAME%%/*}"
REPO_SHORT_NAME="${REPO_NAME#*/}"

extract_parent_ref_from_github() {
  local child_number="$1"
  gh api graphql \
    -f query='query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){issue(number:$number){parent{number}}}}' \
    -f owner="$REPO_OWNER" \
    -f name="$REPO_SHORT_NAME" \
    -F number="$child_number" \
    --jq '.data.repository.issue.parent.number // empty | "#"+tostring' 2>/dev/null || true
}

build_status_comment() {
  local parent_number="$1"
  local parent_state="$2"
  local total="$3"
  local closed_count="$4"
  local open_count="$5"
  local open_lines="$6"
  local marker="<!-- parent-issue-status:${parent_number} -->"

  echo "$marker"
  echo "### Parent Issue Status"
  echo "Parent: #${parent_number}"
  echo ""
  echo "- Required children detected: ${total}"
  echo "- Closed: ${closed_count}"
  echo "- Open: ${open_count}"
  echo ""

  if [[ "$open_count" -eq 0 ]]; then
    echo "All required child issues are closed. This parent can be closed."
  else
    echo "Some required child issues are still open:"
    echo "$open_lines"
    if [[ "$parent_state" == "CLOSED" && "$strict_guard" == "true" ]]; then
      echo ""
      echo "Guard action: parent was reopened because required children are still open."
    fi
  fi
}

evaluate_parent_issue() {
  local parent_number="$1"

  local issue_json
  issue_json="$(gh issue view "$parent_number" -R "$REPO_NAME" --json number,title,body,state,url 2>/dev/null || true)"
  if [[ -z "$issue_json" ]]; then
    return
  fi

  local parent_state
  parent_state="$(echo "$issue_json" | jq -r '.state')"

  local body
  body="$(echo "$issue_json" | jq -r '.body // ""')"

  mapfile -t child_refs < <(github_issue_extract_subissue_refs "$REPO_OWNER" "$REPO_SHORT_NAME" "$parent_number")
  if [[ ${#child_refs[@]} -eq 0 ]]; then
    mapfile -t child_refs < <(github_issue_extract_tasklist_refs "$body")
  fi
  if [[ ${#child_refs[@]} -eq 0 ]]; then
    return
  fi

  local total="${#child_refs[@]}"
  local closed_count=0
  local open_count=0
  local open_lines=""

  for child_ref in "${child_refs[@]}"; do
    local child_number="${child_ref//#/}"
    local child_json
    child_json="$(gh issue view "$child_number" -R "$REPO_NAME" --json number,title,state,url 2>/dev/null || true)"
    if [[ -z "$child_json" ]]; then
      open_count=$((open_count + 1))
      open_lines+="- ${child_ref} (unreadable or missing)"$'\n'
      continue
    fi

    local child_state child_title
    child_state="$(echo "$child_json" | jq -r '.state')"
    child_title="$(echo "$child_json" | jq -r '.title')"

    if [[ "$child_state" == "CLOSED" ]]; then
      closed_count=$((closed_count + 1))
    else
      open_count=$((open_count + 1))
      open_lines+="- ${child_ref} ${child_title}"$'\n'
    fi
  done

  local comment_body
  comment_body="$(build_status_comment "$parent_number" "$parent_state" "$total" "$closed_count" "$open_count" "$open_lines")"
  github_issue_upsert_marker_comment \
    "$REPO_NAME" \
    "$parent_number" \
    "<!-- parent-issue-status:${parent_number} -->" \
    "$comment_body" \
    "true"

  if [[ "$open_count" -eq 0 && "$parent_state" == "OPEN" ]]; then
    gh issue close "$parent_number" -R "$REPO_NAME" \
      --comment "All required child issues are closed. Auto-closed by parent-issue-guard." >/dev/null
    echo "Closed parent issue #${parent_number} because all required children are closed."
  fi

  if [[ "$strict_guard" == "true" && "$parent_state" == "CLOSED" && "$open_count" -gt 0 ]]; then
    gh issue reopen "$parent_number" -R "$REPO_NAME" >/dev/null
    echo "Reopened parent issue #${parent_number} due to open required children."
  fi
}

if [[ -n "$issue_arg" ]]; then
  require_number "--issue" "$issue_arg"
  evaluate_parent_issue "$issue_arg"
  exit 0
fi

require_number "--child" "$child_arg"

mapfile -t parent_candidates < <(extract_parent_ref_from_github "$child_arg" | sed 's/^#//')
if [[ ${#parent_candidates[@]} -eq 0 ]]; then
  mapfile -t parent_candidates < <(
    gh api "search/issues" \
      -f q="repo:${REPO_NAME} is:issue \"#${child_arg}\"" \
      --jq '.items[].number' 2>/dev/null | sort -u
  )
fi

for parent_number in "${parent_candidates[@]}"; do
  [[ -z "$parent_number" ]] && continue
  if [[ "$parent_number" == "$child_arg" ]]; then
    continue
  fi
  evaluate_parent_issue "$parent_number"
done
