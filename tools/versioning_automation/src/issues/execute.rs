//! tools/versioning_automation/src/issues/execute.rs
use std::collections::{self, HashMap, HashSet};

use regex::Regex;
use serde::Deserialize;

use crate::gh_cli::{gh_issue_target_command, output_trim_or_empty, push_arg, status_code_owned};
use crate::issue_comment_upsert::upsert_issue_comment_by_marker;
use crate::issue_remote_snapshot::IssueRemoteSnapshot;
use crate::issues::commands::{CloseOptions, IssueTarget, UpsertMarkerCommentOptions};
use crate::issues::{self, AutoLinkRelationSnapshot, Validation, extract_tasklist_refs};

use crate::parent_field::extract_parent_field;
use crate::pr::{extract_effective_action_issue_numbers, load_pr_text_payload};
use crate::pr_remote_snapshot::load_pr_remote_snapshot;
use crate::repo_name::resolve_repo_name;

pub(crate) fn pr_state_allows_reopen_sync(state: &str) -> bool {
    matches!(state, "OPEN" | "MERGED")
}

pub(crate) fn load_effective_issue_action_numbers_for_pr(
    pr_number: &str,
    repo_name: &str,
) -> Result<(collections::BTreeSet<String>, collections::BTreeSet<String>), i32> {
    let payload = match load_pr_text_payload(pr_number, repo_name) {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Error: unable to read PR #{}.", pr_number);
            return Err(4);
        }
    };

    Ok(extract_effective_action_issue_numbers(&payload))
}

pub(crate) fn has_label_named(labels_raw: &str, expected_label: &str) -> bool {
    labels_raw
        .split('|')
        .map(str::trim)
        .any(|label| label == expected_label)
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn run_auto_link_parent_none(
    repo_name: &str,
    repo_owner: &str,
    repo_short_name: &str,
    issue_number: &str,
    parent_mode: &str,
    marker: &str,
    label_required_missing: &str,
    label_automation_failed: &str,
) -> i32 {
    let relation_json =
        auto_link_query_child_parent_relation(repo_owner, repo_short_name, issue_number);
    if relation_json.trim().is_empty() {
        let status = auto_link_set_runtime_error_state(
            repo_name,
            issue_number,
            marker,
            label_automation_failed,
            &format!(
                "Unable to query current parent relation while processing `Parent: {parent_mode}`."
            ),
            "Retry later. If this persists, unlink parent manually in GitHub UI.",
        );
        return if status == 0 { 0 } else { status };
    }
    if auto_link_graphql_has_errors(&relation_json) {
        let errors = auto_link_graphql_error_messages(&relation_json);
        let status = auto_link_set_runtime_error_state(
            repo_name,
            issue_number,
            marker,
            label_automation_failed,
            "GitHub GraphQL query returned errors while reading current parent relation.",
            &format!(
                "API errors: {errors}\n\nRetry later. If this persists, unlink parent manually in GitHub UI."
            ),
        );
        return if status == 0 { 0 } else { status };
    }

    let snapshot = AutoLinkRelationSnapshot::from_payload(&relation_json);
    let current_parent_number = snapshot.current_parent_number();
    let current_parent_node_id = snapshot.current_parent_node_id();
    let child_node_id = snapshot.child_node_id();

    if !current_parent_number.is_empty() {
        if current_parent_node_id.is_empty() || child_node_id.is_empty() {
            let status = auto_link_set_runtime_error_state(
                repo_name,
                issue_number,
                marker,
                label_automation_failed,
                &format!(
                    "Missing node IDs required to unlink current parent #{current_parent_number}."
                ),
                "Retry later. If this persists, unlink parent manually in GitHub UI.",
            );
            return if status == 0 { 0 } else { status };
        }

        let unlink_result =
            auto_link_remove_sub_issue_relation(current_parent_node_id, child_node_id);
        if unlink_result.trim().is_empty() {
            let status = auto_link_set_runtime_error_state(
                repo_name,
                issue_number,
                marker,
                label_automation_failed,
                &format!(
                    "GitHub API mutation failed while unlinking issue from parent #{current_parent_number}."
                ),
                "Retry later. If this persists, unlink parent manually in GitHub UI.",
            );
            return if status == 0 { 0 } else { status };
        }
        if auto_link_graphql_has_errors(&unlink_result) {
            let errors = auto_link_graphql_error_messages(&unlink_result);
            let status = auto_link_set_runtime_error_state(
                repo_name,
                issue_number,
                marker,
                label_automation_failed,
                &format!(
                    "GitHub GraphQL mutation returned errors while unlinking parent #{current_parent_number}."
                ),
                &format!(
                    "API errors: {errors}\n\nRetry later. If this persists, unlink parent manually in GitHub UI."
                ),
            );
            return if status == 0 { 0 } else { status };
        }

        let status = auto_link_set_success_state(
            repo_name,
            issue_number,
            marker,
            label_required_missing,
            label_automation_failed,
            &format!(
                "Removed existing parent link #{current_parent_number} (`Parent: {parent_mode}`)."
            ),
        );
        return if status == 0 { 0 } else { status };
    }

    let status = auto_link_set_success_state(
        repo_name,
        issue_number,
        marker,
        label_required_missing,
        label_automation_failed,
        &format!("No parent linking requested (`Parent: {parent_mode}`)."),
    );
    if status == 0 { 0 } else { status }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn run_auto_link_parent_link(
    repo_name: &str,
    repo_owner: &str,
    repo_short_name: &str,
    issue_number: &str,
    parent_number: &str,
    marker: &str,
    label_required_missing: &str,
    label_automation_failed: &str,
) -> i32 {
    if parent_number == issue_number {
        let status = auto_link_set_validation_error_state(
            repo_name,
            issue_number,
            marker,
            label_required_missing,
            label_automation_failed,
            &format!("Issue cannot reference itself as parent (`Parent: #{issue_number}`)."),
            "Use another parent issue number or `Parent: none`.",
        );
        return if status == 0 { 0 } else { status };
    }

    let parent_snapshot = issue_remote_snapshot_or_default(repo_name, parent_number);
    let parent_title = parent_snapshot.title;
    let parent_state = parent_snapshot.state;
    if parent_state.is_empty() && parent_title.is_empty() {
        let status = auto_link_set_validation_error_state(
            repo_name,
            issue_number,
            marker,
            label_required_missing,
            label_automation_failed,
            &format!("Parent issue `#{parent_number}` was not found."),
            "Use an existing issue number in `Parent:`.",
        );
        return if status == 0 { 0 } else { status };
    }
    if parent_state != "OPEN" {
        let status = auto_link_set_validation_error_state(
            repo_name,
            issue_number,
            marker,
            label_required_missing,
            label_automation_failed,
            &format!("Parent issue `#{parent_number}` is not open (state: {parent_state})."),
            "Reopen the parent or choose another open parent issue.",
        );
        return if status == 0 { 0 } else { status };
    }

    let relation_json = auto_link_query_parent_child_relation(
        repo_owner,
        repo_short_name,
        issue_number,
        parent_number,
    );
    if relation_json.trim().is_empty() {
        let status = auto_link_set_runtime_error_state(
            repo_name,
            issue_number,
            marker,
            label_automation_failed,
            "Unable to query parent/child relation state from GitHub API.",
            "Retry later. If this persists, link the issue manually in GitHub UI.",
        );
        return if status == 0 { 0 } else { status };
    }
    if auto_link_graphql_has_errors(&relation_json) {
        let errors = auto_link_graphql_error_messages(&relation_json);
        let status = auto_link_set_runtime_error_state(
            repo_name,
            issue_number,
            marker,
            label_automation_failed,
            "GitHub GraphQL query returned errors while reading relation state.",
            &format!(
                "API errors: {errors}\n\nRetry later. If this persists, link the issue manually in GitHub UI."
            ),
        );
        return if status == 0 { 0 } else { status };
    }

    let snapshot = AutoLinkRelationSnapshot::from_payload(&relation_json);
    let current_parent_number = snapshot.current_parent_number();
    let current_parent_node_id = snapshot.current_parent_node_id();
    let child_node_id = snapshot.child_node_id();
    let parent_node_id = snapshot.parent_node_id();

    if current_parent_number == parent_number {
        let status = auto_link_set_success_state(
            repo_name,
            issue_number,
            marker,
            label_required_missing,
            label_automation_failed,
            &format!("Issue already linked to parent #{parent_number}."),
        );
        return if status == 0 { 0 } else { status };
    }

    if !current_parent_number.is_empty() && current_parent_number != parent_number {
        if current_parent_node_id.is_empty() || child_node_id.is_empty() {
            let status = auto_link_set_runtime_error_state(
                repo_name,
                issue_number,
                marker,
                label_automation_failed,
                &format!(
                    "Missing node IDs required to re-parent issue from #{current_parent_number} to #{parent_number}."
                ),
                "Retry later. If this persists, update parent linkage manually in GitHub UI.",
            );
            return if status == 0 { 0 } else { status };
        }
        let unlink_result =
            auto_link_remove_sub_issue_relation(current_parent_node_id, child_node_id);
        if unlink_result.trim().is_empty() {
            let status = auto_link_set_runtime_error_state(
                repo_name,
                issue_number,
                marker,
                label_automation_failed,
                &format!(
                    "GitHub API mutation failed while unlinking child from previous parent #{current_parent_number}."
                ),
                "Retry later. If this persists, unlink manually in GitHub UI and rerun automation.",
            );
            return if status == 0 { 0 } else { status };
        }
        if auto_link_graphql_has_errors(&unlink_result) {
            let errors = auto_link_graphql_error_messages(&unlink_result);
            let status = auto_link_set_runtime_error_state(
                repo_name,
                issue_number,
                marker,
                label_automation_failed,
                &format!(
                    "GitHub GraphQL mutation returned errors while unlinking previous parent #{current_parent_number}."
                ),
                &format!(
                    "API errors: {errors}\n\nRetry later. If this persists, unlink manually in GitHub UI and rerun automation."
                ),
            );
            return if status == 0 { 0 } else { status };
        }
    }

    if child_node_id.is_empty() || parent_node_id.is_empty() {
        let status = auto_link_set_runtime_error_state(
            repo_name,
            issue_number,
            marker,
            label_automation_failed,
            "Missing GitHub node IDs required for sub-issue linking.",
            "Retry later. If this persists, link parent/child manually in GitHub UI.",
        );
        return if status == 0 { 0 } else { status };
    }

    let link_result = auto_link_add_sub_issue_relation(parent_node_id, child_node_id);
    if link_result.trim().is_empty() {
        let status = auto_link_set_runtime_error_state(
            repo_name,
            issue_number,
            marker,
            label_automation_failed,
            "GitHub API mutation failed while linking child to parent.",
            &format!(
                "Link manually in GitHub UI, then keep `Parent: #{parent_number}` in issue body for traceability."
            ),
        );
        return if status == 0 { 0 } else { status };
    }
    if auto_link_graphql_has_errors(&link_result) {
        let errors = auto_link_graphql_error_messages(&link_result);
        let status = auto_link_set_runtime_error_state(
            repo_name,
            issue_number,
            marker,
            label_automation_failed,
            "GitHub GraphQL mutation returned errors while linking child to parent.",
            &format!(
                "API errors: {errors}\n\nLink manually in GitHub UI, then keep `Parent: #{parent_number}` in issue body for traceability."
            ),
        );
        return if status == 0 { 0 } else { status };
    }

    let linked_child_number = auto_link_add_sub_issue_linked_number(&link_result);
    if linked_child_number.is_empty() {
        let status = auto_link_set_runtime_error_state(
            repo_name,
            issue_number,
            marker,
            label_automation_failed,
            "GitHub mutation returned no linked sub-issue confirmation.",
            &format!(
                "Retry later. If this persists, link manually in GitHub UI and keep `Parent: #{parent_number}` in issue body."
            ),
        );
        return if status == 0 { 0 } else { status };
    }

    if !current_parent_number.is_empty() && current_parent_number != parent_number {
        let status = auto_link_set_success_state(
            repo_name,
            issue_number,
            marker,
            label_required_missing,
            label_automation_failed,
            &format!("Re-parented this issue from #{current_parent_number} to #{parent_number}."),
        );
        if status != 0 {
            return status;
        }
        println!(
            "Re-parented issue #{} from #{} to #{}.",
            issue_number, current_parent_number, parent_number
        );
        return 0;
    }

    let status = auto_link_set_success_state(
        repo_name,
        issue_number,
        marker,
        label_required_missing,
        label_automation_failed,
        &format!("Linked this issue as child of #{parent_number}."),
    );
    if status != 0 {
        return status;
    }
    println!(
        "Linked issue #{} to parent #{}.",
        issue_number, parent_number
    );
    0
}

pub(crate) fn auto_link_set_validation_error_state(
    repo_name: &str,
    issue_number: &str,
    marker: &str,
    required_missing_label: &str,
    automation_failed_label: &str,
    message: &str,
    help_text: &str,
) -> i32 {
    auto_link_add_label(repo_name, issue_number, required_missing_label);
    auto_link_remove_label(repo_name, issue_number, automation_failed_label);
    let body =
        format!("{marker}\n### Parent Field Autolink Status\n\n❌ {message}\n\n{help_text}\n");
    UpsertMarkerCommentOptions::run_upsert_marker_comment(
        issues::commands::UpsertMarkerCommentOptions {
            repo: repo_name.to_string(),
            issue: issue_number.to_string(),
            marker: marker.to_string(),
            body,
            announce: false,
        },
    )
}

pub(crate) fn auto_link_set_runtime_error_state(
    repo_name: &str,
    issue_number: &str,
    marker: &str,
    automation_failed_label: &str,
    message: &str,
    help_text: &str,
) -> i32 {
    auto_link_add_label(repo_name, issue_number, automation_failed_label);
    let body =
        format!("{marker}\n### Parent Field Autolink Status\n\n⚠️ {message}\n\n{help_text}\n");
    UpsertMarkerCommentOptions::run_upsert_marker_comment(UpsertMarkerCommentOptions {
        repo: repo_name.to_string(),
        issue: issue_number.to_string(),
        marker: marker.to_string(),
        body,
        announce: false,
    })
}

pub(crate) fn auto_link_set_success_state(
    repo_name: &str,
    issue_number: &str,
    marker: &str,
    required_missing_label: &str,
    automation_failed_label: &str,
    message: &str,
) -> i32 {
    auto_link_remove_label(repo_name, issue_number, required_missing_label);
    auto_link_remove_label(repo_name, issue_number, automation_failed_label);
    let body = format!("{marker}\n### Parent Field Autolink Status\n\n✅ {message}\n");
    UpsertMarkerCommentOptions::run_upsert_marker_comment(UpsertMarkerCommentOptions {
        repo: repo_name.to_string(),
        issue: issue_number.to_string(),
        marker: marker.to_string(),
        body,
        announce: false,
    })
}

fn auto_link_add_label(repo_name: &str, issue_number: &str, label: &str) {
    let _ = status_code_owned({
        let mut cmd = gh_issue_target_command("edit", issue_number, Some(repo_name));
        push_arg(&mut cmd, "--add-label");
        push_arg(&mut cmd, label);
        cmd
    });
}

fn auto_link_remove_label(repo_name: &str, issue_number: &str, label: &str) {
    let _ = status_code_owned({
        let mut cmd = gh_issue_target_command("edit", issue_number, Some(repo_name));
        push_arg(&mut cmd, "--remove-label");
        push_arg(&mut cmd, label);
        cmd
    });
}

pub(crate) fn auto_link_extract_parent(body: &str) -> Option<String> {
    let re = Regex::new(r"(?im)^\s*Parent\s*:\s*(.+)$").expect("static regex must compile");
    re.captures(body)
        .and_then(|captures| captures.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn auto_link_query_child_parent_relation(
    repo_owner: &str,
    repo_short_name: &str,
    issue_number: &str,
) -> String {
    output_trim_or_empty(&[
        "api",
        "graphql",
        "-f",
        "query=query($owner:String!,$name:String!,$child:Int!){repository(owner:$owner,name:$name){child:issue(number:$child){id parent{number id}}}}",
        "-f",
        &format!("owner={repo_owner}"),
        "-f",
        &format!("name={repo_short_name}"),
        "-F",
        &format!("child={issue_number}"),
    ])
}

fn auto_link_query_parent_child_relation(
    repo_owner: &str,
    repo_short_name: &str,
    child_issue_number: &str,
    parent_issue_number: &str,
) -> String {
    output_trim_or_empty(&[
        "api",
        "graphql",
        "-f",
        "query=query($owner:String!,$name:String!,$child:Int!,$parent:Int!){repository(owner:$owner,name:$name){child:issue(number:$child){id parent{number id}} parent:issue(number:$parent){id state}}}",
        "-f",
        &format!("owner={repo_owner}"),
        "-f",
        &format!("name={repo_short_name}"),
        "-F",
        &format!("child={child_issue_number}"),
        "-F",
        &format!("parent={parent_issue_number}"),
    ])
}

fn auto_link_remove_sub_issue_relation(parent_node_id: &str, child_node_id: &str) -> String {
    output_trim_or_empty(&[
        "api",
        "graphql",
        "-f",
        "query=mutation($issueId:ID!,$subIssueId:ID!){removeSubIssue(input:{issueId:$issueId,subIssueId:$subIssueId}){issue{id}}}",
        "-f",
        &format!("issueId={parent_node_id}"),
        "-f",
        &format!("subIssueId={child_node_id}"),
    ])
}

fn auto_link_add_sub_issue_relation(parent_node_id: &str, child_node_id: &str) -> String {
    output_trim_or_empty(&[
        "api",
        "graphql",
        "-f",
        "query=mutation($issueId:ID!,$subIssueId:ID!){addSubIssue(input:{issueId:$issueId,subIssueId:$subIssueId}){issue{subIssues(first:1){nodes{number}}}}}",
        "-f",
        &format!("issueId={parent_node_id}"),
        "-f",
        &format!("subIssueId={child_node_id}"),
    ])
}

fn auto_link_graphql_has_errors(payload: &str) -> bool {
    #[derive(Debug, Deserialize)]
    struct GraphqlErrorsPayload {
        errors: Option<Vec<GraphqlError>>,
    }
    #[derive(Debug, Deserialize)]
    struct GraphqlError {}
    if payload.trim().is_empty() {
        return false;
    }
    common_json::from_json_str::<GraphqlErrorsPayload>(payload)
        .ok()
        .and_then(|json| json.errors)
        .map(|errors| !errors.is_empty())
        .unwrap_or(false)
}

fn auto_link_graphql_error_messages(payload: &str) -> String {
    #[derive(Debug, Deserialize)]
    struct GraphqlErrorsPayload {
        errors: Option<Vec<GraphqlError>>,
    }
    #[derive(Debug, Deserialize)]
    struct GraphqlError {
        message: Option<String>,
    }
    let Ok(json) = common_json::from_json_str::<GraphqlErrorsPayload>(payload) else {
        return String::new();
    };
    let Some(errors) = json.errors else {
        return String::new();
    };
    errors
        .iter()
        .filter_map(|entry| entry.message.as_deref())
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>()
        .join("; ")
}

fn auto_link_add_sub_issue_linked_number(payload: &str) -> String {
    #[derive(Debug, Deserialize)]
    struct GraphqlAddSubIssuePayload {
        data: Option<GraphqlAddSubIssueData>,
    }
    #[derive(Debug, Deserialize)]
    struct GraphqlAddSubIssueData {
        #[serde(rename = "addSubIssue")]
        add_sub_issue: Option<GraphqlAddSubIssueResult>,
    }
    #[derive(Debug, Deserialize)]
    struct GraphqlAddSubIssueResult {
        issue: Option<GraphqlAddSubIssueIssue>,
    }
    #[derive(Debug, Deserialize)]
    struct GraphqlAddSubIssueIssue {
        #[serde(rename = "subIssues")]
        sub_issues: Option<GraphqlAddSubIssueNodes>,
    }
    #[derive(Debug, Deserialize)]
    struct GraphqlAddSubIssueNodes {
        nodes: Option<Vec<GraphqlAddSubIssueNode>>,
    }
    #[derive(Debug, Deserialize)]
    struct GraphqlAddSubIssueNode {
        number: Option<u64>,
    }

    let Ok(json) = common_json::from_json_str::<GraphqlAddSubIssuePayload>(payload) else {
        return String::new();
    };
    json.data
        .and_then(|data| data.add_sub_issue)
        .and_then(|result| result.issue)
        .and_then(|issue| issue.sub_issues)
        .and_then(|sub| sub.nodes)
        .and_then(|nodes| nodes.first().and_then(|node| node.number))
        .map(|value| value.to_string())
        .unwrap_or_default()
}

pub(crate) fn split_repo_name(repo_name: &str) -> (String, String) {
    let mut parts = repo_name.splitn(2, '/');
    let owner = parts.next().unwrap_or("").to_string();
    let name = parts.next().unwrap_or("").to_string();
    (owner, name)
}

pub(crate) fn is_issue_key(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.starts_with('#') && trimmed[1..].chars().all(|ch| ch.is_ascii_digit())
}

pub(crate) fn issue_remote_snapshot_or_default(
    repo_name: &str,
    issue_number: &str,
) -> IssueRemoteSnapshot {
    IssueRemoteSnapshot::load_issue_remote_snapshot(issue_number, Some(repo_name))
        .unwrap_or_default()
}

pub(crate) fn gh_issue_state_or_empty(repo_name: Option<&str>, issue_number: &str) -> String {
    IssueRemoteSnapshot::load_issue_remote_snapshot(issue_number, repo_name)
        .ok()
        .and_then(|snapshot| normalize_issue_state(&snapshot.state).map(str::to_string))
        .unwrap_or_default()
}

fn normalize_issue_state(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    match trimmed {
        "OPEN" => Some("OPEN"),
        "CLOSED" => Some("CLOSED"),
        _ => None,
    }
}

pub(crate) fn gh_pr_body_or_empty(repo_name: &str, pr_number: &str) -> String {
    load_pr_remote_snapshot(pr_number, repo_name)
        .map(|snapshot| snapshot.body)
        .unwrap_or_default()
}

pub(crate) fn neutralize_reason_for_issue_cached(
    issue_number: &str,
    repo_name: &str,
    cache: &mut HashMap<String, String>,
) -> String {
    let cache_key = format!("#{issue_number}");
    if let Some(value) = cache.get(&cache_key) {
        return value.clone();
    }
    let reason =
        Validation::fetch_non_compliance_reason(issue_number, Some(repo_name)).unwrap_or_default();
    cache.insert(cache_key, reason.clone());
    reason
}

pub(crate) fn apply_rejected_marker(
    text: &str,
    keyword_pattern: &str,
    issue_key: &str,
) -> Result<String, String> {
    let issue_pattern = regex::escape(issue_key);
    let pattern = format!(
        "(?i)\\b(?P<kw>(?:{}))\\b(?P<ws>\\s+)(?P<rej>rejected\\s+)?(?P<ref>[^\\s]*{})\\b",
        keyword_pattern, issue_pattern
    );
    let re = Regex::new(&pattern).map_err(|err| format!("invalid keyword pattern: {err}"))?;
    Ok(re
        .replace_all(text, |caps: &regex::Captures<'_>| {
            let kw = caps.name("kw").map_or("", |m| m.as_str());
            let ws = caps.name("ws").map_or(" ", |m| m.as_str());
            let rej = caps.name("rej").map_or("", |m| m.as_str());
            let ref_part = caps.name("ref").map_or("", |m| m.as_str());
            if rej.is_empty() {
                format!("{kw}{ws}rejected {ref_part}")
            } else {
                format!("{kw}{ws}{rej}{ref_part}")
            }
        })
        .to_string())
}

pub(crate) fn remove_rejected_marker(
    text: &str,
    keyword_pattern: &str,
    issue_key: &str,
) -> Result<String, String> {
    let issue_pattern = regex::escape(issue_key);
    let pattern = format!(
        "(?i)\\b(?P<kw>(?:{}))\\b(?P<ws>\\s+)rejected\\s+(?P<ref>[^\\s]*{})\\b",
        keyword_pattern, issue_pattern
    );
    let re = Regex::new(&pattern).map_err(|err| format!("invalid keyword pattern: {err}"))?;
    Ok(re.replace_all(text, "${kw}${ws}${ref}").to_string())
}

pub(crate) fn build_neutralize_comment_body(
    marker: &str,
    neutralized_count: usize,
    neutralized_reason: &HashMap<String, String>,
    neutralized_action: &HashMap<String, String>,
) -> String {
    if neutralized_count == 0 {
        return format!(
            "{marker}\n### Closure Neutralization Status\n\n✅ No non-compliant closure refs detected. No neutralization applied."
        );
    }

    let mut issue_keys: Vec<&String> = neutralized_reason.keys().collect();
    issue_keys.sort_by_key(|key| {
        key.trim_start_matches('#')
            .parse::<u64>()
            .unwrap_or(u64::MAX)
    });

    let mut body = format!(
        "{marker}\n### Closure Neutralization Status\n\n⚠️ Non-compliant issue references were neutralized to prevent incorrect auto-close.\n\n"
    );
    for issue_key in issue_keys {
        let action = neutralized_action
            .get(issue_key)
            .cloned()
            .unwrap_or_default();
        let reason = neutralized_reason
            .get(issue_key)
            .cloned()
            .unwrap_or_default();
        body.push_str(&format!("- {action} rejected {issue_key}: {reason}\n"));
    }
    body.push_str("\nHow to restore standard auto-close:\n");
    body.push_str("- Fix issue required fields/title contract (if applicable).\n");
    body.push_str("- Remove or adjust `Reopen #...` for issues that should close now.\n");
    body.push_str("- Remove `rejected` from closure lines in PR body.");
    body
}

pub(crate) fn upsert_pr_marker_comment(
    repo_name: &str,
    pr_number: &str,
    marker: &str,
    body: &str,
) -> i32 {
    match upsert_issue_comment_by_marker(repo_name, pr_number, marker, body) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{err}");
            1
        }
    }
}

pub(crate) fn run_repo_name() -> i32 {
    print_string_result(resolve_repo_name(None), 3)
}

pub(crate) fn run_current_login() -> i32 {
    let login = output_trim_or_empty(&["api", "user", "--jq", ".login"]);
    print_non_empty_lines(&login);
    0
}

pub(crate) fn extract_issue_refs_for_footer(text: &str) -> Vec<(String, String)> {
    let filtered = text
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n");
    let re =
        Regex::new(r"(?i)(cancel[\s_-]*closes|closes|fixes|part\s+of|reopen|reopens)\s+#([0-9]+)")
            .expect("static regex must compile");
    let mut seen = HashSet::<String>::new();
    let mut refs: Vec<(String, String)> = Vec::new();
    for caps in re.captures_iter(&filtered) {
        let Some(action_raw) = caps.get(1).map(|m| m.as_str()) else {
            continue;
        };
        let Some(issue_number_raw) = caps.get(2).map(|m| m.as_str()) else {
            continue;
        };
        let action = action_raw.to_ascii_lowercase();
        let issue_number = issue_number_raw.to_string();
        let key = format!("{action}|{issue_number}");
        if seen.insert(key) {
            refs.push((action, issue_number));
        }
    }
    refs
}

pub(crate) fn is_root_parent_issue_for_repo(
    issue_number: &str,
    repo_name: &str,
) -> Result<bool, String> {
    let body = issue_remote_snapshot_or_default(repo_name, issue_number).body;
    let parent_value = extract_parent_field(&body)
        .unwrap_or_else(|| "none".to_string())
        .to_lowercase();

    if parent_value == "epic" {
        return Ok(true);
    }
    if parent_value == "base" || parent_value.starts_with('#') {
        return Ok(false);
    }

    let (owner, repo_short) = split_repo_name(repo_name);
    if owner.is_empty() || repo_short.is_empty() {
        return Err(format!(
            "❌ Invalid repository format '{repo_name}' (expected owner/name)."
        ));
    }
    let has_children =
        !extract_subissue_refs_for_parent(&owner, &repo_short, issue_number).is_empty();
    Ok(has_children)
}

pub(crate) fn evaluate_parent_issue(
    strict_guard: bool,
    repo_name: &str,
    repo_owner: &str,
    repo_short_name: &str,
    parent_number: &str,
) -> i32 {
    let parent_snapshot = issue_remote_snapshot_or_default(repo_name, parent_number);
    let body = parent_snapshot.body;
    let parent_state = parent_snapshot.state;
    if parent_state.is_empty() && body.is_empty() {
        return 0;
    }

    let mut child_refs =
        extract_subissue_refs_for_parent(repo_owner, repo_short_name, parent_number);
    if child_refs.is_empty() {
        child_refs = extract_tasklist_refs(&body);
    }
    if child_refs.is_empty() {
        return 0;
    }

    let total = child_refs.len();
    let mut closed_count = 0usize;
    let mut open_count = 0usize;
    let mut open_lines = String::new();

    for child_ref in child_refs {
        let child_number = child_ref.trim_start_matches('#');
        let child_snapshot = issue_remote_snapshot_or_default(repo_name, child_number);
        let child_title = child_snapshot.title;
        let child_state = child_snapshot.state;
        if child_state.is_empty() && child_title.is_empty() {
            open_count += 1;
            open_lines.push_str(&format!("- {} (unreadable or missing)\n", child_ref));
            continue;
        }
        if child_state == "CLOSED" {
            closed_count += 1;
        } else {
            open_count += 1;
            open_lines.push_str(&format!("- {} {}\n", child_ref, child_title));
        }
    }

    let marker = format!("<!-- parent-issue-status:{parent_number} -->");
    let comment_body = build_parent_guard_status_comment(
        strict_guard,
        parent_number,
        &parent_state,
        total,
        closed_count,
        open_count,
        &open_lines,
    );
    let status =
        UpsertMarkerCommentOptions::run_upsert_marker_comment(UpsertMarkerCommentOptions {
            repo: repo_name.to_string(),
            issue: parent_number.to_string(),
            marker,
            body: comment_body,
            announce: true,
        });
    if status != 0 {
        return status;
    }

    if open_count == 0 && parent_state == "OPEN" {
        let close_status = CloseOptions {
            issue: parent_number.to_string(),
            repo: Some(repo_name.to_string()),
            reason: "completed".to_string(),
            comment: None,
        }
        .run_close();

        if close_status != 0 {
            return close_status;
        }
        println!(
            "Closed parent issue #{} because all required children are closed.",
            parent_number
        );
    }

    if strict_guard && parent_state == "CLOSED" && open_count > 0 {
        let reopen_status = IssueTarget {
            issue: parent_number.to_string(),
            repo: Some(repo_name.to_string()),
        }
        .run_reopen();

        if reopen_status != 0 {
            return reopen_status;
        }
        println!(
            "Reopened parent issue #{} due to open required children.",
            parent_number
        );
    }
    0
}

pub(crate) fn extract_subissue_refs_for_parent(
    repo_owner: &str,
    repo_short_name: &str,
    parent_number: &str,
) -> Vec<String> {
    let output = output_trim_or_empty(&[
        "api",
        "graphql",
        "-f",
        "query=query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){issue(number:$number){subIssues(first:100){nodes{number}}}}}",
        "-f",
        &format!("owner={repo_owner}"),
        "-f",
        &format!("name={repo_short_name}"),
        "-F",
        &format!("number={parent_number}"),
        "--jq",
        ".data.repository.issue.subIssues.nodes[]?.number | \"#\"+tostring",
    ]);
    output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect()
}

pub(crate) fn collect_parent_candidates(
    repo_name: &str,
    repo_owner: &str,
    repo_short_name: &str,
    child_number: &str,
) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut seen = HashSet::new();

    let direct = output_trim_or_empty(&[
        "api",
        "graphql",
        "-f",
        "query=query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){issue(number:$number){parent{number}}}}",
        "-f",
        &format!("owner={repo_owner}"),
        "-f",
        &format!("name={repo_short_name}"),
        "-F",
        &format!("number={child_number}"),
        "--jq",
        ".data.repository.issue.parent.number // empty | \"#\"+tostring",
    ]);
    for line in direct
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        let candidate = line.trim_start_matches('#').to_string();
        if !candidate.is_empty() && seen.insert(candidate.clone()) {
            out.push(candidate);
        }
    }

    if out.is_empty() {
        let search = output_trim_or_empty(&[
            "api",
            "search/issues",
            "-f",
            &format!("q=repo:{repo_name} is:issue \"#{child_number}\""),
            "--jq",
            ".items[].number",
        ]);
        for line in search
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
        {
            let candidate = line.to_string();
            if seen.insert(candidate.clone()) {
                out.push(candidate);
            }
        }
    }

    out
}

fn build_parent_guard_status_comment(
    strict_guard: bool,
    parent_number: &str,
    parent_state: &str,
    total: usize,
    closed_count: usize,
    open_count: usize,
    open_lines: &str,
) -> String {
    let marker = format!("<!-- parent-issue-status:{parent_number} -->");
    let mut comment = format!(
        "{marker}\n### Parent Issue Status\nParent: #{parent_number}\n\n- Required children detected: {total}\n- Closed: {closed_count}\n- Open: {open_count}\n\n"
    );
    if open_count == 0 {
        comment.push_str("All required child issues are closed. This parent can be closed.");
        return comment;
    }
    comment.push_str("Some required child issues are still open:\n");
    comment.push_str(open_lines);
    if parent_state == "CLOSED" && strict_guard {
        comment.push_str(
            "\nGuard action: parent was reopened because required children are still open.",
        );
    }
    comment
}

pub(crate) fn print_string_result(result: Result<String, String>, error_code: i32) -> i32 {
    match result {
        Ok(value) => {
            println!("{value}");
            0
        }
        Err(message) => {
            eprintln!("{message}");
            error_code
        }
    }
}

pub(crate) fn print_non_empty_lines(text: &str) {
    for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
        println!("{line}");
    }
}
