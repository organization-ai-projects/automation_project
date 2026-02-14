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

classify_pr() {
  local pr_ref="$1"
  local title="$2"
  local title_lc
  local bullet
  local category
  local starts_sync_or_merge=0

  title_lc="$(echo "$title" | tr '[:upper:]' '[:lower:]')"
  bullet="$(build_pr_bullet "$title" "$pr_ref")"

  if [[ "$title_lc" =~ ^[[:space:]]*(sync|merge) ]] \
    || [[ "$title_lc" =~ ^[[:space:]]*(chore|refactor|fix|feat|docs|test|tests)[^:]*:[[:space:]]*(sync|merge) ]]; then
    starts_sync_or_merge=1
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
