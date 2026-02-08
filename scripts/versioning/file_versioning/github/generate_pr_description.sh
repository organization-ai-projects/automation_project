#!/usr/bin/env bash

set -u

# Usage:
#   ./scripts/versioning/file_versioning/github/generate_pr_description.sh [--keep-artifacts] MAIN_PR_NUMBER [OUTPUT_FILE]
# Example:
#   ./scripts/versioning/file_versioning/github/generate_pr_description.sh 234 pr_description.md
#   ./scripts/versioning/file_versioning/github/generate_pr_description.sh --keep-artifacts 234 pr_description.md

main_pr_number=""
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
      echo "Usage: ./scripts/versioning/file_versioning/github/generate_pr_description.sh [--keep-artifacts] MAIN_PR_NUMBER [OUTPUT_FILE]"
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

if [[ -z "$main_pr_number" ]]; then
  echo "Erreur: MAIN_PR_NUMBER est requis." >&2
  echo "Usage: ./scripts/versioning/file_versioning/github/generate_pr_description.sh [--keep-artifacts] MAIN_PR_NUMBER [OUTPUT_FILE]" >&2
  exit 1
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
  local main_pr_body
  local main_pr_comments
  local repo_owner_name
  local timeline_pr_refs

  repo_owner_name="$(gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true)"

  # IMPORTANT: gh pr view --json commits is often capped (e.g. 100 items).
  # Use REST API with --paginate to collect all commit first lines.
  commit_headlines=""
  if [[ -n "$repo_owner_name" ]]; then
    commit_headlines="$(gh api "repos/${repo_owner_name}/pulls/${main_pr_number}/commits" --paginate \
      --jq '.[].commit.message | split("\n")[0]' 2>/dev/null || true)"
  fi

  main_pr_body="$(gh pr view "$main_pr_number" --json body -q '.body' 2>/dev/null || true)"
  main_pr_comments="$(gh pr view "$main_pr_number" --json comments -q '.comments[].body' 2>/dev/null || true)"
  timeline_pr_refs=""
  if [[ -n "$repo_owner_name" ]]; then
    timeline_pr_refs="$(gh api "repos/${repo_owner_name}/issues/${main_pr_number}/timeline" --paginate \
      --jq '.[] | select(.event=="cross-referenced") | select(.source.issue.pull_request.url != null) | .source.issue.number' 2>/dev/null \
      | sed -E 's/^/#/' || true)"
  fi

  if [[ -z "$commit_headlines" && -z "$main_pr_body" && -z "$main_pr_comments" && -z "$timeline_pr_refs" ]]; then
    return 1
  fi

  {
    echo "$commit_headlines" | sed -nE 's/.*Merge pull request #([0-9]+).*/#\1/p'
    echo "$commit_headlines" | sed -nE 's/.*\(#([0-9]+)\)\s*$/#\1/p'
    # Capture PR references from main PR body/comments (e.g. /pull/306 or "PR #306")
    echo "$main_pr_body" | grep -oE '/pull/[0-9]+' | sed -E 's#^/pull/([0-9]+)$#\#\1#'
    echo "$main_pr_body" | sed -nE 's/.*\bPR[[:space:]]*#([0-9]+).*/#\1/ip'
    echo "$main_pr_body" | sed -nE 's/.*pull request #([0-9]+).*/#\1/ip'
    echo "$main_pr_comments" | grep -oE '/pull/[0-9]+' | sed -E 's#^/pull/([0-9]+)$#\#\1#'
    echo "$main_pr_comments" | sed -nE 's/.*\bPR[[:space:]]*#([0-9]+).*/#\1/ip'
    echo "$main_pr_comments" | sed -nE 's/.*pull request #([0-9]+).*/#\1/ip'
    echo "$timeline_pr_refs"
  } | sort -u | grep -v "^#${main_pr_number}$" > "$extracted_prs_file"

  return 0
}

classify_pr() {
  local pr_ref="$1"
  local title="$2"
  local title_lc
  local bullet
  local category

  title_lc="$(echo "$title" | tr '[:upper:]' '[:lower:]')"
  bullet="- ${title} (${pr_ref})"

  # Prefer conventional commit prefixes when present.
  if [[ "$title_lc" =~ ^fix(\(|:|!|[[:space:]]) ]]; then
    category="Bug Fixes"
    echo "$bullet" >> "$bugs_tmp"
    echo "$category"
    return
  fi
  if [[ "$title_lc" =~ ^refactor(\(|:|!|[[:space:]]) ]] || [[ "$title_lc" =~ ^chore(\(|:|!|[[:space:]]) ]]; then
    category="Refactoring"
    echo "$bullet" >> "$refactors_tmp"
    echo "$category"
    return
  fi
  if [[ "$title_lc" =~ ^feat(\(|:|!|[[:space:]]) ]]; then
    category="Features"
    echo "$bullet" >> "$features_tmp"
    echo "$category"
    return
  fi

  if [[ "$title_lc" =~ (fix|bug|hotfix|regression|failure|error) ]]; then
    category="Bug Fixes"
    echo "$bullet" >> "$bugs_tmp"
  elif [[ "$title_lc" =~ (refactor|cleanup|extract|modular|rework|batch|maintainability) ]]; then
    category="Refactoring"
    echo "$bullet" >> "$refactors_tmp"
  else
    category="Features"
    echo "$bullet" >> "$features_tmp"
  fi

  echo "$category"
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

issue_title_and_labels() {
  local issue_number="$1"
  gh issue view "$issue_number" --json title,labels \
    -q '[.title, (.labels | map(.name) | join("||"))] | @tsv' 2>/dev/null || true
}

issue_category_from_labels() {
  local labels_raw="$1"
  local labels
  local has_security=0
  local has_bug=0
  local has_refactor=0
  local has_feature=0
  local has_testing=0
  local has_automation=0
  local has_docs=0

  labels="$(echo "$labels_raw" | tr '[:upper:]' '[:lower:]')"

  # Security is a first-class category and must not be downgraded to bug fixes.
  if [[ "$labels" =~ (security|sec|codeql|cve|vuln|vulnerability|sast) ]]; then
    has_security=1
  fi
  if [[ "$labels" =~ (bug|defect|regression|incident) ]]; then
    has_bug=1
  fi
  if [[ "$labels" =~ (refactor|cleanup|chore|mainten|tech[[:space:]_-]*debt) ]]; then
    has_refactor=1
  fi
  if [[ "$labels" =~ (feature|enhancement|feat) ]]; then
    has_feature=1
  fi
  if [[ "$labels" =~ (testing|tests|test) ]]; then
    has_testing=1
  fi
  if [[ "$labels" =~ (automation|automation-failed|sync_branch|scripts|linting|workflow|ci) ]]; then
    has_automation=1
  fi
  if [[ "$labels" =~ (documentation|docs|readme|translation) ]]; then
    has_docs=1
  fi

  if [[ "$has_security" -eq 1 ]]; then
    echo "Security"
    return
  fi
  if [[ "$has_automation" -eq 1 ]]; then
    echo "Automation"
    return
  fi
  if [[ "$has_testing" -eq 1 ]]; then
    echo "Testing"
    return
  fi
  if [[ "$has_docs" -eq 1 ]]; then
    echo "Docs"
    return
  fi

  count=$((has_bug + has_refactor + has_feature))
  if [[ "$count" -ge 2 ]]; then
    echo "Mixed"
    return
  fi
  if [[ "$has_bug" -eq 1 ]]; then
    echo "Bug Fixes"
    return
  fi
  if [[ "$has_refactor" -eq 1 ]]; then
    echo "Refactoring"
    return
  fi
  if [[ "$has_feature" -eq 1 ]]; then
    echo "Features"
    return
  fi

  echo "Unknown"
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
declare -A issue_category
declare -A issue_action
declare -A issue_name_map
pr_count=0
issue_count=0
base_ref="$(gh pr view "$main_pr_number" --json baseRefName -q '.baseRefName' 2>/dev/null || echo "main")"
head_ref="$(gh pr view "$main_pr_number" --json headRefName -q '.headRefName' 2>/dev/null || echo "dev")"

normalize_issue_action() {
  local action="$1"
  local category="$2"
  local lower

  lower="$(echo "$action" | tr '[:upper:]' '[:lower:]')"

  # Keep closure semantics explicit.
  if [[ "$category" == "Security" ]]; then
    echo "Closes"
    return
  fi

  if [[ "$category" == "Bug Fixes" ]]; then
    echo "Fixes"
    return
  fi

  if [[ "$category" == "Mixed" ]]; then
    echo "Closes"
    return
  fi

  if [[ "$category" == "Automation" || "$category" == "Testing" || "$category" == "Docs" ]]; then
    echo "Closes"
    return
  fi

  if [[ "$category" == "Unknown" ]]; then
    # Keep original keyword when classification is unknown.
    if [[ "$lower" =~ ^fix ]]; then
      echo "Fixes"
    elif [[ "$lower" =~ ^resolve ]]; then
      echo "Resolves"
    else
      echo "Closes"
    fi
    return
  fi

  # Default verb for Features/Refactoring is "Closes".
  echo "Closes"
}

add_issue_entry() {
  local action="$1"
  local issue_key="$2"
  local category="${3:-Unknown}"
  local issue_number
  local issue_name labels_tsv labels_raw label_category
  local final_category
  local normalized_action

  [[ -z "$issue_key" ]] && return
  issue_number="${issue_key//#/}"

  if [[ -n "${seen_issue[$issue_key]:-}" ]]; then
    return
  fi
  seen_issue["$issue_key"]=1

  labels_tsv="$(issue_title_and_labels "$issue_number")"
  issue_name="$(echo "$labels_tsv" | awk -F'\t' '{print $1}')"
  labels_raw="$(echo "$labels_tsv" | awk -F'\t' '{print $2}')"
  label_category="$(issue_category_from_labels "$labels_raw")"

  if [[ -z "$issue_name" ]]; then
    issue_name="$(issue_title "$issue_number")"
    if [[ -z "$issue_name" ]]; then
      issue_name="Issue #${issue_number}"
    fi
  fi

  final_category="$label_category"

  issue_category["$issue_key"]="$final_category"
  issue_name_map["$issue_key"]="$issue_name"
  normalized_action="$(normalize_issue_action "$action" "$final_category")"
  issue_action["$issue_key"]="$normalized_action"
}

if [[ -s "$extracted_prs_file" ]]; then
  while read -r pr_ref; do
    [[ -z "$pr_ref" ]] && continue
    pr_number="${pr_ref//#/}"

    pr_title="$(gh pr view "$pr_number" --json title -q '.title' 2>/dev/null || true)"
    pr_body="$(gh pr view "$pr_number" --json body -q '.body' 2>/dev/null || true)"

    if [[ -z "$pr_title" ]]; then
      pr_title="PR #${pr_number}"
    fi

    pr_category="$(classify_pr "$pr_ref" "$pr_title")"
    pr_count=$((pr_count + 1))

    if [[ -n "$pr_body" ]]; then
      while IFS='|' read -r action issue_key; do
        add_issue_entry "$action" "$issue_key" "$pr_category"
      done < <(parse_issue_refs_from_body "$pr_body")
    fi

    while read -r issue_key; do
      add_issue_entry "Closes" "$issue_key" "$pr_category"
    done < <(
      gh pr view "$pr_number" --json closingIssuesReferences -q \
        '.closingIssuesReferences[]? | "#\(.number)"' 2>/dev/null || true
    )
  done < "$extracted_prs_file"
fi

# Also include issues closed directly by the main PR itself.
main_pr_body="$(gh pr view "$main_pr_number" --json body -q '.body' 2>/dev/null || true)"
if [[ -n "$main_pr_body" ]]; then
  while IFS='|' read -r action issue_key; do
    add_issue_entry "$action" "$issue_key" "Mixed"
  done < <(parse_issue_refs_from_body "$main_pr_body")
fi

while read -r issue_key; do
  add_issue_entry "Closes" "$issue_key" "Mixed"
done < <(
  gh pr view "$main_pr_number" --json closingIssuesReferences -q \
    '.closingIssuesReferences[]? | "#\(.number)"' 2>/dev/null || true
)

echo -n > "$issues_tmp"
for issue_key in "${!seen_issue[@]}"; do
  issue_number="${issue_key//#/}"
  echo "${issue_number}|${issue_category[$issue_key]}|${issue_action[$issue_key]}|${issue_key}|${issue_name_map[$issue_key]}" >> "$issues_tmp"
done

if [[ -s "$issues_tmp" ]]; then
  sort -t'|' -k1,1n "$issues_tmp" \
    | awk -F'|' '
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
              print "- " parts[3] " " parts[4] ": " parts[5]
            }
          }
          if (found) {
            print ""
          }
        }
      }
    ' > "$resolved_issues_file"
  issue_count="${#seen_issue[@]}"
fi

{
  echo "### Description"
  echo "This pull request merges the \`${head_ref}\` branch into \`${base_ref}\`, with ${pr_count} merged pull requests and ${issue_count} explicitly resolved issues."
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
