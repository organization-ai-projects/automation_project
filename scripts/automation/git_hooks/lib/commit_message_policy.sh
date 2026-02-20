#!/usr/bin/env bash

# Shared commit message policy helpers.

is_docs_only_change() {
  local files="$1"
  local file

  [[ -z "$files" ]] && return 1

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    if [[ "$file" == documentation/* ]] || [[ "$file" == .github/documentation/* ]] || [[ "$file" == .github/ISSUE_TEMPLATE/* ]] || [[ "$file" == .github/PULL_REQUEST_TEMPLATE/* ]] || [[ "$file" == *.md ]]; then
      continue
    fi
    return 1
  done <<< "$files"

  return 0
}

is_tests_only_change() {
  local files="$1"
  local file

  [[ -z "$files" ]] && return 1

  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    if [[ "$file" == *"/tests/"* ]] || [[ "$file" == *"_test.rs" ]] || [[ "$file" == *"/tests.rs" ]]; then
      continue
    fi
    return 1
  done <<< "$files"

  return 0
}

map_branch_type() {
  local branch="$1"
  local prefix="${branch%%/*}"
  prefix="$(printf '%s' "$prefix" | tr '[:upper:]' '[:lower:]')"
  case "$prefix" in
    feat|feature) echo "feat" ;;
    fix|hotfix|bugfix) echo "fix" ;;
    docs|doc) echo "docs" ;;
    refactor) echo "refactor" ;;
    test|tests) echo "test" ;;
    chore) echo "chore" ;;
    ci) echo "ci" ;;
    perf) echo "perf" ;;
    build) echo "build" ;;
    *) echo "" ;;
  esac
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

  name="$(echo "$name" | sed -E 's#^(feat|feature|fix|hotfix|bugfix|docs|doc|refactor|test|tests|chore|ci|perf|build)/##')"
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
