#!/usr/bin/env bash
# shellcheck disable=SC2016

parent_guard_extract_parent_ref_from_github() {
  local repo_owner="$1"
  local repo_short_name="$2"
  local child_number="$3"

  gh api graphql \
    -f query='query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){issue(number:$number){parent{number}}}}' \
    -f owner="$repo_owner" \
    -f name="$repo_short_name" \
    -F number="$child_number" \
    --jq '.data.repository.issue.parent.number // empty | "#"+tostring' 2>/dev/null || true
}

parent_guard_collect_parent_candidates() {
  local repo_name="$1"
  local repo_owner="$2"
  local repo_short_name="$3"
  local child_arg="$4"

  mapfile -t parent_candidates < <(parent_guard_extract_parent_ref_from_github "$repo_owner" "$repo_short_name" "$child_arg" | sed 's/^#//')
  if [[ ${#parent_candidates[@]} -eq 0 ]]; then
    mapfile -t parent_candidates < <(
      gh api "search/issues" \
        -f q="repo:${repo_name} is:issue \"#${child_arg}\"" \
        --jq '.items[].number' 2>/dev/null | sort -u
    )
  fi

  printf '%s\n' "${parent_candidates[@]}"
}
