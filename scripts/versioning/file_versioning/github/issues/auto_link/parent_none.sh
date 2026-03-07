#!/usr/bin/env bash
# shellcheck disable=SC2016

auto_link_handle_parent_none() {
  local repo_name="$1"
  local repo_owner="$2"
  local repo_short_name="$3"
  local issue_number="$4"
  local marker="$5"
  local label_required_missing="$6"
  local label_automation_failed="$7"

  local current_relation_json
  current_relation_json="$(gh api graphql \
    -f query='query($owner:String!,$name:String!,$child:Int!){repository(owner:$owner,name:$name){child:issue(number:$child){id parent{number id}}}}' \
    -f owner="$repo_owner" \
    -f name="$repo_short_name" \
    -F child="$issue_number" 2>/dev/null || true)"

  if [[ -z "$current_relation_json" ]]; then
    auto_link_fail_runtime \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "Unable to query current parent relation while processing \`Parent: none\`." \
      "Retry later. If this persists, unlink parent manually in GitHub UI."
  fi

  if auto_link_graphql_has_errors "$current_relation_json"; then
    auto_link_fail_runtime_with_graphql_errors \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "GitHub GraphQL query returned errors while reading current parent relation." \
      "$current_relation_json" \
      "Retry later. If this persists, unlink parent manually in GitHub UI."
  fi

  local current_parent_number_none current_parent_node_id_none child_node_id_none
  current_parent_number_none="$(echo "$current_relation_json" | jq -r '.data.repository.child.parent.number // empty')"
  current_parent_node_id_none="$(echo "$current_relation_json" | jq -r '.data.repository.child.parent.id // empty')"
  child_node_id_none="$(echo "$current_relation_json" | jq -r '.data.repository.child.id // empty')"

  if [[ -n "$current_parent_number_none" ]]; then
    if [[ -z "$current_parent_node_id_none" || -z "$child_node_id_none" ]]; then
      auto_link_fail_runtime \
        "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
        "Missing node IDs required to unlink current parent #${current_parent_number_none}." \
        "Retry later. If this persists, unlink parent manually in GitHub UI."
    fi

    local unlink_result_none
    unlink_result_none="$(auto_link_remove_sub_issue_relation "$current_parent_node_id_none" "$child_node_id_none")"

    if [[ -z "$unlink_result_none" ]]; then
      auto_link_fail_runtime \
        "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
        "GitHub API mutation failed while unlinking issue from parent #${current_parent_number_none}." \
        "Retry later. If this persists, unlink parent manually in GitHub UI."
    fi

    if auto_link_graphql_has_errors "$unlink_result_none"; then
      auto_link_fail_runtime_with_graphql_errors \
        "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
        "GitHub GraphQL mutation returned errors while unlinking parent #${current_parent_number_none}." \
        "$unlink_result_none" \
        "Retry later. If this persists, unlink parent manually in GitHub UI."
    fi

    auto_link_set_success_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Removed existing parent link #${current_parent_number_none} (\`Parent: none\`)."
  else
    auto_link_set_success_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "No parent linking requested (\`Parent: none\`)."
  fi

  exit 0
}
