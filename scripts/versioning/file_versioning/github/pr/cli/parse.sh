#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# CLI parsing helpers.

pr_args_assign_value() {
  local option_name="$1"
  local option_value="$2"
  local target_var_name="$3"

  pr_require_option_value "$option_name" "$option_value"
  printf -v "$target_var_name" '%s' "$option_value"
}

pr_args_parse_cli() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
    --keep-artifacts)
      keep_artifacts="true"
      shift
      ;;
    --dry-run)
      dry_run="true"
      mode_explicit="true"
      shift
      ;;
    --base)
      pr_args_assign_value "--base" "${2:-}" base_ref
      shift 2
      ;;
    --head)
      pr_args_assign_value "--head" "${2:-}" head_ref
      shift 2
      ;;
    --create-pr)
      create_pr="true"
      shift
      ;;
    --allow-partial-create)
      allow_partial_create="true"
      shift
      ;;
    --yes)
      assume_yes="true"
      shift
      ;;
    --auto)
      auto_mode="true"
      mode_explicit="true"
      shift
      ;;
    --auto-edit)
      pr_args_assign_value "--auto-edit" "${2:-}" auto_edit_pr_number
      mode_explicit="true"
      shift 2
      ;;
    --refresh-pr)
      pr_require_option_value "--refresh-pr" "${2:-}"
      if [[ -n "$auto_edit_pr_number" && "$auto_edit_pr_number" != "${2:-}" ]]; then
        pr_usage_error "--refresh-pr and --auto-edit must target the same PR_NUMBER."
      fi
      auto_edit_pr_number="${2:-}"
      refresh_pr_used="true"
      mode_explicit="true"
      shift 2
      ;;
    --duplicate-mode)
      pr_args_assign_value "--duplicate-mode" "${2:-}" duplicate_mode
      shift 2
      ;;
    --debug)
      debug_mode="true"
      shift
      ;;
    --validation-only)
      validation_only="true"
      shift
      ;;
    -h | --help)
      pr_print_help
      exit 0
      ;;
    *)
      positionals+=("$1")
      shift
      ;;
    esac
  done
}
