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
    done < "$file" | sort -t'|' -k1,1n -k2,2 | cut -d'|' -f2-
  else
    echo "- No significant items detected."
  fi
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
    summary+=", and ${categories[${#categories[@]}-1]}"
  fi

  echo "Merge ${head_ref} into ${base_ref}: ${summary}"
}
