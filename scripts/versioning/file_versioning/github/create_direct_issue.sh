#!/usr/bin/env bash

set -euo pipefail

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
  grep -q '^## Context$' "$template_path" || die "Template missing section: ## Context"
  grep -q '^## Problem$' "$template_path" || die "Template missing section: ## Problem"
  grep -q '^## Acceptance Criteria$' "$template_path" || die "Template missing section: ## Acceptance Criteria"
  grep -q '^## Hierarchy$' "$template_path" || die "Template missing section: ## Hierarchy"
  grep -q '^Parent:$' "$template_path" || die "Template missing required Parent field line: Parent:"
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

if [[ ! "$parent" =~ ^(none|#[0-9]+)$ ]]; then
  die "--parent must be 'none' or '#<issue_number>'"
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
