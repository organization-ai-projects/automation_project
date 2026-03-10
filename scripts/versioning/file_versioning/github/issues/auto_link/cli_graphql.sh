#!/usr/bin/env bash

auto_link_usage() {
  cat <<USAGE
Usage:
  issues/auto_link/run.sh --issue ISSUE_NUMBER

Notes:
  - Reads "Parent: #<number> | none | base | epic" from issue body.
  - Attempts to link child -> parent as GitHub sub-issue via GraphQL.
  - On invalid input or API linking failure, posts actionable status comment and labels issue.
USAGE
}

auto_link_trim() {
  local s="${1:-}"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf "%s" "$s"
}

auto_link_graphql_has_errors() {
  local payload="${1:-}"
  jq -e '((.errors // []) | length) > 0' >/dev/null 2>&1 <<<"$payload"
}

auto_link_graphql_error_messages() {
  local payload="${1:-}"
  jq -r '(.errors // []) | map(.message // "unknown GraphQL error") | join("; ")' <<<"$payload" 2>/dev/null || true
}

auto_link_extract_parent_field_value() {
  local body="${1:-}"
  awk '
    BEGIN { IGNORECASE = 1 }
    /^[[:space:]]*Parent[[:space:]]*:/ {
      line = $0
      sub(/^[[:space:]]*Parent[[:space:]]*:[[:space:]]*/, "", line)
      print line
      exit
    }
  ' <<<"$body"
}

auto_link_require_deps() {
  issue_gh_require_gh_and_jq
}

auto_link_graphql_api() {
  gh api graphql "$@" 2>/dev/null || true
}

auto_link_query_child_parent_relation() {
  local repo_owner="$1"
  local repo_short_name="$2"
  local issue_number="$3"

  auto_link_graphql_api \
    -f query='query($owner:String!,$name:String!,$child:Int!){repository(owner:$owner,name:$name){child:issue(number:$child){id parent{number id}}}}' \
    -f owner="$repo_owner" \
    -f name="$repo_short_name" \
    -F child="$issue_number"
}

auto_link_query_parent_child_relation() {
  local repo_owner="$1"
  local repo_short_name="$2"
  local child_issue_number="$3"
  local parent_issue_number="$4"

  auto_link_graphql_api \
    -f query='query($owner:String!,$name:String!,$child:Int!,$parent:Int!){repository(owner:$owner,name:$name){child:issue(number:$child){id parent{number id}} parent:issue(number:$parent){id state}}}' \
    -f owner="$repo_owner" \
    -f name="$repo_short_name" \
    -F child="$child_issue_number" \
    -F parent="$parent_issue_number"
}

auto_link_remove_sub_issue_relation() {
  local parent_node_id="$1"
  local child_node_id="$2"

  auto_link_graphql_api \
    -f query='mutation($issueId:ID!,$subIssueId:ID!){removeSubIssue(input:{issueId:$issueId,subIssueId:$subIssueId}){issue{id}}}' \
    -f issueId="$parent_node_id" \
    -f subIssueId="$child_node_id"
}

auto_link_add_sub_issue_relation() {
  local parent_node_id="$1"
  local child_node_id="$2"

  auto_link_graphql_api \
    -f query='mutation($issueId:ID!,$subIssueId:ID!){addSubIssue(input:{issueId:$issueId,subIssueId:$subIssueId}){issue{subIssues(first:1){nodes{number}}}}}' \
    -f issueId="$parent_node_id" \
    -f subIssueId="$child_node_id"
}
