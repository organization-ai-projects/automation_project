#!/usr/bin/env bash

set -u

# Usage:
#   ./scripts/versioning/file_versioning/github/generate_pr_description.sh [--keep-artifacts] [MAIN_PR_NUMBER] [OUTPUT_FILE]
# Example:
#   ./scripts/versioning/file_versioning/github/generate_pr_description.sh 234 pr_description.md
#   ./scripts/versioning/file_versioning/github/generate_pr_description.sh --keep-artifacts 234 pr_description.md

main_pr_number="234"
output_file="pr_description.md"
keep_artifacts="false"

positionals=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --keep-artifacts)
      keep_artifacts="true"
      shift
      ;;
    -h|--help)
      echo "Usage: ./scripts/versioning/file_versioning/github/generate_pr_description.sh [--keep-artifacts] [MAIN_PR_NUMBER] [OUTPUT_FILE]"
      exit 0
      ;;
    *)
      positionals+=("$1")
      shift
      ;;
  esac
done

if [[ ${#positionals[@]} -ge 1 ]]; then
  main_pr_number="${positionals[0]}"
fi
if [[ ${#positionals[@]} -ge 2 ]]; then
  output_file="${positionals[1]}"
fi

if [[ "$keep_artifacts" == "true" ]]; then
  extracted_prs_file="extracted_prs.txt"
  resolved_issues_file="resolved_issues.txt"
else
  extracted_prs_file="$(mktemp)"
  resolved_issues_file="$(mktemp)"
fi

features_tmp="$(mktemp)"
bugs_tmp="$(mktemp)"
refactors_tmp="$(mktemp)"
issues_tmp="$(mktemp)"

cleanup() {
  rm -f "$features_tmp" "$bugs_tmp" "$refactors_tmp" "$issues_tmp"
  if [[ "$keep_artifacts" != "true" ]]; then
    rm -f "$extracted_prs_file" "$resolved_issues_file"
  fi
}
trap cleanup EXIT

if ! command -v gh >/dev/null 2>&1; then
  echo "Erreur: la commande 'gh' est introuvable." >&2
  exit 1
fi

extract_child_prs() {
  local commit_headlines

  commit_headlines="$(gh pr view "$main_pr_number" --json commits -q '.commits[].messageHeadline' 2>/dev/null || true)"
  if [[ -z "$commit_headlines" ]]; then
    return 1
  fi

  {
    echo "$commit_headlines" | sed -nE 's/.*Merge pull request #([0-9]+).*/#\1/p'
    echo "$commit_headlines" | sed -nE 's/.*\(#([0-9]+)\)\s*$/#\1/p'
  } | sort -u | grep -v "^#${main_pr_number}$" > "$extracted_prs_file"

  return 0
}

classify_pr() {
  local pr_ref="$1"
  local title="$2"
  local title_lc
  local bullet

  title_lc="$(echo "$title" | tr '[:upper:]' '[:lower:]')"
  bullet="- ${title} (${pr_ref})"

  # Prefer conventional commit prefixes when present.
  if [[ "$title_lc" =~ ^fix(\(|:|!|[[:space:]]) ]]; then
    echo "$bullet" >> "$bugs_tmp"
    return
  fi
  if [[ "$title_lc" =~ ^refactor(\(|:|!|[[:space:]]) ]] || [[ "$title_lc" =~ ^chore(\(|:|!|[[:space:]]) ]]; then
    echo "$bullet" >> "$refactors_tmp"
    return
  fi
  if [[ "$title_lc" =~ ^feat(\(|:|!|[[:space:]]) ]]; then
    echo "$bullet" >> "$features_tmp"
    return
  fi

  if [[ "$title_lc" =~ (fix|bug|hotfix|regression|failure|error) ]]; then
    echo "$bullet" >> "$bugs_tmp"
  elif [[ "$title_lc" =~ (refactor|cleanup|extract|modular|rework|batch|maintainability) ]]; then
    echo "$bullet" >> "$refactors_tmp"
  else
    echo "$bullet" >> "$features_tmp"
  fi
}

write_section_from_file() {
  local file="$1"
  if [[ -s "$file" ]]; then
    while IFS= read -r line; do
      pr_num="$(echo "$line" | sed -nE 's/.*\(#([0-9]+)\)$/\1/p')"
      if [[ -z "$pr_num" ]]; then
        pr_num=999999
      fi
      printf "%06d|%s\n" "$pr_num" "$line"
    done < "$file" | sort -t'|' -k1,1n -k2,2 | cut -d'|' -f2-
  else
    echo "- No significant items detected."
  fi
}

issue_title() {
  local issue_number="$1"
  gh issue view "$issue_number" --json title -q '.title' 2>/dev/null || true
}

parse_issue_refs_from_body() {
  local body="$1"
  echo "$body" | awk '
    BEGIN { IGNORECASE = 1 }
    {
      line = $0
      lower = tolower($0)
      action = ""
      if (match(lower, /(closes?|closed|fixes?|fixed|resolves?|resolved)/)) {
        token = substr(lower, RSTART, RLENGTH)
        if (token ~ /^clos/) {
          action = "Closes"
        } else if (token ~ /^fix/) {
          action = "Fixes"
        } else {
          action = "Resolves"
        }

        while (match(line, /([[:alnum:]_.-]+\/)?#[0-9]+/)) {
          issue_ref = substr(line, RSTART, RLENGTH)
          sub(/^[[:alnum:]_.-]+\//, "", issue_ref)
          print action "|" issue_ref
          line = substr(line, RSTART + RLENGTH)
        }
      }
    }
  ' \
    | sort -u
}

echo -n > "$extracted_prs_file"
echo -n > "$resolved_issues_file"

if ! extract_child_prs; then
  echo "Avertissement: impossible de récupérer les commits de la PR #${main_pr_number} (API indisponible ou PR introuvable)." >&2
fi

declare -A seen_issue
pr_count=0
issue_count=0

if [[ -s "$extracted_prs_file" ]]; then
  while read -r pr_ref; do
    [[ -z "$pr_ref" ]] && continue
    pr_number="${pr_ref//#/}"

    pr_title="$(gh pr view "$pr_number" --json title -q '.title' 2>/dev/null || true)"
    pr_body="$(gh pr view "$pr_number" --json body -q '.body' 2>/dev/null || true)"

    if [[ -z "$pr_title" ]]; then
      pr_title="PR #${pr_number}"
    fi

    classify_pr "$pr_ref" "$pr_title"
    pr_count=$((pr_count + 1))

    while read -r issue_key; do
      [[ -z "$issue_key" ]] && continue

      if [[ -n "${seen_issue[$issue_key]:-}" ]]; then
        continue
      fi
      seen_issue["$issue_key"]=1

      issue_number="${issue_key//#/}"
      issue_name="$(issue_title "$issue_number")"
      if [[ -z "$issue_name" ]]; then
        issue_name="Issue #${issue_number}"
      fi

      echo "${issue_number}|Closes|${issue_key}|${issue_name}" >> "$issues_tmp"
    done < <(
      {
        if [[ -n "$pr_body" ]]; then
          while IFS='|' read -r action issue_key; do
            [[ -z "$issue_key" ]] && continue

            if [[ -n "${seen_issue[$issue_key]:-}" ]]; then
              continue
            fi
            seen_issue["$issue_key"]=1

            issue_number="${issue_key//#/}"
            issue_name="$(issue_title "$issue_number")"
            if [[ -z "$issue_name" ]]; then
              issue_name="Issue #${issue_number}"
            fi
            echo "${issue_number}|${action}|${issue_key}|${issue_name}" >> "$issues_tmp"
          done < <(parse_issue_refs_from_body "$pr_body")
        fi

        gh pr view "$pr_number" --json closingIssuesReferences -q \
          '.closingIssuesReferences[]? | "#\(.number)"' 2>/dev/null || true
      } | sort -u
    )
  done < "$extracted_prs_file"
fi

if [[ -s "$issues_tmp" ]]; then
  sort -t'|' -k1,1n "$issues_tmp" \
    | awk -F'|' '{ print "- " $2 " " $3 ": " $4 }' > "$resolved_issues_file"
  issue_count="$(wc -l < "$resolved_issues_file" | tr -d '[:space:]')"
fi

{
  echo "### Description"
  echo "This pull request merges the \`dev\` branch into \`main\`, with ${pr_count} merged pull requests and ${issue_count} explicitly resolved issues."
  echo ""
  echo "### Issues Resolved"
  echo "This PR resolves the following issues:"
  if [[ -s "$resolved_issues_file" ]]; then
    cat "$resolved_issues_file"
  else
    echo "- No resolved issues detected via GitHub references or PR body keywords."
  fi
  echo ""
  echo "### Key Changes"
  echo "#### Features"
  write_section_from_file "$features_tmp"
  echo ""
  echo "#### Bug Fixes"
  write_section_from_file "$bugs_tmp"
  echo ""
  echo "#### Refactoring"
  write_section_from_file "$refactors_tmp"
  echo ""
  cat <<EOF
### Testing
- Ensure all project tests are executed before merge (for example: \`cargo test\`, script-specific checks, and CI workflow validation).
- Validate manually the automation workflows impacted by merged PRs.

### Additional Notes
- Documentation and PR summaries should be aligned with the resolved issues listed above.
- This generated description can be edited to add domain-specific details before submission.
EOF
} > "$output_file"

echo "Fichier généré: $output_file"
if [[ "$keep_artifacts" == "true" ]]; then
  echo "PR extraites: $extracted_prs_file"
  echo "Issues résolues: $resolved_issues_file"
fi
