#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154


# PR body section extraction/replacement/merge helpers.

pr_build_sectional_auto_edit_body() {
  local current_body="$1"
  local generated_body="$2"
  local merged_body heading section target_heading

  merged_body="$current_body"
  for heading in "### Description" "### Validation Gate" "### Issue Outcomes" "### Key Changes"; do
    section="$(pr_extract_top_level_section_from_body "$generated_body" "$heading" || true)"
    [[ -z "$section" ]] && continue
    target_heading="$heading"
    if [[ "$heading" == "### Validation Gate" ]] \
      && ! pr_body_has_top_level_heading "$merged_body" "### Validation Gate" \
      && pr_body_has_top_level_heading "$merged_body" "### Validation Status"; then
      target_heading="### Validation Status"
    fi
    merged_body="$(pr_replace_top_level_section_in_body "$merged_body" "$target_heading" "$section")"
  done

  if pr_body_has_top_level_heading "$merged_body" "### Validation Gate" \
    && pr_body_has_top_level_heading "$merged_body" "### Validation Status"; then
    merged_body="$(pr_remove_top_level_section_from_body "$merged_body" "### Validation Status")"
  fi

  echo "$merged_body"
}
