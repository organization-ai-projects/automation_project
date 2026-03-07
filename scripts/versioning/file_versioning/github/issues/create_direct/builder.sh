#!/usr/bin/env bash

create_direct_require_option_value() {
  local opt="$1"
  local value="${2:-}"
  issue_cli_require_option_value "$opt" "$value" die
}

build_issue_body() {
  local context="$1"
  local problem="$2"
  local parent="$3"
  shift 3
  local -a acceptance_criteria=("$@")

  local acceptance_block=""
  local criterion
  for criterion in "${acceptance_criteria[@]}"; do
    criterion="$(trim "$criterion")"
    [[ -n "$criterion" ]] || continue
    acceptance_block+="- [ ] $criterion"$'\n'
  done
  [[ -n "$acceptance_block" ]] || die "All --acceptance values are empty"

  cat <<EOF_BODY
## Context

$context

## Problem

$problem

## Acceptance Criteria

Done when :

${acceptance_block%$'\n'}

## Hierarchy

Parent: $parent
EOF_BODY
}

append_references_section_if_needed() {
  local body="$1"
  local -n related_issues_ref="$2"
  local -n related_prs_ref="$3"
  local -a related_issues_clean=()
  local -a related_prs_clean=()
  local ref

  for ref in "${related_issues_ref[@]}"; do
    ref="$(trim "$ref")"
    [[ -n "$ref" ]] && related_issues_clean+=("$ref")
  done
  for ref in "${related_prs_ref[@]}"; do
    ref="$(trim "$ref")"
    [[ -n "$ref" ]] && related_prs_clean+=("$ref")
  done

  if [[ ${#related_issues_clean[@]} -gt 0 || ${#related_prs_clean[@]} -gt 0 ]]; then
    body+=$'\n\n## References\n'
    if [[ ${#related_issues_clean[@]} -gt 0 ]]; then
      body+=$'\nRelated issue(s):'" $(printf '%s ' "${related_issues_clean[@]}")"
    fi
    if [[ ${#related_prs_clean[@]} -gt 0 ]]; then
      body+=$'\nRelated PR(s):'" $(printf '%s ' "${related_prs_clean[@]}")"
    fi
  fi

  printf '%s' "$body"
}

run_create_direct_issue() {
  local template_path=".github/ISSUE_TEMPLATE/direct_issue.md"
  local repo=""
  local title=""
  local context=""
  local problem=""
  local parent="none"
  local dry_run=false
  local -a acceptance_criteria=()
  local -a related_issues=()
  local -a related_prs=()
  local -a labels=()
  local -a assignees=()

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --title)
      create_direct_require_option_value "$1" "${2:-}"
      title="${2:-}"
      shift 2
      ;;
    --context)
      create_direct_require_option_value "$1" "${2:-}"
      context="${2:-}"
      shift 2
      ;;
    --problem)
      create_direct_require_option_value "$1" "${2:-}"
      problem="${2:-}"
      shift 2
      ;;
    --acceptance)
      create_direct_require_option_value "$1" "${2:-}"
      acceptance_criteria+=("${2:-}")
      shift 2
      ;;
    --parent)
      create_direct_require_option_value "$1" "${2:-}"
      parent="${2:-}"
      shift 2
      ;;
    --related-issue)
      create_direct_require_option_value "$1" "${2:-}"
      related_issues+=("${2:-}")
      shift 2
      ;;
    --related-pr)
      create_direct_require_option_value "$1" "${2:-}"
      related_prs+=("${2:-}")
      shift 2
      ;;
    --label)
      create_direct_require_option_value "$1" "${2:-}"
      labels+=("${2:-}")
      shift 2
      ;;
    --assignee)
      create_direct_require_option_value "$1" "${2:-}"
      assignees+=("${2:-}")
      shift 2
      ;;
    --repo)
      create_direct_require_option_value "$1" "${2:-}"
      repo="${2:-}"
      shift 2
      ;;
    --template)
      create_direct_require_option_value "$1" "${2:-}"
      template_path="${2:-}"
      shift 2
      ;;
    --dry-run)
      dry_run=true
      shift
      ;;
    -h | --help)
      usage
      exit 0
      ;;
    *) die "Unknown option: $1" ;;
    esac
  done

  [[ -n "$(trim "$title")" ]] || die "--title is required"
  [[ -n "$(trim "$context")" ]] || die "--context is required"
  [[ -n "$(trim "$problem")" ]] || die "--problem is required"
  [[ ${#acceptance_criteria[@]} -gt 0 ]] || die "At least one --acceptance is required"

  local title_validation
  title_validation="$(issue_validate_title "$title" || true)"
  if [[ -n "$title_validation" ]]; then
    die "Invalid --title. Expected conventional issue format (e.g. feat(scope): summary)"
  fi

  require_template_contract "$template_path"

  local body
  body="$(build_issue_body "$context" "$problem" "$parent" "${acceptance_criteria[@]}")"
  body="$(append_references_section_if_needed "$body" related_issues related_prs)"

  local body_validation
  body_validation="$(issue_validate_body "$body" || true)"
  if [[ -n "$body_validation" ]]; then
    echo "Issue body validation failed against required-fields contract:" >&2
    while IFS='|' read -r kind _ message; do
      [[ -z "$message" ]] && continue
      echo " - [${kind}] ${message}" >&2
    done <<<"$body_validation"
    die "Issue body is non-compliant with required issue format."
  fi

  local -a cmd=(gh issue create --title "$title" --body "$body")
  local label
  local assignee

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
    return 0
  fi

  "${cmd[@]}"
}
