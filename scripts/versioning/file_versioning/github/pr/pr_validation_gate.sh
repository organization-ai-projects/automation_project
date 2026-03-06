#!/usr/bin/env bash
# shellcheck shell=bash
# shellcheck disable=SC2034,SC2154


# Validation Gate section helpers extracted from generate_pr_description.sh.

pr_build_validation_gate_section() {
  local ci_status_with_symbol="$1"
  local breaking_detected="$2"
  local breaking_scope_crates="$3"
  local breaking_scope_commits="$4"

  {
    echo "### Validation Gate"
    echo ""
    echo "- CI: ${ci_status_with_symbol}"
    if [[ "$breaking_detected" -eq 1 ]]; then
      echo "- Breaking change"
      echo "- Breaking scope:"
      if [[ -n "$breaking_scope_crates" ]]; then
        echo "  - crate(s): ${breaking_scope_crates}"
      else
        echo "  - crate(s): unknown"
      fi
      if [[ -n "$breaking_scope_commits" ]]; then
        echo "  - source commit(s): ${breaking_scope_commits}"
      else
        echo "  - source commit(s): unknown"
      fi
    else
      echo "- No breaking change"
    fi
  }
}

pr_replace_validation_gate_in_body() {
  local original_body="$1"
  local replacement="$2"
  local repl_file
  repl_file="$(mktemp)"
  printf "%s\n" "$replacement" >"$repl_file"

  awk -v repl_file="$repl_file" '
    function print_replacement() {
      while ((getline line < repl_file) > 0) {
        print line
      }
      close(repl_file)
    }
    BEGIN {
      in_validation = 0
      replaced = 0
    }
    {
      if ($0 == "### Validation Gate") {
        print_replacement()
        replaced = 1
        in_validation = 1
        next
      }

      if (in_validation) {
        if ($0 ~ /^### /) {
          in_validation = 0
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
