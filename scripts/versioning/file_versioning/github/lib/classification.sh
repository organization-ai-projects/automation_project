#!/usr/bin/env bash

build_pr_bullet() {
  local title="$1"
  local pr_ref="$2"
  local pr_num
  local normalized_title

  pr_num="${pr_ref//#/}"
  normalized_title="$title"

  # Remove redundant trailing "(#N)" when we already have the canonical PR ref.
  normalized_title="$(echo "$normalized_title" | sed -E "s/[[:space:]]*\\(#${pr_num}\\)//g")"
  # Normalize merge commit headline to avoid rendering "#N" twice.
  normalized_title="$(echo "$normalized_title" | sed -E "s/(merge[[:space:]]+pull[[:space:]]+request)[[:space:]]+#${pr_num}([[:space:]]+from)/\\1\\2/I")"
  normalized_title="$(echo "$normalized_title" | sed -E 's/[[:space:]]+/ /g; s/[[:space:]]+$//')"

  if echo "$normalized_title" | grep -Eq "(^|[^0-9])#${pr_num}([^0-9]|$)"; then
    echo "- ${normalized_title}"
  else
    echo "- ${normalized_title} (${pr_ref})"
  fi
}

extract_merge_source_ref() {
  local title_lc="$1"
  echo "$title_lc" | sed -nE 's/.*merge[[:space:]]+pull[[:space:]]+request[[:space:]]*#[0-9]+[[:space:]]+from[[:space:]]+([^[:space:]]+).*/\1/p'
}

classify_merge_by_source_ref() {
  local merge_source_ref="$1"
  local merge_branch

  merge_branch="${merge_source_ref##*/}"

  if [[ "$merge_source_ref" =~ /sync/ ]] \
    || [[ "$merge_branch" =~ (main|dev|master|staging|release[^[:space:]]*)-?(into|to)-?(main|dev|master|staging|release[^[:space:]]*) ]]; then
    echo "Synchronization"
    return
  fi

  if [[ "$merge_source_ref" =~ /fix/ ]]; then
    echo "Bug Fixes"
    return
  fi

  if [[ "$merge_source_ref" =~ /(refactor|chore|docs|doc|test|tests)/ ]]; then
    echo "Refactoring"
    return
  fi

  if [[ "$merge_source_ref" =~ /(feat|feature|enhancement)/ ]]; then
    echo "Features"
    return
  fi

  # Safer default for merge commits: avoid over-classifying as Features.
  echo "Refactoring"
}

classify_pr() {
  local pr_ref="$1"
  local title="$2"
  local title_lc
  local bullet
  local category
  local starts_sync_or_merge=0
  local merge_source_ref=""
  local merge_category=""

  title_lc="$(echo "$title" | tr '[:upper:]' '[:lower:]')"
  bullet="$(build_pr_bullet "$title" "$pr_ref")"

  if [[ "$title_lc" =~ ^[[:space:]]*(sync|merge) ]] \
    || [[ "$title_lc" =~ ^[[:space:]]*(chore|refactor|fix|feat|docs|test|tests)[^:]*:[[:space:]]*(sync|merge) ]]; then
    starts_sync_or_merge=1
  fi

  if [[ "$title_lc" =~ ^[[:space:]]*merge[[:space:]]+pull[[:space:]]+request[[:space:]]*#[0-9]+[[:space:]]+from[[:space:]]+ ]]; then
    merge_source_ref="$(extract_merge_source_ref "$title_lc")"
    merge_category="$(classify_merge_by_source_ref "$merge_source_ref")"

    case "$merge_category" in
      Synchronization)
        echo "$bullet" >> "$sync_tmp"
        ;;
      "Bug Fixes")
        echo "$bullet" >> "$bugs_tmp"
        ;;
      Refactoring)
        echo "$bullet" >> "$refactors_tmp"
        ;;
      *)
        echo "$bullet" >> "$features_tmp"
        ;;
    esac
    debug_log "classify_pr: ${pr_ref} -> ${merge_category} (merge source: ${merge_source_ref})"
    echo "$merge_category"
    return
  fi

  # Keep synchronization PRs in a dedicated category.
  # Allow an optional conventional prefix (e.g. "chore:" / "chore(scope):")
  # and require explicit branch-flow markers to avoid false positives.
  if [[ "$starts_sync_or_merge" -eq 1 ]] \
    && [[ "$title_lc" =~ (main|dev|master|staging|release[^[:space:]]*)[^[:alnum:]_/-]+(into|->|→)[^[:alnum:]_/-]+(main|dev|master|staging|release[^[:space:]]*) ]]; then
    category="Synchronization"
    echo "$bullet" >> "$sync_tmp"
    debug_log "classify_pr: ${pr_ref} -> ${category}"
    echo "$category"
    return
  fi

  # Prefer conventional commit prefixes when present.
  if [[ "$title_lc" =~ ^fix(\(|:|!|[[:space:]]) ]]; then
    category="Bug Fixes"
    echo "$bullet" >> "$bugs_tmp"
    debug_log "classify_pr: ${pr_ref} -> ${category}"
    echo "$category"
    return
  fi
  if [[ "$title_lc" =~ ^refactor(\(|:|!|[[:space:]]) ]] || [[ "$title_lc" =~ ^chore(\(|:|!|[[:space:]]) ]]; then
    category="Refactoring"
    echo "$bullet" >> "$refactors_tmp"
    debug_log "classify_pr: ${pr_ref} -> ${category}"
    echo "$category"
    return
  fi
  if [[ "$title_lc" =~ ^feat(\(|:|!|[[:space:]]) ]]; then
    category="Features"
    echo "$bullet" >> "$features_tmp"
    debug_log "classify_pr: ${pr_ref} -> ${category}"
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

  debug_log "classify_pr: ${pr_ref} -> ${category}"
  echo "$category"
}

issue_category_from_labels() {
  local labels_raw="$1"
  local labels
  local label
  local has_security=0
  local has_bug=0
  local has_refactor=0
  local has_feature=0
  local has_testing=0
  local has_automation=0
  local has_docs=0

  labels="$(echo "$labels_raw" | tr '[:upper:]' '[:lower:]')"

  # Analyze each label token independently to avoid cross-label false positives.
  # labels_raw format is "label1||label2||..."
  IFS='||' read -r -a labels_arr <<< "$labels"
  for label in "${labels_arr[@]}"; do
    [[ -z "$label" ]] && continue

    # Security is a first-class category and must not be downgraded.
    case "$label" in
      security|sec|codeql|cve|vuln|vulnerability|sast)
        has_security=1
        ;;
      bug|defect|regression|incident)
        has_bug=1
        ;;
      refactor|cleanup|chore|maintainability|maintenance|tech-debt|tech_debt|technical-debt|technical_debt)
        has_refactor=1
        ;;
      feature|enhancement|feat)
        has_feature=1
        ;;
      testing|tests|test)
        has_testing=1
        ;;
      automation|automation-failed|sync_branch|scripts|linting|workflow|ci)
        has_automation=1
        ;;
      documentation|docs|readme|translation)
        has_docs=1
        ;;
      *)
        ;;
    esac
  done

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

normalize_issue_action() {
  local _action="$1"
  local _category="$2"
  # Footer/PR policy standardization: use a single closing verb.
  echo "Closes"
}
