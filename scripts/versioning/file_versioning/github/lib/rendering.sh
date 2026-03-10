#!/usr/bin/env bash

write_section_from_file() {
  local file="$1"
  if [[ -s "$file" ]]; then
    while IFS= read -r line; do
      pr_num="$(echo "$line" | sed -nE 's/.*\(#([0-9]+)\)$/\1/p')"
      if [[ -z "$pr_num" ]]; then
        # Fallback when bullet keeps inline refs (e.g. merge headlines with #N).
        pr_num="$(echo "$line" | sed -nE 's/.*#([0-9]+).*/\1/p')"
      fi
      if [[ -z "$pr_num" ]]; then
        pr_num=999999
      fi
      printf "%06d|%s\n" "$pr_num" "$line"
    done <"$file" | sort -t'|' -k1,1n -k2,2 | cut -d'|' -f2-
  else
    echo "- No significant items detected."
  fi
}

pr_render_grouped_by_category() {
  local input_file="$1"
  local mode="$2"
  local output_file="$3"
  local va_output=""

  if command -v va_exec >/dev/null 2>&1; then
    va_output="$(va_exec pr group-by-category --input-file "$input_file" --mode "$mode" 2>/dev/null || true)"
    if [[ -n "$va_output" ]]; then
      printf '%s\n' "$va_output" >"$output_file"
      return 0
    fi
  fi

  sort -t'|' -k1,1n "$input_file" |
    awk -F'|' -v mode="$mode" '
      BEGIN {
        cats[1] = "Security"
        cats[2] = "Features"
        cats[3] = "Bug Fixes"
        cats[4] = "Refactoring"
        cats[5] = "Automation"
        cats[6] = "Testing"
        cats[7] = "Docs"
        cats[8] = "Mixed"
        cats[9] = "Unknown"
      }
      {
        lines[NR] = $0
      }
      END {
        for (c = 1; c <= 9; c++) {
          cat = cats[c]
          found = 0
          for (i = 1; i <= NR; i++) {
            split(lines[i], parts, "|")
            if (parts[2] == cat) {
              if (!found) {
                print "#### " cat
                found = 1
              }
              if (mode == "resolved") {
                print "- " parts[3] " " parts[4]
              } else if (mode == "reopen") {
                print "- Reopen " parts[3]
              } else if (mode == "conflict") {
                print "- " parts[3] " - " parts[4]
              } else {
                print "- " parts[3]
              }
            }
          }
          if (found) {
            print ""
          }
        }
      }
    ' >"$output_file"
}

build_dynamic_pr_title() {
  local categories=()
  local summary

  if [[ -s "$sync_tmp" ]]; then
    categories+=("Synchronization")
  fi
  if [[ -s "$features_tmp" ]]; then
    categories+=("Features")
  fi
  if [[ -s "$bugs_tmp" ]]; then
    categories+=("Bug Fixes")
  fi
  if [[ -s "$refactors_tmp" ]]; then
    categories+=("Refactoring")
  fi

  if [[ ${#categories[@]} -eq 0 ]]; then
    summary="Changes"
  elif [[ ${#categories[@]} -eq 1 ]]; then
    summary="${categories[0]}"
  elif [[ ${#categories[@]} -eq 2 ]]; then
    summary="${categories[0]} and ${categories[1]}"
  else
    summary="${categories[0]}"
    for ((i = 1; i < ${#categories[@]} - 1; i++)); do
      summary+=", ${categories[i]}"
    done
    summary+=", and ${categories[${#categories[@]} - 1]}"
  fi

  echo "Merge ${head_ref} into ${base_ref}: ${summary}"
}
