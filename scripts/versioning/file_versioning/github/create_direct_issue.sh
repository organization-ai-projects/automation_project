#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck disable=SC1091
source "${SCRIPT_DIR}/lib/issue_required_fields.sh"

usage() {
  cat <<'USAGE'
Usage:
  create_direct_issue.sh \
    --title "type(scope): summary" \
    --context "Why this exists" \
    --problem "What is wrong" \
    --acceptance "criterion 1" \
    [--acceptance "criterion 2"] \
    [--parent none|#123] \
    [--related-issue "#456"] \
    [--related-pr "#789"] \
    [--label "bug"] \
    [--assignee "octocat"] \
    [--repo "owner/name"] \
    [--template ".github/ISSUE_TEMPLATE/direct_issue.md"] \
    [--dry-run]

Notes:
  - Uses the direct issue template as canonical structure contract.
  - Enforces required Parent format: none or #<issue_number>.
USAGE
}

die() {
  echo "Error: $*" >&2
  exit 2
}

trim() {
  local s="${1:-}"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf "%s" "$s"
}

require_template_contract() {
  local template_path="$1"
  [[ -f "$template_path" ]] || die "Template not found: $template_path"
  issue_contract_load || die "Unable to load issue contract"

  local section
  while IFS= read -r section; do
    section="$(trim_whitespace "$section")"
    [[ -z "$section" ]] && continue
    grep -qF "$section" "$template_path" || die "Template missing section: ${section}"
  done <<< "${ISSUE_REQUIRED_SECTIONS:-}"

  local rule field_name
  while IFS= read -r rule; do
    [[ -z "$rule" ]] && continue
    IFS=$'\t' read -r field_name _ _ <<< "$rule"
    field_name="$(trim_whitespace "${field_name:-}")"
    [[ -z "$field_name" ]] && continue
    grep -q "^${field_name}:$" "$template_path" || die "Template missing required field line: ${field_name}:"
  done <<< "${ISSUE_REQUIRED_FIELDS:-}"
}

template_path=".github/ISSUE_TEMPLATE/direct_issue.md"
repo=""
title=""
context=""
problem=""
parent="none"
dry_run=false
declare -a acceptance_criteria=()
declare -a related_issues=()
declare -a related_prs=()
declare -a labels=()
declare -a assignees=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    --title) title="${2:-}"; shift 2 ;;
    --context) context="${2:-}"; shift 2 ;;
    --problem) problem="${2:-}"; shift 2 ;;
    --acceptance) acceptance_criteria+=("${2:-}"); shift 2 ;;
    --parent) parent="${2:-}"; shift 2 ;;
    --related-issue) related_issues+=("${2:-}"); shift 2 ;;
    --related-pr) related_prs+=("${2:-}"); shift 2 ;;
    --label) labels+=("${2:-}"); shift 2 ;;
    --assignee) assignees+=("${2:-}"); shift 2 ;;
    --repo) repo="${2:-}"; shift 2 ;;
    --template) template_path="${2:-}"; shift 2 ;;
    --dry-run) dry_run=true; shift ;;
    -h|--help) usage; exit 0 ;;
    *) die "Unknown option: $1" ;;
  esac
done

[[ -n "$(trim "$title")" ]] || die "--title is required"
[[ -n "$(trim "$context")" ]] || die "--context is required"
[[ -n "$(trim "$problem")" ]] || die "--problem is required"
[[ ${#acceptance_criteria[@]} -gt 0 ]] || die "At least one --acceptance is required"

title_validation="$(issue_validate_title "$title" || true)"
if [[ -n "$title_validation" ]]; then
  die "Invalid --title. Expected conventional issue format (e.g. feat(scope): summary)"
fi

require_template_contract "$template_path"

acceptance_block=""
for criterion in "${acceptance_criteria[@]}"; do
  criterion="$(trim "$criterion")"
  [[ -n "$criterion" ]] || continue
  acceptance_block+="- [ ] $criterion"$'\n'
done
[[ -n "$acceptance_block" ]] || die "All --acceptance values are empty"

body=$(
  cat <<EOF
## Context

$context

## Problem

$problem

## Acceptance Criteria

Done when :

${acceptance_block%$'\n'}

## Hierarchy

Parent: $parent
EOF
)

if [[ ${#related_issues[@]} -gt 0 || ${#related_prs[@]} -gt 0 ]]; then
  body+=$'\n\n## References\n'
  if [[ ${#related_issues[@]} -gt 0 ]]; then
    body+=$'\nRelated issue(s):'" $(printf '%s ' "${related_issues[@]}")"
  fi
  if [[ ${#related_prs[@]} -gt 0 ]]; then
    body+=$'\nRelated PR(s):'" $(printf '%s ' "${related_prs[@]}")"
  fi
fi

body_validation="$(issue_validate_body "$body" || true)"
if [[ -n "$body_validation" ]]; then
  echo "Issue body validation failed against required-fields contract:" >&2
  while IFS='|' read -r kind field message; do
    [[ -z "$message" ]] && continue
    echo " - [${kind}] ${message}" >&2
  done <<< "$body_validation"
  die "Issue body is non-compliant with required issue format."
fi

cmd=(gh issue create --title "$title" --body "$body")
if [[ -n "$repo" ]]; then
  cmd+=(-R "$repo")
fi
for label in "${labels[@]}"; do
  cmd+=(--label "$label")
done
for assignee in "${assignees[@]}"; do
  cmd+=(--assignee "$assignee")
done

if [[ "$dry_run" == "true" ]]; then
  echo "Dry-run mode. Issue was not created."
  echo "Template contract: $template_path"
  echo "----- title -----"
  echo "$title"
  echo "----- body -----"
  echo "$body"
  exit 0
fi

"${cmd[@]}"
