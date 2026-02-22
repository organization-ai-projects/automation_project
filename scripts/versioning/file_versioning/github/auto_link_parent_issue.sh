#!/usr/bin/env bash

set -euo pipefail

# shellcheck source=scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh
source "$(git rev-parse --show-toplevel)/scripts/common_lib/versioning/file_versioning/github/issue_helpers.sh"

usage() {
  cat <<USAGE
Usage:
  $0 --issue ISSUE_NUMBER

Notes:
  - Reads "Parent: #<number>" or "Parent: none" from issue body.
  - Attempts to link child -> parent as GitHub sub-issue via GraphQL.
  - On invalid input or API linking failure, posts actionable status comment and labels issue.
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

trim() {
  local s="${1:-}"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf "%s" "$s"
}

extract_parent_field_value() {
  local body="${1:-}"
  awk '
    BEGIN { IGNORECASE = 1 }
    /^[[:space:]]*Parent[[:space:]]*:/ {
      line = $0
      sub(/^[[:space:]]*Parent[[:space:]]*:[[:space:]]*/, "", line)
      print line
      exit
    }
  ' <<< "$body"
}

issue_arg=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --issue)
      issue_arg="${2:-}"
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

if [[ -z "$issue_arg" ]]; then
  echo "Erreur: --issue est requis." >&2
  usage >&2
  exit 2
fi

require_number "--issue" "$issue_arg"

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
ISSUE_NUMBER="$issue_arg"

MARKER="<!-- parent-field-autolink:${ISSUE_NUMBER} -->"
LABEL_INVALID="invalid"
LABEL_AUTOMATION_FAILED="automation-failed"

add_label() {
  local issue_number="$1"
  local label="$2"
  gh api "repos/${REPO_NAME}/issues/${issue_number}/labels" \
    -f labels[]="$label" >/dev/null 2>&1 || true
}

remove_label() {
  local issue_number="$1"
  local label="$2"
  gh api -X DELETE "repos/${REPO_NAME}/issues/${issue_number}/labels/${label}" >/dev/null 2>&1 || true
}

set_error_state() {
  local message="$1"
  local help_text="$2"
  local body
  body="$MARKER
### Parent Field Autolink Status

❌ $message

$help_text
"
  add_label "$ISSUE_NUMBER" "$LABEL_INVALID"
  add_label "$ISSUE_NUMBER" "$LABEL_AUTOMATION_FAILED"
  github_issue_upsert_marker_comment "$REPO_NAME" "$ISSUE_NUMBER" "$MARKER" "$body"
}

set_success_state() {
  local message="$1"
  local body
  body="$MARKER
### Parent Field Autolink Status

✅ $message
"
  remove_label "$ISSUE_NUMBER" "$LABEL_INVALID"
  remove_label "$ISSUE_NUMBER" "$LABEL_AUTOMATION_FAILED"
  github_issue_upsert_marker_comment "$REPO_NAME" "$ISSUE_NUMBER" "$MARKER" "$body"
}

issue_json="$(gh issue view "$ISSUE_NUMBER" -R "$REPO_NAME" --json number,title,body,state,url 2>/dev/null || true)"
if [[ -z "$issue_json" ]]; then
  echo "Erreur: impossible de lire l'issue #${ISSUE_NUMBER}." >&2
  exit 4
fi

issue_body="$(echo "$issue_json" | jq -r '.body // ""')"
parent_raw="$(extract_parent_field_value "$issue_body")"
parent_raw="$(trim "${parent_raw:-}")"

if [[ -z "$parent_raw" ]]; then
  set_error_state \
    "Missing required field \`Parent:\` in issue body." \
    "Expected format:
\n- \`Parent: #<issue_number>\` for child issues
\n- \`Parent: none\` for root/parent issues"
  exit 0
fi

parent_raw_lc="$(echo "$parent_raw" | tr '[:upper:]' '[:lower:]')"
if [[ "$parent_raw_lc" == "none" ]]; then
  set_success_state "No parent linking requested (\`Parent: none\`)."
  exit 0
fi

if [[ ! "$parent_raw" =~ ^#[0-9]+$ ]]; then
  set_error_state \
    "Invalid \`Parent:\` value: \`${parent_raw}\`." \
    "Expected \`Parent: #<issue_number>\` or \`Parent: none\`."
  exit 0
fi

parent_number="${parent_raw//#/}"
if [[ "$parent_number" == "$ISSUE_NUMBER" ]]; then
  set_error_state \
    "Issue cannot reference itself as parent (\`Parent: #${ISSUE_NUMBER}\`)." \
    "Use another parent issue number or \`Parent: none\`."
  exit 0
fi

parent_json="$(gh issue view "$parent_number" -R "$REPO_NAME" --json number,title,state,url 2>/dev/null || true)"
if [[ -z "$parent_json" ]]; then
  set_error_state \
    "Parent issue \`#${parent_number}\` was not found." \
    "Use an existing issue number in \`Parent:\`."
  exit 0
fi

parent_state="$(echo "$parent_json" | jq -r '.state // ""')"
if [[ "$parent_state" != "OPEN" ]]; then
  set_error_state \
    "Parent issue \`#${parent_number}\` is not open (state: ${parent_state})." \
    "Reopen the parent or choose another open parent issue."
  exit 0
fi

relation_json="$(gh api graphql \
  -f query='query($owner:String!,$name:String!,$child:Int!,$parent:Int!){repository(owner:$owner,name:$name){child:issue(number:$child){id parent{number id}} parent:issue(number:$parent){id state}}}' \
  -f owner="$REPO_OWNER" \
  -f name="$REPO_SHORT_NAME" \
  -F child="$ISSUE_NUMBER" \
  -F parent="$parent_number" 2>/dev/null || true)"

if [[ -z "$relation_json" ]]; then
  set_error_state \
    "Unable to query parent/child relation state from GitHub API." \
    "Retry later. If this persists, link the issue manually in GitHub UI."
  exit 0
fi

current_parent_number="$(echo "$relation_json" | jq -r '.data.repository.child.parent.number // empty')"
child_node_id="$(echo "$relation_json" | jq -r '.data.repository.child.id // empty')"
parent_node_id="$(echo "$relation_json" | jq -r '.data.repository.parent.id // empty')"

if [[ -n "$current_parent_number" && "$current_parent_number" == "$parent_number" ]]; then
  set_success_state "Issue already linked to parent #${parent_number}."
  exit 0
fi

if [[ -n "$current_parent_number" && "$current_parent_number" != "$parent_number" ]]; then
  set_error_state \
    "Issue is already linked to another parent (\`#${current_parent_number}\`)." \
    "Please update parent linkage manually in GitHub UI before retrying automation."
  exit 0
fi

if [[ -z "$child_node_id" || -z "$parent_node_id" ]]; then
  set_error_state \
    "Missing GitHub node IDs required for sub-issue linking." \
    "Retry later. If this persists, link parent/child manually in GitHub UI."
  exit 0
fi

link_result="$(gh api graphql \
  -f query='mutation($issueId:ID!,$subIssueId:ID!){addSubIssue(input:{issueId:$issueId,subIssueId:$subIssueId}){issue{subIssues(first:1){nodes{number}}}}}' \
  -f issueId="$parent_node_id" \
  -f subIssueId="$child_node_id" 2>/dev/null || true)"

if [[ -z "$link_result" ]]; then
  set_error_state \
    "GitHub API mutation failed while linking child to parent." \
    "Link manually in GitHub UI, then keep \`Parent: #${parent_number}\` in issue body for traceability."
  exit 0
fi

set_success_state "Linked this issue as child of #${parent_number}."
echo "Linked issue #${ISSUE_NUMBER} to parent #${parent_number}."
