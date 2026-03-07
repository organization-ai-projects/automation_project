#!/usr/bin/env bash
# shellcheck disable=SC2016

auto_link_fail_validation() {
  local repo_name="$1"
  local issue_number="$2"
  local marker="$3"
  local label_required_missing="$4"
  local label_automation_failed="$5"
  local summary="$6"
  local next_steps="$7"

  auto_link_set_validation_error_state \
    "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
    "$summary" "$next_steps"
  exit 0
}

auto_link_fail_runtime() {
  local repo_name="$1"
  local issue_number="$2"
  local marker="$3"
  local label_automation_failed="$4"
  local summary="$5"
  local next_steps="$6"

  auto_link_set_runtime_error_state \
    "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
    "$summary" "$next_steps"
  exit 0
}

auto_link_fail_runtime_with_graphql_errors() {
  local repo_name="$1"
  local issue_number="$2"
  local marker="$3"
  local label_automation_failed="$4"
  local summary="$5"
  local errors_payload="$6"
  local next_steps="$7"
  local relation_errors

  relation_errors="$(auto_link_graphql_error_messages "$errors_payload")"
  auto_link_fail_runtime \
    "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
    "$summary" \
    "API errors: ${relation_errors}

${next_steps}"
}

auto_link_handle_parent_link() {
  local repo_name="$1"
  local repo_owner="$2"
  local repo_short_name="$3"
  local issue_number="$4"
  local parent_number="$5"
  local marker="$6"
  local label_required_missing="$7"
  local label_automation_failed="$8"

  if [[ "$parent_number" == "$issue_number" ]]; then
    auto_link_fail_validation \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Issue cannot reference itself as parent (\`Parent: #${issue_number}\`)." \
      "Use another parent issue number or \`Parent: none\`."
  fi

  local parent_json
  parent_json="$(gh issue view "$parent_number" -R "$repo_name" --json number,title,state,url 2>/dev/null || true)"
  if [[ -z "$parent_json" ]]; then
    auto_link_fail_validation \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Parent issue \`#${parent_number}\` was not found." \
      "Use an existing issue number in \`Parent:\`."
  fi

  local parent_state
  parent_state="$(echo "$parent_json" | jq -r '.state // ""')"
  if [[ "$parent_state" != "OPEN" ]]; then
    auto_link_fail_validation \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Parent issue \`#${parent_number}\` is not open (state: ${parent_state})." \
      "Reopen the parent or choose another open parent issue."
  fi

  local relation_json
  relation_json="$(gh api graphql \
    -f query='query($owner:String!,$name:String!,$child:Int!,$parent:Int!){repository(owner:$owner,name:$name){child:issue(number:$child){id parent{number id}} parent:issue(number:$parent){id state}}}' \
    -f owner="$repo_owner" \
    -f name="$repo_short_name" \
    -F child="$issue_number" \
    -F parent="$parent_number" 2>/dev/null || true)"

  if [[ -z "$relation_json" ]]; then
    auto_link_fail_runtime \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "Unable to query parent/child relation state from GitHub API." \
      "Retry later. If this persists, link the issue manually in GitHub UI."
  fi

  if auto_link_graphql_has_errors "$relation_json"; then
    auto_link_fail_runtime_with_graphql_errors \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "GitHub GraphQL query returned errors while reading relation state." \
      "$relation_json" \
      "Retry later. If this persists, link the issue manually in GitHub UI."
  fi

  local current_parent_number current_parent_node_id child_node_id parent_node_id
  current_parent_number="$(echo "$relation_json" | jq -r '.data.repository.child.parent.number // empty')"
  current_parent_node_id="$(echo "$relation_json" | jq -r '.data.repository.child.parent.id // empty')"
  child_node_id="$(echo "$relation_json" | jq -r '.data.repository.child.id // empty')"
  parent_node_id="$(echo "$relation_json" | jq -r '.data.repository.parent.id // empty')"

  if [[ -n "$current_parent_number" && "$current_parent_number" == "$parent_number" ]]; then
    auto_link_set_success_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Issue already linked to parent #${parent_number}."
    exit 0
  fi

  if [[ -n "$current_parent_number" && "$current_parent_number" != "$parent_number" ]]; then
    if [[ -z "$current_parent_node_id" || -z "$child_node_id" ]]; then
      auto_link_fail_runtime \
        "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
        "Missing node IDs required to re-parent issue from #${current_parent_number} to #${parent_number}." \
        "Retry later. If this persists, update parent linkage manually in GitHub UI."
    fi

    local unlink_result
    unlink_result="$(gh api graphql \
      -f query='mutation($issueId:ID!,$subIssueId:ID!){removeSubIssue(input:{issueId:$issueId,subIssueId:$subIssueId}){issue{id}}}' \
      -f issueId="$current_parent_node_id" \
      -f subIssueId="$child_node_id" 2>/dev/null || true)"

    if [[ -z "$unlink_result" ]]; then
      auto_link_fail_runtime \
        "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
        "GitHub API mutation failed while unlinking child from previous parent #${current_parent_number}." \
        "Retry later. If this persists, unlink manually in GitHub UI and rerun automation."
    fi

    if auto_link_graphql_has_errors "$unlink_result"; then
      auto_link_fail_runtime_with_graphql_errors \
        "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
        "GitHub GraphQL mutation returned errors while unlinking previous parent #${current_parent_number}." \
        "$unlink_result" \
        "Retry later. If this persists, unlink manually in GitHub UI and rerun automation."
    fi
  fi

  if [[ -z "$child_node_id" || -z "$parent_node_id" ]]; then
    auto_link_fail_runtime \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "Missing GitHub node IDs required for sub-issue linking." \
      "Retry later. If this persists, link parent/child manually in GitHub UI."
  fi

  local link_result
  link_result="$(gh api graphql \
    -f query='mutation($issueId:ID!,$subIssueId:ID!){addSubIssue(input:{issueId:$issueId,subIssueId:$subIssueId}){issue{subIssues(first:1){nodes{number}}}}}' \
    -f issueId="$parent_node_id" \
    -f subIssueId="$child_node_id" 2>/dev/null || true)"

  if [[ -z "$link_result" ]]; then
    auto_link_fail_runtime \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "GitHub API mutation failed while linking child to parent." \
      "Link manually in GitHub UI, then keep \`Parent: #${parent_number}\` in issue body for traceability."
  fi

  if auto_link_graphql_has_errors "$link_result"; then
    auto_link_fail_runtime_with_graphql_errors \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "GitHub GraphQL mutation returned errors while linking child to parent." \
      "$link_result" \
      "Link manually in GitHub UI, then keep \`Parent: #${parent_number}\` in issue body for traceability."
  fi

  local linked_child_number
  linked_child_number="$(echo "$link_result" | jq -r '.data.addSubIssue.issue.subIssues.nodes[0].number // empty')"
  if [[ -z "$linked_child_number" ]]; then
    auto_link_fail_runtime \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "GitHub mutation returned no linked sub-issue confirmation." \
      "Retry later. If this persists, link manually in GitHub UI and keep \`Parent: #${parent_number}\` in issue body."
  fi

  if [[ -n "$current_parent_number" && "$current_parent_number" != "$parent_number" ]]; then
    auto_link_set_success_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Re-parented this issue from #${current_parent_number} to #${parent_number}."
    echo "Re-parented issue #${issue_number} from #${current_parent_number} to #${parent_number}."
  else
    auto_link_set_success_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Linked this issue as child of #${parent_number}."
    echo "Linked issue #${issue_number} to parent #${parent_number}."
  fi
}
