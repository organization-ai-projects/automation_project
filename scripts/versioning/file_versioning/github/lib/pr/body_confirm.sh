#!/usr/bin/env bash

# PR body confirmation/prompt helpers.

pr_body_confirm_with_policy() {
  local prompt="$1"
  local noninteractive_policy="$2" # auto-yes | require-yes
  local noninteractive_error="${3:-}"
  local answer

  if [[ "$assume_yes" == "true" ]]; then
    return 0
  fi

  if ! pr_is_human_interactive_terminal; then
    if [[ "$noninteractive_policy" == "auto-yes" ]]; then
      return 0
    fi
    pr_usage_error "$noninteractive_error"
  fi

  read -r -p "$prompt" answer
  case "$answer" in
  y | Y | yes | YES) return 0 ;;
  *) return 1 ;;
  esac
}

pr_body_should_show_create_summary() {
  if [[ "$assume_yes" == "true" ]]; then
    return 1
  fi

  if [[ "$auto_mode" == "true" ]] && ! pr_is_human_interactive_terminal; then
    return 1
  fi

  return 0
}
