#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154


# CLI parsing/validation helpers for generate_pr_description.sh.

pr_args_init_defaults() {
  main_pr_number=""
  output_file="pr_description.md"
  output_file_explicit="false"
  write_body_to_file="true"
  keep_artifacts="false"
  dry_run="false"
  base_ref=""
  head_ref=""
  create_pr="false"
  allow_partial_create="false"
  assume_yes="false"
  auto_mode="false"
  mode_explicit="false"
  auto_edit_pr_number=""
  refresh_pr_used="false"
  debug_mode="false"
  duplicate_mode=""
  repo_name_with_owner_cache=""
  validation_only="false"
  allow_inferred_directive_conflicts="false"
  positionals=()
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
      pr_require_option_value "--base" "${2:-}"
      base_ref="${2:-}"
      shift 2
      ;;
    --head)
      pr_require_option_value "--head" "${2:-}"
      head_ref="${2:-}"
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
      pr_require_option_value "--auto-edit" "${2:-}"
      auto_edit_pr_number="${2:-}"
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
      pr_require_option_value "--duplicate-mode" "${2:-}"
      duplicate_mode="${2:-}"
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

pr_args_finalize() {
  if [[ "$mode_explicit" != "true" && ${#positionals[@]} -eq 0 ]]; then
    auto_mode="true"
  fi

  if [[ "$auto_mode" == "true" ]]; then
    dry_run="true"
    create_pr="true"
    if [[ ${#positionals[@]} -gt 0 ]]; then
      pr_usage_error "--auto does not accept a positional OUTPUT_FILE."
    fi
  fi

  if [[ -n "$auto_edit_pr_number" ]] && [[ ! "$auto_edit_pr_number" =~ ^[0-9]+$ ]]; then
    if [[ "$refresh_pr_used" == "true" ]]; then
      pr_usage_error "--refresh-pr requires a numeric PR_NUMBER."
    fi
    pr_usage_error "--auto-edit requires a numeric PR_NUMBER."
  fi

  if [[ "$validation_only" == "true" && -z "$auto_edit_pr_number" ]]; then
    pr_usage_error "--validation-only requires --auto-edit/--refresh-pr."
  fi

  if [[ -n "$duplicate_mode" ]] && [[ "$duplicate_mode" != "safe" && "$duplicate_mode" != "auto-close" ]]; then
    pr_usage_error "--duplicate-mode must be 'safe' or 'auto-close'."
  fi

  if [[ "$create_pr" == "true" && "$dry_run" != "true" ]]; then
    pr_usage_error "--create-pr is only supported with --dry-run."
  fi

  if [[ "$allow_partial_create" == "true" && "$create_pr" != "true" ]]; then
    pr_usage_error "--allow-partial-create requires --create-pr."
  fi

  if [[ -n "$auto_edit_pr_number" && "$create_pr" == "true" ]]; then
    pr_usage_error "--auto-edit cannot be combined with --create-pr/--auto."
  fi

  if [[ "$dry_run" == "false" ]]; then
    if [[ -n "$auto_edit_pr_number" && ${#positionals[@]} -gt 1 ]]; then
      pr_usage_error "In --auto-edit mode (MAIN_PR_NUMBER), positional OUTPUT_FILE is not allowed."
    fi
    if [[ -z "$auto_edit_pr_number" && ${#positionals[@]} -gt 2 ]]; then
      pr_usage_error "Too many positional arguments. Expected usage: MAIN_PR_NUMBER [OUTPUT_FILE]."
    fi
    if [[ ${#positionals[@]} -ge 1 ]]; then
      main_pr_number="${positionals[0]}"
    fi
    if [[ -z "$auto_edit_pr_number" && ${#positionals[@]} -ge 2 ]]; then
      output_file="${positionals[1]}"
      output_file_explicit="true"
    fi
    if [[ -z "$main_pr_number" ]]; then
      pr_usage_error "MAIN_PR_NUMBER is required."
    fi
  else
    if [[ -n "$auto_edit_pr_number" && "$auto_mode" != "true" && ${#positionals[@]} -gt 0 ]]; then
      pr_usage_error "In --auto-edit dry-run mode, positional OUTPUT_FILE is not allowed."
    fi
    if [[ -z "$auto_edit_pr_number" && "$auto_mode" != "true" && ${#positionals[@]} -gt 1 ]]; then
      pr_usage_error "Too many positional arguments for --dry-run. Only OUTPUT_FILE is allowed."
    fi
    if [[ -z "$auto_edit_pr_number" && "$auto_mode" != "true" && ${#positionals[@]} -ge 1 ]]; then
      output_file="${positionals[0]}"
      output_file_explicit="true"
    fi
  fi

  if [[ "$dry_run" == "true" && "$create_pr" != "true" && "$auto_mode" != "true" && -z "$auto_edit_pr_number" && "$output_file_explicit" != "true" ]]; then
    write_body_to_file="false"
  fi
}
