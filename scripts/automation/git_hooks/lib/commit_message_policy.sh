#!/usr/bin/env bash

# Shared commit message policy helpers.

detect_commit_type_from_context() {
  local staged_files="$1"
  local _branch="${2:-}"
  local type=""
  local warning=""

  if is_docs_only_change "$staged_files"; then
    type="docs"
  elif is_tests_only_change "$staged_files"; then
    type="test"
  else
    type="type"
    warning="# WARNING: type not inferred from staged files; replace 'type' with feat/fix/refactor/chore/etc."
  fi

  printf '%s|%s\n' "$type" "$warning"
}

join_scopes_csv() {
  local scopes="$1"
  local csv=""
  local scope
  while IFS= read -r scope; do
    [[ -z "$scope" ]] && continue
    if [[ -z "$csv" ]]; then
      csv="$scope"
    else
      csv="${csv},${scope}"
    fi
  done <<< "$scopes"
  printf '%s' "$csv"
}

slug_to_words() {
  local input="$1"
  local output="$input"
  output="${output//\// }"
  output="${output//_/ }"
  output="${output//-/ }"
  output="$(echo "$output" | sed -E 's/[[:space:]]+/ /g; s/^[[:space:]]+//; s/[[:space:]]+$//')"
  printf '%s' "$output"
}

derive_description() {
  local branch="$1"
  local files="$2"
  local name="$branch"

  name="$(echo "$name" | sed -E 's#^(feat|feature|fix|hotfix|bugfix|docs|doc|refactor|test|tests|chore|perf)/##')"
  name="$(echo "$name" | sed -E 's#^[A-Za-z]+-[0-9]+[-_/]##')"
  name="$(slug_to_words "$name")"

  if [[ -n "$name" ]]; then
    printf '%s' "$name"
    return 0
  fi

  local first_file
  first_file="$(echo "$files" | head -n1)"
  if [[ -n "$first_file" ]]; then
    local stem
    stem="$(basename "$first_file")"
    stem="${stem%.*}"
    stem="$(slug_to_words "$stem")"
    if [[ -n "$stem" ]]; then
      printf 'update %s' "$stem"
      return 0
    fi
  fi

  printf 'update changes'
}

extract_scopes_from_commit_message() {
  local message="$1"
  local scope_re='^[a-z]+\(([^)]+)\):'
  local scope

  if [[ ! "$message" =~ $scope_re ]]; then
    return 0
  fi

  IFS=',' read -r -a scopes <<< "${BASH_REMATCH[1]}"
  for scope in "${scopes[@]}"; do
    scope="${scope#"${scope%%[![:space:]]*}"}"
    scope="${scope%"${scope##*[![:space:]]}"}"
    [[ -n "$scope" ]] && printf '%s\n' "$scope"
  done
}

scope_covers_required_scope() {
  local commit_scope="$1"
  local required_scope="$2"
  [[ "$commit_scope" == "$required_scope" ]]
}
