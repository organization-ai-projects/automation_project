#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154

# CLI finalize/validation helpers.

pr_args_finalize_apply_mode() {
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
}

pr_args_finalize_validate_flags() {
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
}

pr_args_finalize_assign_main_mode_positionals() {
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
}

pr_args_finalize_assign_dry_mode_positionals() {
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
}

pr_args_finalize_write_mode() {
  if [[ "$dry_run" == "true" && "$create_pr" != "true" && "$auto_mode" != "true" && -z "$auto_edit_pr_number" && "$output_file_explicit" != "true" ]]; then
    write_body_to_file="false"
  fi
}

pr_args_finalize() {
  pr_args_finalize_apply_mode
  pr_args_finalize_validate_flags

  if [[ "$dry_run" == "false" ]]; then
    pr_args_finalize_assign_main_mode_positionals
  else
    pr_args_finalize_assign_dry_mode_positionals
  fi

  pr_args_finalize_write_mode
}
