#!/usr/bin/env bash

# Workspace-first Rust scope resolution helpers.

workspace_rust_member_dirs() {
  local repo_root
  repo_root="$(git rev-parse --show-toplevel 2>/dev/null || true)"
  [[ -n "$repo_root" ]] || return 1

  local workspace_dirs=""
  workspace_dirs="$(workspace_rust_member_dirs_from_root_toml "$repo_root" || true)"
  if [[ -n "$workspace_dirs" ]]; then
    printf '%s\n' "$workspace_dirs"
    return 0
  fi

  # Fallback: cargo metadata when workspace.members parsing is unavailable.
  workspace_rust_member_dirs_from_metadata "$repo_root"
}

workspace_rust_member_dirs_from_root_toml() {
  local repo_root="$1"
  local root_toml="${repo_root}/Cargo.toml"
  [[ -f "$root_toml" ]] || return 1

  local patterns=""
  patterns="$(workspace_rust_member_patterns_from_root_toml "$root_toml" || true)"
  [[ -n "$patterns" ]] || return 1

  (
    cd "$repo_root" || exit 1
    while IFS= read -r pattern; do
      [[ -z "$pattern" ]] && continue
      workspace_rust_expand_member_pattern "$pattern"
    done <<< "$patterns"
  ) | sed '/^$/d' | sort -u
}

workspace_rust_member_patterns_from_root_toml() {
  local root_toml="$1"

  awk '
    BEGIN {
      in_workspace = 0
      in_members = 0
    }
    /^\[workspace\][[:space:]]*$/ {
      in_workspace = 1
      in_members = 0
      next
    }
    /^\[[^]]+\][[:space:]]*$/ {
      if (in_workspace) {
        in_workspace = 0
        in_members = 0
      }
    }
    {
      if (!in_workspace) next
      line = $0
      sub(/[[:space:]]*#.*/, "", line)

      if (!in_members && line ~ /^[[:space:]]*members[[:space:]]*=/) {
        in_members = 1
        sub(/^[[:space:]]*members[[:space:]]*=[[:space:]]*/, "", line)
      } else if (!in_members) {
        next
      }

      while (match(line, /"[^"]+"/)) {
        token = substr(line, RSTART + 1, RLENGTH - 2)
        if (token != "") print token
        line = substr(line, RSTART + RLENGTH)
      }

      if (line ~ /\]/) {
        in_members = 0
      }
    }
  ' "$root_toml"
}

workspace_rust_expand_member_pattern() {
  local pattern="$1"
  local has_glob=0
  local match_path

  [[ "$pattern" == *"*"* || "$pattern" == *"?"* || "$pattern" == *"["* ]] && has_glob=1

  if [[ $has_glob -eq 1 ]]; then
    while IFS= read -r match_path; do
      [[ -z "$match_path" ]] && continue
      workspace_rust_emit_member_dir_if_cargo "$match_path"
    done < <(compgen -G "$pattern" || true)
    return 0
  fi

  workspace_rust_emit_member_dir_if_cargo "$pattern"
}

workspace_rust_emit_member_dir_if_cargo() {
  local path="$1"
  local member_dir="$path"

  if [[ "$path" == */Cargo.toml ]]; then
    member_dir="${path%/Cargo.toml}"
  fi

  [[ -f "${member_dir}/Cargo.toml" ]] || return 0
  [[ "$member_dir" == projects/* ]] || return 0
  printf '%s\n' "$member_dir"
}

workspace_rust_member_dirs_from_metadata() {
  local repo_root="$1"

  cargo metadata --no-deps --format-version 1 2>/dev/null \
    | tr ',' '\n' \
    | sed -n 's/.*"manifest_path":"\([^"]*\)".*/\1/p' \
    | sed 's#\\/#/#g' \
    | while IFS= read -r manifest_path; do
        [[ -z "$manifest_path" ]] && continue
        local crate_dir="${manifest_path%/Cargo.toml}"
        local rel_dir="$crate_dir"
        case "$crate_dir" in
          "$repo_root"/*) rel_dir="${crate_dir#"$repo_root"/}" ;;
        esac
        [[ "$rel_dir" == projects/* ]] && printf '%s\n' "$rel_dir"
      done \
    | sort -u
}

workspace_rust_load_member_dirs() {
  if [[ -n "${WORKSPACE_RUST_MEMBER_DIRS_LOADED:-}" ]]; then
    return 0
  fi

  WORKSPACE_RUST_MEMBER_DIRS="$(workspace_rust_member_dirs || true)"
  WORKSPACE_RUST_MEMBER_DIRS_LOADED=1
}

workspace_rust_resolve_scope_from_members() {
  local file="$1"
  local dir="$file"

  workspace_rust_load_member_dirs
  [[ -n "${WORKSPACE_RUST_MEMBER_DIRS:-}" ]] || return 1

  if [[ ! -d "$dir" ]]; then
    dir="$(dirname "$file")"
  fi

  while :; do
    if grep -Fxq "$dir" <<< "$WORKSPACE_RUST_MEMBER_DIRS"; then
      printf '%s\n' "$dir"
      return 0
    fi
    [[ "$dir" == "." || "$dir" == "/" ]] && break
    dir="$(dirname "$dir")"
  done

  return 1
}
