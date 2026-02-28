#!/usr/bin/env bash

# Shared policy helpers for pre-push orchestration.

ROOT_DIR="$(git rev-parse --show-toplevel)"
HOOKS_DIR="$ROOT_DIR/scripts/automation/git_hooks"

# shellcheck source=scripts/automation/git_hooks/lib/issue_parent_guard.sh
source "$HOOKS_DIR/lib/issue_parent_guard.sh"
# shellcheck source=scripts/common_lib/automation/scope_resolver.sh
source "$ROOT_DIR/scripts/common_lib/automation/scope_resolver.sh"
# shellcheck source=scripts/automation/git_hooks/lib/policy.sh
source "$HOOKS_DIR/lib/policy.sh"
# shellcheck source=scripts/common_lib/automation/file_types.sh
source "$ROOT_DIR/scripts/common_lib/automation/file_types.sh"
# shellcheck source=scripts/automation/git_hooks/lib/markdownlint_policy.sh
source "$HOOKS_DIR/lib/markdownlint_policy.sh"

push_policy_remote_checks_warn_only() {
  if [[ "${HOOKS_REMOTE_POLICY:-}" == "warn" || "${ALLOW_OFFLINE_REMOTE_CHECKS:-}" == "1" ]]; then
    return 0
  fi
  return 1
}

push_policy_resolve_upstream_branch() {
  local upstream
  upstream="$(git rev-parse --abbrev-ref --symbolic-full-name @{u} 2>/dev/null || echo "")"
  if [[ -z "$upstream" ]]; then
    echo "⚠️  No upstream branch detected. Falling back to origin/dev for scope detection." >&2
    upstream="origin/dev"
  fi
  printf '%s\n' "$upstream"
}

push_policy_collect_push_commits() {
  local upstream="$1"
  git log "$upstream"..HEAD --format=%B 2>/dev/null || true
}

push_policy_refresh_upstream_branch() {
  local upstream="$1"
  local current_branch
  local upstream_branch

  current_branch="$(git rev-parse --abbrev-ref HEAD 2>/dev/null || true)"

  if [[ -n "$upstream" && "$upstream" == */* ]]; then
    upstream_branch="${upstream#*/}"
    git fetch origin "$upstream_branch" >/dev/null 2>&1 || true
    return 0
  fi

  if [[ -n "$current_branch" ]]; then
    git fetch origin "$current_branch" >/dev/null 2>&1 || true
  fi
}

push_policy_extract_issue_refs_with_duplicates() {
  local text="$1"
  echo "$text" | awk '
    {
      line = $0
      lower = tolower($0)
      while (match(lower, /(closes|fixes|part[[:space:]]+of|reopen|reopens)[[:space:]]+#[0-9]+/)) {
        matched = substr(line, RSTART, RLENGTH)
        keyword = tolower(matched)
        gsub(/[[:space:]]+#[0-9]+$/, "", keyword)
        issue = matched
        sub(/^.*#/, "", issue)
        print keyword "|" issue
        line = substr(line, RSTART + RLENGTH)
        lower = substr(lower, RSTART + RLENGTH)
      }
    }
  '
}

push_policy_compute_changed_files() {
  local upstream="$1"
  local files=""

  files=$(git diff --name-only "${upstream}"..HEAD 2>/dev/null || true)
  if [[ -n "$files" ]]; then
    printf '%s\n' "$files"
    return 0
  fi

  if git rev-parse --verify --quiet origin/dev >/dev/null; then
    local base
    base=$(git merge-base origin/dev HEAD 2>/dev/null || true)
    if [[ -n "$base" ]]; then
      files=$(git diff --name-only "${base}"..HEAD 2>/dev/null || true)
      if [[ -n "$files" ]]; then
        printf '%s\n' "$files"
        return 0
      fi
    fi
  fi

  files=$(git diff-tree --no-commit-id --name-only -r HEAD 2>/dev/null || true)
  printf '%s\n' "$files"
}

push_policy_collect_markdown_files() {
  local files="$1"
  local file

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    if is_markdown_path_file "$file" && [[ -f "$file" ]]; then
      printf '%s\n' "$file"
    fi
  done <<< "$files"
}

push_policy_run_shell_syntax_checks() {
  local files="$1"
  local checked=0
  local file

  while IFS= read -r file; do
    if is_shell_file "$file"; then
      echo "   - bash -n $file"
      bash -n "$file"
      checked=1
    fi
  done <<< "$files"

  if [[ $checked -eq 0 ]]; then
    echo "   (no shell scripts changed)"
  fi
}

push_policy_run_markdownlint_checks() {
  markdownlint_policy_run_checks "$1"
}

push_policy_validate_no_root_parent_issue_refs() {
  local commits_input="$1"
  local refs
  refs="$(extract_issue_refs_from_text "$commits_input" || true)"
  [[ -z "$refs" ]] && return 0

  if ! command -v gh >/dev/null 2>&1; then
    if push_policy_remote_checks_warn_only; then
      echo "⚠️  Remote footer check skipped (gh unavailable; warn-only mode enabled)."
      return 0
    fi
    echo "❌ Cannot validate root parent issue references: 'gh' CLI is required."
    echo "   Install gh, or bypass in emergency: SKIP_PRE_PUSH=1 git push"
    return 1
  fi

  local repo_name
  repo_name="$(resolve_repo_name_with_owner)"
  if [[ -z "$repo_name" ]]; then
    if push_policy_remote_checks_warn_only; then
      echo "⚠️  Remote footer check skipped (repo unresolved; warn-only mode enabled)."
      return 0
    fi
    echo "❌ Cannot resolve GitHub repository for footer validation."
    echo "   Ensure gh auth/network is available, or bypass in emergency: SKIP_PRE_PUSH=1 git push"
    return 1
  fi

  local parent_refs=()
  local action
  local issue_number
  while IFS='|' read -r action issue_number; do
    [[ -z "$issue_number" ]] && continue
    if issue_is_root_parent "$issue_number" "$repo_name"; then
      parent_refs+=("${action} #${issue_number}")
    fi
  done <<< "$refs"

  if [[ ${#parent_refs[@]} -gt 0 ]]; then
    echo "❌ Root parent issue references detected in commits being pushed:"
    printf '   - %s\n' "${parent_refs[@]}"
    echo "   Root parent refs are forbidden in commit trailers (Part of/Closes/Fixes/Reopen)."
    echo "   Reference child issues instead."
    return 1
  fi
}

push_policy_validate_part_of_only_push() {
  local commits_input="$1"
  local refs_with_duplicates
  local action
  local issue_number
  local repo_name
  local current_login
  local -a violations=()

  refs_with_duplicates="$(push_policy_extract_issue_refs_with_duplicates "$commits_input" || true)"
  [[ -z "$refs_with_duplicates" ]] && return 0

  if ! command -v gh >/dev/null 2>&1; then
    if push_policy_remote_checks_warn_only; then
      echo "⚠️  Assignment policy check skipped (gh unavailable; warn-only mode enabled)."
      return 0
    fi
    if [[ "${ALLOW_PART_OF_ONLY_PUSH:-}" == "1" ]]; then
      return 0
    fi
    echo "❌ Cannot validate Part-of assignment policy: 'gh' CLI is required."
    echo "   Install gh, or bypass in emergency:"
    echo "   ALLOW_PART_OF_ONLY_PUSH=1 git push"
    return 1
  fi

  repo_name="$(resolve_repo_name_with_owner)"
  if [[ -z "$repo_name" ]]; then
    if push_policy_remote_checks_warn_only; then
      echo "⚠️  Assignment policy check skipped (repo unresolved; warn-only mode enabled)."
      return 0
    fi
    if [[ "${ALLOW_PART_OF_ONLY_PUSH:-}" == "1" ]]; then
      return 0
    fi
    echo "❌ Cannot resolve GitHub repository for Part-of assignment policy."
    echo "   Ensure gh auth/network is available, or bypass in emergency:"
    echo "   ALLOW_PART_OF_ONLY_PUSH=1 git push"
    return 1
  fi

  current_login="$(gh api user --jq '.login' 2>/dev/null || true)"
  if [[ -z "$current_login" ]]; then
    if push_policy_remote_checks_warn_only; then
      echo "⚠️  Assignment policy check skipped (login unresolved; warn-only mode enabled)."
      return 0
    fi
    if [[ "${ALLOW_PART_OF_ONLY_PUSH:-}" == "1" ]]; then
      return 0
    fi
    echo "❌ Cannot resolve current GitHub login for Part-of assignment policy."
    echo "   Authenticate gh, or bypass in emergency:"
    echo "   ALLOW_PART_OF_ONLY_PUSH=1 git push"
    return 1
  fi

  declare -A has_part_of=()
  declare -A has_closing=()
  declare -A assignee_count=()
  declare -A sole_assignee=()

  while IFS='|' read -r action issue_number; do
    [[ -z "$action" || -z "$issue_number" ]] && continue
    if [[ "$action" == "part of" ]]; then
      has_part_of["$issue_number"]=1
    fi
    if [[ "$action" == "closes" || "$action" == "fixes" ]]; then
      has_closing["$issue_number"]=1
    fi
  done <<< "$refs_with_duplicates"

  for issue_number in "${!has_part_of[@]}"; do
    [[ -n "${has_closing[$issue_number]:-}" ]] && continue

    local assignees
    local count
    assignees="$(gh issue view "$issue_number" -R "$repo_name" --json assignees --jq '.assignees[].login' 2>/dev/null || true)"
    count="$(printf '%s\n' "$assignees" | sed '/^$/d' | wc -l | tr -d '[:space:]')"
    assignee_count["$issue_number"]="${count:-0}"
    sole_assignee["$issue_number"]="$(printf '%s\n' "$assignees" | sed '/^$/d' | head -n1)"

    if [[ "${assignee_count[$issue_number]}" == "1" && "${sole_assignee[$issue_number]}" == "$current_login" ]]; then
      violations+=("#${issue_number} is assigned only to @${current_login}: 'Closes #${issue_number}' is required (Part of only is not allowed)")
    fi
  done

  if [[ ${#violations[@]} -gt 0 && "${ALLOW_PART_OF_ONLY_PUSH:-}" != "1" ]]; then
    echo "❌ Push blocked by assignment policy."
    printf '   - %s\n' "${violations[@]}"
    echo "   If this push is exceptional, bypass once with:"
    echo "   ALLOW_PART_OF_ONLY_PUSH=1 git push"
    return 1
  fi
}

push_policy_detect_crates_from_scopes() {
  local commits_input="$1"
  local -a crates=()
  local invalid_scopes=0
  local scope_re='^[a-z]+\(([^)]+)\):'
  SCOPES_HAD_INVALID=0

  while IFS= read -r line; do
    if [[ $line =~ $scope_re ]]; then
      local -a raw_scopes=()
      local raw_scope
      IFS=',' read -r -a raw_scopes <<< "${BASH_REMATCH[1]}"

      for raw_scope in "${raw_scopes[@]}"; do
        local scope="${raw_scope#"${raw_scope%%[![:space:]]*}"}"
        scope="${scope%"${scope##*[![:space:]]}"}"
        [[ -z "$scope" ]] && continue

        local cargo_toml=""
        local -a cargo_tomls=()
        if [[ $scope =~ ^projects/libraries/core$ ]]; then
          local -a core_matches=()
          local core_match
          while IFS= read -r core_match; do
            [[ -n "$core_match" ]] && core_matches+=("$core_match")
          done < <(find projects/libraries/core -mindepth 2 -maxdepth 3 -type f -name Cargo.toml)

          if [[ ${#core_matches[@]} -eq 0 ]]; then
            invalid_scopes=$((invalid_scopes + 1))
            continue
          fi

          cargo_tomls=("${core_matches[@]}")
        elif [[ $scope =~ ^projects/libraries/([^/]+)$ ]]; then
          cargo_toml="projects/libraries/${BASH_REMATCH[1]}/Cargo.toml"
        elif [[ $scope =~ ^projects/libraries/core/([^/]+)/([^/]+)$ ]]; then
          cargo_toml="projects/libraries/core/${BASH_REMATCH[1]}/${BASH_REMATCH[2]}/Cargo.toml"
        elif [[ $scope =~ ^projects/libraries/core/([^/]+)$ ]]; then
          local core_segment="${BASH_REMATCH[1]}"
          local direct_core_cargo_toml="projects/libraries/core/${core_segment}/Cargo.toml"
          if [[ -f "$direct_core_cargo_toml" ]]; then
            cargo_toml="$direct_core_cargo_toml"
          else
            local -a nested_core_matches=()
            local nested_core_match
            while IFS= read -r nested_core_match; do
              [[ -n "$nested_core_match" ]] && nested_core_matches+=("$nested_core_match")
            done < <(find "projects/libraries/core/${core_segment}" -mindepth 1 -maxdepth 1 -type f -name Cargo.toml 2>/dev/null)

            if [[ ${#nested_core_matches[@]} -eq 0 ]]; then
              while IFS= read -r nested_core_match; do
                [[ -n "$nested_core_match" ]] && nested_core_matches+=("$nested_core_match")
              done < <(find "projects/libraries/core/${core_segment}" -mindepth 2 -maxdepth 2 -type f -name Cargo.toml 2>/dev/null)
            fi

            cargo_tomls=("${nested_core_matches[@]}")
          fi
        elif [[ $scope =~ ^projects/products/([^/]+)/([^/]+)/(ui|backend)$ ]]; then
          local tier="${BASH_REMATCH[1]}"
          local product="${BASH_REMATCH[2]}"
          local component="${BASH_REMATCH[3]}"
          local -a matches=()
          local match
          while IFS= read -r match; do
            [[ -n "$match" ]] && matches+=("$match")
          done < <(find projects/products -type f -path "*/${tier}/${product}/${component}/Cargo.toml")

          if [[ ${#matches[@]} -eq 1 ]]; then
            cargo_toml="${matches[0]}"
          elif [[ ${#matches[@]} -gt 1 ]]; then
            echo "⚠️  Scope '$scope' matches multiple Cargo.toml files; falling back to changed-file inference:"
            printf '   - %s\n' "${matches[@]}"
            invalid_scopes=$((invalid_scopes + 1))
            continue
          fi
        elif [[ $scope =~ ^projects/products/([^/]+)/([^/]+)$ ]]; then
          local tier_or_product="${BASH_REMATCH[1]}"
          local product_or_component="${BASH_REMATCH[2]}"
          local -a matches=()
          local match

          while IFS= read -r match; do
            [[ -n "$match" ]] && matches+=("$match")
          done < <(find projects/products -type f -path "*/${tier_or_product}/${product_or_component}/Cargo.toml")

          if [[ ${#matches[@]} -eq 0 ]]; then
            while IFS= read -r match; do
              [[ -n "$match" ]] && matches+=("$match")
            done < <(find projects/products -type f -path "*/${tier_or_product}/${product_or_component}/Cargo.toml")
          fi

          if [[ ${#matches[@]} -eq 1 ]]; then
            cargo_toml="${matches[0]}"
          elif [[ ${#matches[@]} -gt 1 ]]; then
            echo "⚠️  Scope '$scope' matches multiple Cargo.toml files; falling back to changed-file inference:"
            printf '   - %s\n' "${matches[@]}"
            invalid_scopes=$((invalid_scopes + 1))
            continue
          fi
        elif [[ $scope =~ ^projects/products/([^/]+)(/(ui|backend))?$ ]]; then
          local product="${BASH_REMATCH[1]}"
          local component="${BASH_REMATCH[3]:-}"
          local -a matches=()
          local match
          while IFS= read -r match; do
            [[ -n "$match" ]] && matches+=("$match")
          done < <(find projects/products -type f -path "*/${product}/${component:+$component/}Cargo.toml")

          if [[ ${#matches[@]} -eq 1 ]]; then
            cargo_toml="${matches[0]}"
          elif [[ ${#matches[@]} -gt 1 ]]; then
            echo "⚠️  Scope '$scope' matches multiple Cargo.toml files; falling back to changed-file inference:"
            printf '   - %s\n' "${matches[@]}"
            invalid_scopes=$((invalid_scopes + 1))
            continue
          fi
        fi

        if [[ -n "$cargo_toml" ]]; then
          cargo_tomls+=("$cargo_toml")
        fi

        if [[ ${#cargo_tomls[@]} -gt 0 ]]; then
          local ct
          for ct in "${cargo_tomls[@]}"; do
            if [[ ! -f "$ct" ]]; then
              invalid_scopes=$((invalid_scopes + 1))
              continue
            fi

            local crate_name
            crate_name=$(sed -n 's/^name[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$ct" | head -n1)
            if [[ -z "$crate_name" ]]; then
              invalid_scopes=$((invalid_scopes + 1))
              continue
            fi

            if [[ ! " ${crates[*]} " =~ " ${crate_name} " ]]; then
              crates+=("$crate_name")
            fi
          done
        fi
      done
    fi
  done <<< "$commits_input"

  if [[ $invalid_scopes -gt 0 ]]; then
    SCOPES_HAD_INVALID=1
  fi

  if [[ ${#crates[@]} -eq 0 ]]; then
    return 1
  fi

  printf '%s\n' "${crates[@]+"${crates[@]}"}"
}
