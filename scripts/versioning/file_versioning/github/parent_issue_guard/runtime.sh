#!/usr/bin/env bash
# shellcheck disable=SC2034

parent_guard_init_repo_context() {
  local repo_name_var_name="$1"
  local repo_owner_var_name="$2"
  local repo_short_name_var_name="$3"

  local -n repo_name_ref="$repo_name_var_name"
  local -n repo_owner_ref="$repo_owner_var_name"
  local -n repo_short_name_ref="$repo_short_name_var_name"

  repo_name_ref="${GH_REPO:-}"
  if [[ -z "$repo_name_ref" ]]; then
    repo_name_ref="$(gh_cli_resolve_repo_name)"
  fi
  if [[ -z "$repo_name_ref" ]]; then
    echo "Erreur: impossible de déterminer le repository (GH_REPO)." >&2
    exit 3
  fi

  repo_owner_ref="${repo_name_ref%%/*}"
  repo_short_name_ref="${repo_name_ref#*/}"
}
