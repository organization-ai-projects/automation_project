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
# shellcheck source=scripts/automation/git_hooks/lib/hook_utils.sh
source "$HOOKS_DIR/lib/hook_utils.sh"

push_policy_validate_no_root_parent_issue_refs() {
  local commits_input="$1"
  local refs
  refs="$(extract_issue_refs_from_text "$commits_input" || true)"
  [[ -z "$refs" ]] && return 0

  if ! command -v gh >/dev/null 2>&1; then
    echo "❌ Cannot validate root parent issue references: 'gh' CLI is required."
    echo "   Install gh, or bypass in emergency: SKIP_PRE_PUSH=1 git push"
    return 1
  fi

  local repo_name
  repo_name="$(resolve_repo_name_with_owner)"
  if [[ -z "$repo_name" ]]; then
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
    echo "   Root parent refs are forbidden in commit trailers (Part of/Related to/Closes/Fixes/Resolves)."
    echo "   Reference child issues instead."
    return 1
  fi
}

push_policy_validate_part_of_only_push() {
  local commits_input="$1"
  local refs
  local action
  local has_tracking=0
  local has_closing=0

  refs="$(extract_issue_refs_from_text "$commits_input" || true)"
  [[ -z "$refs" ]] && return 0

  while IFS='|' read -r action _; do
    [[ -z "$action" ]] && continue
    if [[ "$action" == "closes" || "$action" == "fixes" || "$action" == "resolves" ]]; then
      has_closing=1
    fi
    if [[ "$action" == "part of" || "$action" == "related to" ]]; then
      has_tracking=1
    fi
  done <<< "$refs"

  if [[ $has_tracking -eq 1 && $has_closing -eq 0 && "${ALLOW_PART_OF_ONLY_PUSH:-}" != "1" ]]; then
    echo "❌ Push blocked: commit range contains tracking refs (Part of/Related to) without any closing ref."
    echo "   This usually indicates incomplete issue lifecycle updates."
    echo "   Fix by adding proper child issue closures when done, or confirm exceptional push with:"
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
        if [[ $scope =~ ^projects/libraries/core$ ]]; then
          local -a core_matches=()
          local core_match
          while IFS= read -r core_match; do
            [[ -n "$core_match" ]] && core_matches+=("$core_match")
          done < <(find projects/libraries/core -mindepth 2 -maxdepth 2 -type f -name Cargo.toml)

          if [[ ${#core_matches[@]} -eq 0 ]]; then
            invalid_scopes=$((invalid_scopes + 1))
            continue
          fi

          local cm
          for cm in "${core_matches[@]}"; do
            local core_crate_name
            core_crate_name=$(sed -n 's/^name[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$cm" | head -n1)
            if [[ -z "$core_crate_name" ]]; then
              invalid_scopes=$((invalid_scopes + 1))
              continue
            fi
            if [[ ! " ${crates[*]} " =~ " ${core_crate_name} " ]]; then
              crates+=("$core_crate_name")
            fi
          done
          continue
        elif [[ $scope =~ ^projects/libraries/([^/]+)$ ]]; then
          cargo_toml="projects/libraries/${BASH_REMATCH[1]}/Cargo.toml"
        elif [[ $scope =~ ^projects/libraries/core/([^/]+)$ ]]; then
          cargo_toml="projects/libraries/core/${BASH_REMATCH[1]}/Cargo.toml"
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
          if [[ ! -f "$cargo_toml" ]]; then
            invalid_scopes=$((invalid_scopes + 1))
            continue
          fi

          local crate_name
          crate_name=$(sed -n 's/^name[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' "$cargo_toml" | head -n1)
          if [[ -z "$crate_name" ]]; then
            invalid_scopes=$((invalid_scopes + 1))
            continue
          fi

          if [[ ! " ${crates[*]} " =~ " ${crate_name} " ]]; then
            crates+=("$crate_name")
          fi
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
