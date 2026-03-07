#!/usr/bin/env bash
# shellcheck disable=SC2034

parent_guard_usage() {
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

parent_guard_require_number() {
  local name="$1"
  local value="${2:-}"
  if [[ ! "$value" =~ ^[0-9]+$ ]]; then
    echo "Erreur: ${name} doit être un numéro d'issue." >&2
    exit 2
  fi
}

parent_guard_parse_args() {
  local issue_var_name="$1"
  local child_var_name="$2"
  local strict_guard_var_name="$3"
  shift 3

  local -n issue_ref="$issue_var_name"
  local -n child_ref="$child_var_name"
  local -n strict_guard_ref="$strict_guard_var_name"

  issue_ref=""
  child_ref=""
  strict_guard_ref="true"

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --issue)
      issue_ref="${2:-}"
      shift 2
      ;;
    --child)
      child_ref="${2:-}"
      shift 2
      ;;
    --strict-guard)
      strict_guard_ref="${2:-}"
      shift 2
      ;;
    -h | --help)
      parent_guard_usage
      exit 0
      ;;
    *)
      echo "Erreur: option inconnue: $1" >&2
      parent_guard_usage >&2
      exit 2
      ;;
    esac
  done
}

parent_guard_validate_args() {
  local issue_arg="$1"
  local child_arg="$2"
  local strict_guard="$3"

  if [[ -z "$issue_arg" && -z "$child_arg" ]]; then
    echo "Erreur: --issue ou --child est requis." >&2
    parent_guard_usage >&2
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
}
