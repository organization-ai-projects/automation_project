#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154


# PR body section extraction/replacement/merge helpers.

pr_extract_top_level_section_from_body() {
  local body="$1"
  local heading="$2"

  awk -v heading="$heading" '
    BEGIN {
      in_section = 0
      found = 0
    }
    {
      if (!in_section && $0 == heading) {
        in_section = 1
        found = 1
      }

      if (in_section) {
        if ($0 ~ /^### / && $0 != heading) {
          exit
        }
        print $0
      }
    }
    END {
      if (!found) {
        exit 1
      }
    }
  ' <<<"$body"
}

pr_body_has_top_level_heading() {
  local body="$1"
  local heading="$2"
  grep -Fxq "$heading" <<<"$body"
}

pr_remove_top_level_section_from_body() {
  local original_body="$1"
  local heading="$2"

  awk -v heading="$heading" '
    BEGIN {
      in_section = 0
    }
    {
      if ($0 == heading) {
        in_section = 1
        next
      }

      if (in_section) {
        if ($0 ~ /^### /) {
          in_section = 0
          print $0
        }
        next
      }

      print $0
    }
  ' <<<"$original_body"
}

pr_replace_top_level_section_in_body() {
  local original_body="$1"
  local heading="$2"
  local replacement="$3"
  local repl_file
  repl_file="$(mktemp)"
  printf "%s\n" "$replacement" >"$repl_file"

  awk -v heading="$heading" -v repl_file="$repl_file" '
    function print_replacement() {
      while ((getline line < repl_file) > 0) {
        print line
      }
      close(repl_file)
    }
    BEGIN {
      in_section = 0
      replaced = 0
    }
    {
      if ($0 == heading) {
        print_replacement()
        replaced = 1
        in_section = 1
        next
      }

      if (in_section) {
        if ($0 ~ /^### /) {
          in_section = 0
          print ""
          print $0
        }
        next
      }

      print $0
    }
    END {
      if (!replaced) {
        if (NR > 0) {
          print ""
        }
        print_replacement()
      }
    }
  ' <<<"$original_body"

  rm -f "$repl_file"
}

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
