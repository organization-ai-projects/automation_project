#!/usr/bin/env bash
# shellcheck disable=SC2016

auto_link_handle_parent_none() {
  local repo_name="$1"
  local repo_owner="$2"
  local repo_short_name="$3"
  local issue_number="$4"
  local parent_mode="$5"
  local marker="$6"
  local label_required_missing="$7"
  local label_automation_failed="$8"

  local current_relation_json
  current_relation_json="$(auto_link_query_child_parent_relation \
    "$repo_owner" "$repo_short_name" "$issue_number")"
  auto_link_validate_graphql_runtime_result \
    "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
    "$current_relation_json" \
    "Unable to query current parent relation while processing \`Parent: ${parent_mode}\`." \
    "GitHub GraphQL query returned errors while reading current parent relation." \
    "Retry later. If this persists, unlink parent manually in GitHub UI."

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
    auto_link_validate_graphql_runtime_result \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "$unlink_result_none" \
      "GitHub API mutation failed while unlinking issue from parent #${current_parent_number_none}." \
      "GitHub GraphQL mutation returned errors while unlinking parent #${current_parent_number_none}." \
      "Retry later. If this persists, unlink parent manually in GitHub UI."

    auto_link_set_success_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Removed existing parent link #${current_parent_number_none} (\`Parent: ${parent_mode}\`)."
  else
    auto_link_set_success_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "No parent linking requested (\`Parent: ${parent_mode}\`)."
  fi

  exit 0
}
