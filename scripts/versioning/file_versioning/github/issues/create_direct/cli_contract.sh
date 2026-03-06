#!/usr/bin/env bash

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
  done <<<"${ISSUE_REQUIRED_SECTIONS:-}"

  local rule field_name
  while IFS= read -r rule; do
    [[ -z "$rule" ]] && continue
    IFS=$'\t' read -r field_name _ _ <<<"$rule"
    field_name="$(trim_whitespace "${field_name:-}")"
    [[ -z "$field_name" ]] && continue
    grep -q "^${field_name}:$" "$template_path" || die "Template missing required field line: ${field_name}:"
  done <<<"${ISSUE_REQUIRED_FIELDS:-}"
}
