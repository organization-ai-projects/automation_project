#!/usr/bin/env bash
# shellcheck disable=SC2016

auto_link_run() {
  local issue_arg=""

  while [[ $# -gt 0 ]]; do
    case "$1" in
    --issue)
      issue_arg="${2:-}"
      shift 2
      ;;
    -h | --help)
      auto_link_usage
      exit 0
      ;;
    *)
      echo "Erreur: option inconnue: $1" >&2
      auto_link_usage >&2
      exit 2
      ;;
    esac
  done

  if [[ -z "$issue_arg" ]]; then
    echo "Erreur: --issue est requis." >&2
    auto_link_usage >&2
    exit 2
  fi

  auto_link_require_number "--issue" "$issue_arg"
  auto_link_require_deps

  local repo_name="${GH_REPO:-}"
  if [[ -z "$repo_name" ]]; then
    repo_name="$(gh repo view --json nameWithOwner -q '.nameWithOwner' 2>/dev/null || true)"
  fi
  if [[ -z "$repo_name" ]]; then
    echo "Erreur: impossible de déterminer le repository (GH_REPO)." >&2
    exit 3
  fi

  local repo_owner="${repo_name%%/*}"
  local repo_short_name="${repo_name#*/}"
  local issue_number="$issue_arg"

  local marker="<!-- parent-field-autolink:${issue_number} -->"
  local label_required_missing="issue-required-missing"
  local label_automation_failed="automation-failed"

  local issue_json
  issue_json="$(gh issue view "$issue_number" -R "$repo_name" --json number,title,body,state,url,labels 2>/dev/null || true)"
  if [[ -z "$issue_json" ]]; then
    echo "Erreur: impossible de lire l'issue #${issue_number}." >&2
    exit 4
  fi

  local issue_title issue_body issue_labels_raw
  issue_title="$(echo "$issue_json" | jq -r '.title // ""')"
  issue_body="$(echo "$issue_json" | jq -r '.body // ""')"
  issue_labels_raw="$(echo "$issue_json" | jq -r '(.labels // []) | map(.name) | join("||")')"

  local contract_errors
  contract_errors="$(issue_validate_content "$issue_title" "$issue_body" "$issue_labels_raw" || true)"
  if [[ -n "$contract_errors" ]]; then
    local summary_lines=""
    while IFS='|' read -r _ _ message; do
      [[ -z "$message" ]] && continue
      summary_lines+="- ${message}"$'\n'
    done <<<"$contract_errors"
    auto_link_set_validation_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Issue body/title is non-compliant with required issue format." \
      "Detected problems:

${summary_lines}
Expected contract source: \`.github/issue_required_fields.conf\`."
    exit 0
  fi

  local parent_raw
  parent_raw="$(auto_link_extract_parent_field_value "$issue_body")"
  parent_raw="$(auto_link_trim "${parent_raw:-}")"

  if [[ -z "$parent_raw" ]]; then
    auto_link_set_validation_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Missing required field \`Parent:\` in issue body." \
      "Expected format:
\n- \`Parent: #<issue_number>\` for child issues
\n- \`Parent: none\` for root/parent issues"
    exit 0
  fi

  local parent_raw_lc
  parent_raw_lc="$(echo "$parent_raw" | tr '[:upper:]' '[:lower:]')"
  if [[ "$parent_raw_lc" == "none" ]]; then
    local current_relation_json
    current_relation_json="$(gh api graphql \
      -f query='query($owner:String!,$name:String!,$child:Int!){repository(owner:$owner,name:$name){child:issue(number:$child){id parent{number id}}}}' \
      -f owner="$repo_owner" \
      -f name="$repo_short_name" \
      -F child="$issue_number" 2>/dev/null || true)"

    if [[ -z "$current_relation_json" ]]; then
      auto_link_set_runtime_error_state \
        "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
        "Unable to query current parent relation while processing \`Parent: none\`." \
        "Retry later. If this persists, unlink parent manually in GitHub UI."
      exit 0
    fi

    if auto_link_graphql_has_errors "$current_relation_json"; then
      local relation_errors
      relation_errors="$(auto_link_graphql_error_messages "$current_relation_json")"
      auto_link_set_runtime_error_state \
        "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
        "GitHub GraphQL query returned errors while reading current parent relation." \
        "API errors: ${relation_errors}

Retry later. If this persists, unlink parent manually in GitHub UI."
      exit 0
    fi

    local current_parent_number_none current_parent_node_id_none child_node_id_none
    current_parent_number_none="$(echo "$current_relation_json" | jq -r '.data.repository.child.parent.number // empty')"
    current_parent_node_id_none="$(echo "$current_relation_json" | jq -r '.data.repository.child.parent.id // empty')"
    child_node_id_none="$(echo "$current_relation_json" | jq -r '.data.repository.child.id // empty')"

    if [[ -n "$current_parent_number_none" ]]; then
      if [[ -z "$current_parent_node_id_none" || -z "$child_node_id_none" ]]; then
        auto_link_set_runtime_error_state \
          "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
          "Missing node IDs required to unlink current parent #${current_parent_number_none}." \
          "Retry later. If this persists, unlink parent manually in GitHub UI."
        exit 0
      fi

      local unlink_result_none
      unlink_result_none="$(gh api graphql \
        -f query='mutation($issueId:ID!,$subIssueId:ID!){removeSubIssue(input:{issueId:$issueId,subIssueId:$subIssueId}){issue{id}}}' \
        -f issueId="$current_parent_node_id_none" \
        -f subIssueId="$child_node_id_none" 2>/dev/null || true)"

      if [[ -z "$unlink_result_none" ]]; then
        auto_link_set_runtime_error_state \
          "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
          "GitHub API mutation failed while unlinking issue from parent #${current_parent_number_none}." \
          "Retry later. If this persists, unlink parent manually in GitHub UI."
        exit 0
      fi

      if auto_link_graphql_has_errors "$unlink_result_none"; then
        local unlink_errors
        unlink_errors="$(auto_link_graphql_error_messages "$unlink_result_none")"
        auto_link_set_runtime_error_state \
          "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
          "GitHub GraphQL mutation returned errors while unlinking parent #${current_parent_number_none}." \
          "API errors: ${unlink_errors}

Retry later. If this persists, unlink parent manually in GitHub UI."
        exit 0
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
  fi

  if [[ ! "$parent_raw" =~ ^#[0-9]+$ ]]; then
    auto_link_set_validation_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Invalid \`Parent:\` value: \`${parent_raw}\`." \
      "Expected \`Parent: #<issue_number>\` or \`Parent: none\`."
    exit 0
  fi

  local parent_number
  parent_number="${parent_raw//#/}"
  if [[ "$parent_number" == "$issue_number" ]]; then
    auto_link_set_validation_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Issue cannot reference itself as parent (\`Parent: #${issue_number}\`)." \
      "Use another parent issue number or \`Parent: none\`."
    exit 0
  fi

  local parent_json
  parent_json="$(gh issue view "$parent_number" -R "$repo_name" --json number,title,state,url 2>/dev/null || true)"
  if [[ -z "$parent_json" ]]; then
    auto_link_set_validation_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Parent issue \`#${parent_number}\` was not found." \
      "Use an existing issue number in \`Parent:\`."
    exit 0
  fi

  local parent_state
  parent_state="$(echo "$parent_json" | jq -r '.state // ""')"
  if [[ "$parent_state" != "OPEN" ]]; then
    auto_link_set_validation_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_required_missing" "$label_automation_failed" \
      "Parent issue \`#${parent_number}\` is not open (state: ${parent_state})." \
      "Reopen the parent or choose another open parent issue."
    exit 0
  fi

  local relation_json
  relation_json="$(gh api graphql \
    -f query='query($owner:String!,$name:String!,$child:Int!,$parent:Int!){repository(owner:$owner,name:$name){child:issue(number:$child){id parent{number id}} parent:issue(number:$parent){id state}}}' \
    -f owner="$repo_owner" \
    -f name="$repo_short_name" \
    -F child="$issue_number" \
    -F parent="$parent_number" 2>/dev/null || true)"

  if [[ -z "$relation_json" ]]; then
    auto_link_set_runtime_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "Unable to query parent/child relation state from GitHub API." \
      "Retry later. If this persists, link the issue manually in GitHub UI."
    exit 0
  fi

  if auto_link_graphql_has_errors "$relation_json"; then
    local relation_errors
    relation_errors="$(auto_link_graphql_error_messages "$relation_json")"
    auto_link_set_runtime_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "GitHub GraphQL query returned errors while reading relation state." \
      "API errors: ${relation_errors}

Retry later. If this persists, link the issue manually in GitHub UI."
    exit 0
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
      auto_link_set_runtime_error_state \
        "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
        "Missing node IDs required to re-parent issue from #${current_parent_number} to #${parent_number}." \
        "Retry later. If this persists, update parent linkage manually in GitHub UI."
      exit 0
    fi

    local unlink_result
    unlink_result="$(gh api graphql \
      -f query='mutation($issueId:ID!,$subIssueId:ID!){removeSubIssue(input:{issueId:$issueId,subIssueId:$subIssueId}){issue{id}}}' \
      -f issueId="$current_parent_node_id" \
      -f subIssueId="$child_node_id" 2>/dev/null || true)"

    if [[ -z "$unlink_result" ]]; then
      auto_link_set_runtime_error_state \
        "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
        "GitHub API mutation failed while unlinking child from previous parent #${current_parent_number}." \
        "Retry later. If this persists, unlink manually in GitHub UI and rerun automation."
      exit 0
    fi

    if auto_link_graphql_has_errors "$unlink_result"; then
      local unlink_errors
      unlink_errors="$(auto_link_graphql_error_messages "$unlink_result")"
      auto_link_set_runtime_error_state \
        "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
        "GitHub GraphQL mutation returned errors while unlinking previous parent #${current_parent_number}." \
        "API errors: ${unlink_errors}

Retry later. If this persists, unlink manually in GitHub UI and rerun automation."
      exit 0
    fi
  fi

  if [[ -z "$child_node_id" || -z "$parent_node_id" ]]; then
    auto_link_set_runtime_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "Missing GitHub node IDs required for sub-issue linking." \
      "Retry later. If this persists, link parent/child manually in GitHub UI."
    exit 0
  fi

  local link_result
  link_result="$(gh api graphql \
    -f query='mutation($issueId:ID!,$subIssueId:ID!){addSubIssue(input:{issueId:$issueId,subIssueId:$subIssueId}){issue{subIssues(first:1){nodes{number}}}}}' \
    -f issueId="$parent_node_id" \
    -f subIssueId="$child_node_id" 2>/dev/null || true)"

  if [[ -z "$link_result" ]]; then
    auto_link_set_runtime_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "GitHub API mutation failed while linking child to parent." \
      "Link manually in GitHub UI, then keep \`Parent: #${parent_number}\` in issue body for traceability."
    exit 0
  fi

  if auto_link_graphql_has_errors "$link_result"; then
    local link_errors
    link_errors="$(auto_link_graphql_error_messages "$link_result")"
    auto_link_set_runtime_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "GitHub GraphQL mutation returned errors while linking child to parent." \
      "API errors: ${link_errors}

Link manually in GitHub UI, then keep \`Parent: #${parent_number}\` in issue body for traceability."
    exit 0
  fi

  local linked_child_number
  linked_child_number="$(echo "$link_result" | jq -r '.data.addSubIssue.issue.subIssues.nodes[0].number // empty')"
  if [[ -z "$linked_child_number" ]]; then
    auto_link_set_runtime_error_state \
      "$repo_name" "$issue_number" "$marker" "$label_automation_failed" \
      "GitHub mutation returned no linked sub-issue confirmation." \
      "Retry later. If this persists, link manually in GitHub UI and keep \`Parent: #${parent_number}\` in issue body."
    exit 0
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
