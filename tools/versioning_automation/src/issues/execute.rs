//! tools/versioning_automation/src/issues/execute.rs
use std::collections::{HashMap, HashSet};

use regex::Regex;
use serde::Deserialize;

use crate::issues::commands::{
    AssigneeLoginsOptions, AutoLinkOptions, CloseOptions, ClosureHygieneOptions, CreateOptions,
    DoneStatusMode, DoneStatusOptions, ExtractRefsOptions, ExtractRefsProfile,
    FetchNonComplianceReasonOptions, HasLabelOptions, IsRootParentOptions, IssueFieldName,
    IssueFieldOptions, IssueTarget, LabelExistsOptions, ListByLabelOptions, NeutralizeOptions,
    NonComplianceReasonOptions, OpenNumbersOptions, OpenSnapshotsOptions, ParentGuardOptions,
    ReadOptions, ReevaluateOptions, ReopenOnDevOptions, RequiredFieldsValidateOptions,
    RequiredFieldsValidationMode, StateOptions, SubissueRefsOptions, TasklistRefsOptions,
    UpdateOptions, UpsertMarkerCommentOptions,
};
use crate::issues::issue_comments::{find_latest_matching_comment_id, parse_issue_comments};
use crate::issues::render::render_direct_issue_body;
use crate::issues::required_fields::{
    fetch_non_compliance_reason, non_compliance_reason_from_content, validate_body,
    validate_content, validate_title,
};
use crate::issues::sync_project_status::run_sync_project_status;
use crate::issues::tasklist_refs::extract_tasklist_refs;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_create(opts: CreateOptions) -> i32 {
    let body = render_direct_issue_body(&opts);
    if opts.dry_run {
        println!("Dry-run mode. Issue was not created.");
        println!("----- title -----");
        println!("{}", opts.title);
        println!("----- body -----");
        println!("{}", body);
        return 0;
    }

    let mut cmd = gh_command(&["issue", "create"]);
    push_arg(&mut cmd, "--title");
    push_arg(&mut cmd, &opts.title);
    push_arg(&mut cmd, "--body");
    push_arg(&mut cmd, &body);
    add_repo_arg(&mut cmd, opts.repo.as_deref());
    for label in &opts.labels {
        push_arg(&mut cmd, "--label");
        push_arg(&mut cmd, label);
    }
    for assignee in &opts.assignees {
        push_arg(&mut cmd, "--assignee");
        push_arg(&mut cmd, assignee);
    }
    execute_command(cmd)
}

pub(crate) fn run_read(opts: ReadOptions) -> i32 {
    let mut cmd = gh_command(&["issue"]);
    if let Some(issue) = &opts.issue {
        push_arg(&mut cmd, "view");
        push_arg(&mut cmd, issue);
    } else {
        push_arg(&mut cmd, "list");
    }
    add_repo_arg(&mut cmd, opts.repo.as_deref());
    if let Some(json) = &opts.json {
        push_arg(&mut cmd, "--json");
        push_arg(&mut cmd, json);
    }
    if let Some(jq) = &opts.jq {
        push_arg(&mut cmd, "--jq");
        push_arg(&mut cmd, jq);
    }
    if let Some(template) = &opts.template {
        push_arg(&mut cmd, "--template");
        push_arg(&mut cmd, template);
    }
    execute_command(cmd)
}

pub(crate) fn run_done_status(opts: DoneStatusOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo.clone()) {
        Ok(repo) => repo,
        Err(message) => {
            eprintln!("{message}");
            return 3;
        }
    };
    let label_name = opts.label;

    let label_exists = gh_output_or_empty(&[
        "label", "list", "-R", &repo_name, "--limit", "1000", "--json", "name", "--jq", ".[].name",
    ])
    .lines()
    .any(|value| value.trim() == label_name);

    match opts.mode {
        DoneStatusMode::OnDevMerge => {
            let Some(pr_number) = opts.pr else {
                eprintln!("done-status --on-dev-merge requires: --pr");
                return 2;
            };

            let pr_state = gh_output_or_empty(&[
                "pr",
                "view",
                &pr_number,
                "-R",
                &repo_name,
                "--json",
                "state",
                "--jq",
                ".state // \"\"",
            ]);
            if pr_state != "MERGED" {
                println!("PR #{} is not merged; nothing to do.", pr_number);
                return 0;
            }

            let pr_title = gh_output_or_empty(&[
                "pr",
                "view",
                &pr_number,
                "-R",
                &repo_name,
                "--json",
                "title",
                "--jq",
                ".title // \"\"",
            ]);
            let pr_body = gh_output_or_empty(&[
                "pr",
                "view",
                &pr_number,
                "-R",
                &repo_name,
                "--json",
                "body",
                "--jq",
                ".body // \"\"",
            ]);
            let pr_commits = gh_output_or_empty(&[
                "api",
                &format!("repos/{repo_name}/pulls/{pr_number}/commits"),
                "--paginate",
                "--jq",
                ".[].commit.message",
            ]);
            let payload = format!("{pr_title}\n{pr_body}\n{pr_commits}");
            let closing_issue_numbers = extract_closing_issue_numbers(&payload);
            if closing_issue_numbers.is_empty() {
                println!("No closing issue refs found for PR #{}.", pr_number);
                return 0;
            }

            if !label_exists {
                println!(
                    "Warning: label '{}' does not exist in {}; skipping done-in-dev labeling.",
                    label_name, repo_name
                );
                return 0;
            }

            for issue_number in closing_issue_numbers {
                let state = gh_issue_state_or_empty(Some(repo_name.as_str()), &issue_number);
                if state.is_empty() {
                    println!("Issue #{}: unreadable; skipping.", issue_number);
                    continue;
                }
                if state != "OPEN" {
                    println!(
                        "Issue #{}: state={}; skipping done-in-dev label.",
                        issue_number, state
                    );
                    continue;
                }

                let has_label = gh_output_or_empty(&[
                    "issue",
                    "view",
                    &issue_number,
                    "-R",
                    &repo_name,
                    "--json",
                    "labels",
                    "--jq",
                    ".labels[].name",
                ])
                .lines()
                .any(|value| value.trim() == label_name);
                if has_label {
                    println!(
                        "Issue #{}: label '{}' already present.",
                        issue_number, label_name
                    );
                    continue;
                }

                let status = execute_command({
                    let mut cmd =
                        gh_issue_target_command("edit", &issue_number, Some(repo_name.as_str()));
                    push_arg(&mut cmd, "--add-label");
                    push_arg(&mut cmd, &label_name);
                    cmd
                });
                if status != 0 {
                    return status;
                }
                println!("Issue #{}: added label '{}'.", issue_number, label_name);
            }
            0
        }
        DoneStatusMode::OnIssueClosed => {
            let Some(issue_number) = opts.issue else {
                eprintln!("done-status --on-issue-closed requires: --issue");
                return 2;
            };

            if !label_exists {
                println!(
                    "Warning: label '{}' does not exist in {}; skipping.",
                    label_name, repo_name
                );
                return 0;
            }

            let has_label = gh_output_or_empty(&[
                "issue",
                "view",
                &issue_number,
                "-R",
                &repo_name,
                "--json",
                "labels",
                "--jq",
                ".labels[].name",
            ])
            .lines()
            .any(|value| value.trim() == label_name);

            if has_label {
                let status = execute_command({
                    let mut cmd =
                        gh_issue_target_command("edit", &issue_number, Some(repo_name.as_str()));
                    push_arg(&mut cmd, "--remove-label");
                    push_arg(&mut cmd, &label_name);
                    cmd
                });
                if status != 0 {
                    return status;
                }
                println!("Issue #{}: removed label '{}'.", issue_number, label_name);
            } else {
                println!(
                    "Issue #{}: label '{}' not present.",
                    issue_number, label_name
                );
            }
            0
        }
    }
}

pub(crate) fn run_reopen_on_dev(opts: ReopenOnDevOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo.clone()) {
        Ok(repo) => repo,
        Err(message) => {
            eprintln!("{message}");
            return 3;
        }
    };
    let label_name = opts.label;
    let pr_number = opts.pr;

    let pr_state = gh_output_or_empty(&[
        "pr",
        "view",
        &pr_number,
        "-R",
        &repo_name,
        "--json",
        "state",
        "--jq",
        ".state // \"\"",
    ]);
    if pr_state != "MERGED" {
        println!("PR #{} is not merged; nothing to do.", pr_number);
        return 0;
    }

    let pr_title = gh_output_or_empty(&[
        "pr",
        "view",
        &pr_number,
        "-R",
        &repo_name,
        "--json",
        "title",
        "--jq",
        ".title // \"\"",
    ]);
    let pr_body = gh_output_or_empty(&[
        "pr",
        "view",
        &pr_number,
        "-R",
        &repo_name,
        "--json",
        "body",
        "--jq",
        ".body // \"\"",
    ]);
    let pr_commits = gh_output_or_empty(&[
        "api",
        &format!("repos/{repo_name}/pulls/{pr_number}/commits"),
        "--paginate",
        "--jq",
        ".[].commit.message",
    ]);
    let payload = format!("{pr_title}\n{pr_body}\n{pr_commits}");

    let reopen_issue_numbers = extract_reopen_issue_numbers(&payload);
    if reopen_issue_numbers.is_empty() {
        println!("No reopen issue refs found for PR #{}.", pr_number);
        return 0;
    }

    let label_exists = gh_output_or_empty(&[
        "label", "list", "-R", &repo_name, "--limit", "1000", "--json", "name", "--jq", ".[].name",
    ])
    .lines()
    .any(|value| value.trim() == label_name);

    let reopen_status =
        std::env::var("PROJECT_STATUS_REOPEN_NAME").unwrap_or_else(|_| "Todo".to_string());

    for issue_number in reopen_issue_numbers {
        let state = gh_issue_state_or_empty(Some(repo_name.as_str()), &issue_number);
        if state.is_empty() {
            println!("Issue #{}: unreadable; skipping reopen sync.", issue_number);
            continue;
        }

        if state == "CLOSED" {
            let status = execute_command(gh_issue_target_command(
                "reopen",
                &issue_number,
                Some(repo_name.as_str()),
            ));
            if status != 0 {
                return status;
            }
            println!("Issue #{}: reopened from Reopen ref.", issue_number);
        } else {
            println!(
                "Issue #{}: state={}; no reopen needed.",
                issue_number, state
            );
        }

        if label_exists {
            let has_label = gh_output_or_empty(&[
                "issue",
                "view",
                &issue_number,
                "-R",
                &repo_name,
                "--json",
                "labels",
                "--jq",
                ".labels[].name",
            ])
            .lines()
            .any(|value| value.trim() == label_name);
            if has_label {
                let status = execute_command({
                    let mut cmd =
                        gh_issue_target_command("edit", &issue_number, Some(repo_name.as_str()));
                    push_arg(&mut cmd, "--remove-label");
                    push_arg(&mut cmd, &label_name);
                    cmd
                });
                if status != 0 {
                    return status;
                }
                println!(
                    "Issue #{}: removed label '{}' due to Reopen ref.",
                    issue_number, label_name
                );
            }
        }

        let status = run_sync_project_status(crate::issues::commands::SyncProjectStatusOptions {
            repo: repo_name.clone(),
            issue: issue_number.clone(),
            status: reopen_status.clone(),
        });
        if status != 0 {
            return status;
        }
    }

    0
}

pub(crate) fn run_neutralize(opts: NeutralizeOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };

    let marker = format!("<!-- closure-neutralizer:{} -->", opts.pr);
    let original_body = gh_pr_body_or_empty(&repo_name, &opts.pr);
    if original_body.trim().is_empty() {
        eprintln!("Error: unable to read PR #{}.", opts.pr);
        return 4;
    }

    let (closing_refs, pre_neutralized_refs) = collect_neutralize_refs(&original_body);
    let mut updated_body = original_body.clone();
    let mut seen_refs: HashSet<String> = HashSet::new();
    let mut reason_cache: HashMap<String, String> = HashMap::new();
    let mut neutralized_reason: HashMap<String, String> = HashMap::new();
    let mut neutralized_action: HashMap<String, String> = HashMap::new();
    let mut neutralized_count = 0usize;

    for (action, issue_key) in closing_refs {
        let dedupe_key = format!("{action}|{issue_key}");
        if !seen_refs.insert(dedupe_key) {
            continue;
        }
        let issue_number = issue_key.trim_start_matches('#');
        let reason =
            neutralize_reason_for_issue_cached(issue_number, &repo_name, &mut reason_cache);
        if reason.is_empty() {
            continue;
        }

        match apply_rejected_marker(&updated_body, "closes|fixes", &issue_key) {
            Ok(body) => updated_body = body,
            Err(_) => continue,
        }
        neutralized_reason.insert(issue_key.clone(), reason);
        neutralized_action.insert(issue_key, action);
        neutralized_count += 1;
    }

    for (action, issue_key) in pre_neutralized_refs {
        let dedupe_key = format!("{action}|{issue_key}");
        if !seen_refs.insert(dedupe_key) {
            continue;
        }
        let issue_number = issue_key.trim_start_matches('#');
        let reason =
            neutralize_reason_for_issue_cached(issue_number, &repo_name, &mut reason_cache);
        if reason.is_empty() {
            match remove_rejected_marker(&updated_body, "closes|fixes", &issue_key) {
                Ok(body) => updated_body = body,
                Err(_) => continue,
            }
            continue;
        }

        match apply_rejected_marker(&updated_body, "closes|fixes", &issue_key) {
            Ok(body) => updated_body = body,
            Err(_) => continue,
        }
        neutralized_reason.insert(issue_key.clone(), reason);
        neutralized_action.insert(issue_key, action);
        neutralized_count += 1;
    }

    if updated_body != original_body {
        let status = execute_command({
            let mut cmd = gh_command(&["pr", "edit", &opts.pr]);
            add_repo_arg(&mut cmd, Some(repo_name.as_str()));
            push_arg(&mut cmd, "--body");
            push_arg(&mut cmd, &updated_body);
            cmd
        });
        if status != 0 {
            return status;
        }
    }

    let comment_body = build_neutralize_comment_body(
        &marker,
        neutralized_count,
        &neutralized_reason,
        &neutralized_action,
    );
    let status = upsert_pr_marker_comment(&repo_name, &opts.pr, &marker, &comment_body);
    if status != 0 {
        return status;
    }

    println!("Closure neutralization evaluated for PR #{}.", opts.pr);
    0
}

pub(crate) fn run_auto_link(opts: AutoLinkOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };

    let marker = format!("<!-- parent-field-autolink:{} -->", opts.issue);
    let label_required_missing = "issue-required-missing";
    let label_automation_failed = "automation-failed";
    let (repo_owner, repo_short_name) = split_repo_name(&repo_name);

    let (issue_title, issue_body, issue_state, issue_labels_raw) =
        gh_issue_autolink_payload(&repo_name, &opts.issue);
    if issue_state.is_empty() {
        eprintln!("Erreur: impossible de lire l'issue #{}.", opts.issue);
        return 4;
    }

    let contract_errors =
        validate_content(&issue_title, &issue_body, &issue_labels_raw).unwrap_or_default();
    if !contract_errors.is_empty() {
        let mut summary_lines = String::new();
        for entry in contract_errors {
            summary_lines.push_str("- ");
            summary_lines.push_str(&entry.message);
            summary_lines.push('\n');
        }
        let help = format!(
            "Detected problems:\n\n{summary_lines}\nExpected contract source: `.github/issue_required_fields.conf`."
        );
        let status = auto_link_set_validation_error_state(
            &repo_name,
            &opts.issue,
            &marker,
            label_required_missing,
            label_automation_failed,
            "Issue body/title is non-compliant with required issue format.",
            &help,
        );
        return if status == 0 { 0 } else { status };
    }

    let parent_raw = match auto_link_extract_parent(&issue_body) {
        Some(value) if !value.is_empty() => value,
        _ => {
            let help = "Expected format:\n\n- `Parent: #<issue_number>` for child issues\n\n- `Parent: none` for independent issues\n\n- `Parent: base` for cascade root issues\n\n- `Parent: epic` for epic umbrella issues";
            let status = auto_link_set_validation_error_state(
                &repo_name,
                &opts.issue,
                &marker,
                label_required_missing,
                label_automation_failed,
                "Missing required field `Parent:` in issue body.",
                help,
            );
            return if status == 0 { 0 } else { status };
        }
    };

    let parent_raw_lc = parent_raw.to_lowercase();
    if parent_raw_lc == "none" || parent_raw_lc == "base" || parent_raw_lc == "epic" {
        return run_auto_link_parent_none(
            &repo_name,
            &repo_owner,
            &repo_short_name,
            &opts.issue,
            &parent_raw_lc,
            &marker,
            label_required_missing,
            label_automation_failed,
        );
    }

    if !is_issue_key(&parent_raw) {
        let status = auto_link_set_validation_error_state(
            &repo_name,
            &opts.issue,
            &marker,
            label_required_missing,
            label_automation_failed,
            &format!("Invalid `Parent:` value: `{parent_raw}`."),
            "Expected `Parent: #<issue_number>` or one of `Parent: none|base|epic`.",
        );
        return if status == 0 { 0 } else { status };
    }

    run_auto_link_parent_link(
        &repo_name,
        &repo_owner,
        &repo_short_name,
        &opts.issue,
        parent_raw.trim_start_matches('#'),
        &marker,
        label_required_missing,
        label_automation_failed,
    )
}

#[allow(clippy::too_many_arguments)]
fn run_auto_link_parent_none(
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

    let (current_parent_number, current_parent_node_id, child_node_id, _) =
        auto_link_relation_snapshot(&relation_json);

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
            auto_link_remove_sub_issue_relation(&current_parent_node_id, &child_node_id);
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
fn run_auto_link_parent_link(
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

    let (parent_title, _, parent_state, _) = gh_issue_autolink_payload(repo_name, parent_number);
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

    let (current_parent_number, current_parent_node_id, child_node_id, parent_node_id) =
        auto_link_relation_snapshot(&relation_json);

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
            auto_link_remove_sub_issue_relation(&current_parent_node_id, &child_node_id);
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

    let link_result = auto_link_add_sub_issue_relation(&parent_node_id, &child_node_id);
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

fn auto_link_set_validation_error_state(
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
    run_upsert_marker_comment(crate::issues::commands::UpsertMarkerCommentOptions {
        repo: repo_name.to_string(),
        issue: issue_number.to_string(),
        marker: marker.to_string(),
        body,
        announce: false,
    })
}

fn auto_link_set_runtime_error_state(
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
    run_upsert_marker_comment(crate::issues::commands::UpsertMarkerCommentOptions {
        repo: repo_name.to_string(),
        issue: issue_number.to_string(),
        marker: marker.to_string(),
        body,
        announce: false,
    })
}

fn auto_link_set_success_state(
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
    run_upsert_marker_comment(crate::issues::commands::UpsertMarkerCommentOptions {
        repo: repo_name.to_string(),
        issue: issue_number.to_string(),
        marker: marker.to_string(),
        body,
        announce: false,
    })
}

fn auto_link_add_label(repo_name: &str, issue_number: &str, label: &str) {
    let _ = execute_command({
        let mut cmd = gh_issue_target_command("edit", issue_number, Some(repo_name));
        push_arg(&mut cmd, "--add-label");
        push_arg(&mut cmd, label);
        cmd
    });
}

fn auto_link_remove_label(repo_name: &str, issue_number: &str, label: &str) {
    let _ = execute_command({
        let mut cmd = gh_issue_target_command("edit", issue_number, Some(repo_name));
        push_arg(&mut cmd, "--remove-label");
        push_arg(&mut cmd, label);
        cmd
    });
}

fn auto_link_extract_parent(body: &str) -> Option<String> {
    let re = Regex::new(r"(?im)^\s*Parent\s*:\s*(.+)$").expect("static regex must compile");
    re.captures(body)
        .and_then(|captures| captures.get(1))
        .map(|m| m.as_str().trim().to_string())
}

fn extract_parent_field(body: &str) -> Option<String> {
    let re =
        Regex::new(r"(?i)^\s*Parent:\s*(#?[0-9]+|none|base|epic|\(none\)|\(base\)|\(epic\))\s*$")
            .expect("static regex must compile");
    let mut parent_value: Option<String> = None;

    for line in body.lines() {
        if let Some(captures) = re.captures(line) {
            parent_value = captures.get(1).map(|m| m.as_str().trim().to_lowercase());
        }
    }

    parent_value.map(|raw| {
        raw.trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .to_string()
    })
}

fn auto_link_query_child_parent_relation(
    repo_owner: &str,
    repo_short_name: &str,
    issue_number: &str,
) -> String {
    gh_output_or_empty(&[
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
    gh_output_or_empty(&[
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
    gh_output_or_empty(&[
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
    gh_output_or_empty(&[
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

type AutoLinkRelationSnapshot = (String, String, String, String);

fn auto_link_relation_snapshot(payload: &str) -> AutoLinkRelationSnapshot {
    #[derive(Debug, Deserialize)]
    struct GraphqlRelationPayload {
        data: Option<GraphqlRelationData>,
    }
    #[derive(Debug, Deserialize)]
    struct GraphqlRelationData {
        repository: Option<GraphqlRelationRepository>,
    }
    #[derive(Debug, Deserialize)]
    struct GraphqlRelationRepository {
        child: Option<GraphqlRelationChild>,
        parent: Option<GraphqlRelationParentIssue>,
    }
    #[derive(Debug, Deserialize)]
    struct GraphqlRelationChild {
        id: Option<String>,
        parent: Option<GraphqlRelationParentRef>,
    }
    #[derive(Debug, Deserialize)]
    struct GraphqlRelationParentRef {
        number: Option<u64>,
        id: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    struct GraphqlRelationParentIssue {
        id: Option<String>,
    }

    let Ok(json) = common_json::from_json_str::<GraphqlRelationPayload>(payload) else {
        return (String::new(), String::new(), String::new(), String::new());
    };
    let repository = json.data.and_then(|data| data.repository);
    let child = repository.as_ref().and_then(|repo| repo.child.as_ref());
    let parent_ref = child.and_then(|child| child.parent.as_ref());
    let parent_issue = repository.as_ref().and_then(|repo| repo.parent.as_ref());

    (
        parent_ref
            .and_then(|parent| parent.number)
            .map(|value| value.to_string())
            .unwrap_or_default(),
        parent_ref
            .and_then(|parent| parent.id.clone())
            .unwrap_or_default(),
        child.and_then(|entry| entry.id.clone()).unwrap_or_default(),
        parent_issue
            .and_then(|parent| parent.id.clone())
            .unwrap_or_default(),
    )
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

fn split_repo_name(repo_name: &str) -> (String, String) {
    let mut parts = repo_name.splitn(2, '/');
    let owner = parts.next().unwrap_or("").to_string();
    let name = parts.next().unwrap_or("").to_string();
    (owner, name)
}

fn is_issue_key(value: &str) -> bool {
    let trimmed = value.trim();
    trimmed.starts_with('#') && trimmed[1..].chars().all(|ch| ch.is_ascii_digit())
}

type IssueAutoLinkPayload = (String, String, String, String);

fn gh_issue_autolink_payload(repo_name: &str, issue_number: &str) -> IssueAutoLinkPayload {
    #[derive(Debug, Deserialize)]
    struct IssueFieldLabel {
        name: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    struct AutoLinkIssuePayload {
        title: Option<String>,
        body: Option<String>,
        state: Option<String>,
        labels: Option<Vec<IssueFieldLabel>>,
    }

    let payload_raw = gh_output_or_empty(&[
        "issue",
        "view",
        issue_number,
        "-R",
        repo_name,
        "--json",
        "title,body,state,labels",
    ]);
    let payload = common_json::from_json_str::<AutoLinkIssuePayload>(&payload_raw).unwrap_or(
        AutoLinkIssuePayload {
            title: None,
            body: None,
            state: None,
            labels: None,
        },
    );
    let labels_raw = payload
        .labels
        .unwrap_or_default()
        .into_iter()
        .filter_map(|label| label.name)
        .collect::<Vec<_>>()
        .join("|");
    (
        payload.title.unwrap_or_default(),
        payload.body.unwrap_or_default(),
        payload.state.unwrap_or_default(),
        labels_raw,
    )
}

fn gh_issue_state_or_empty(repo_name: Option<&str>, issue_number: &str) -> String {
    let mut state_args = vec![
        "issue",
        "view",
        issue_number,
        "--json",
        "state",
        "--jq",
        ".state // \"\"",
    ];
    if let Some(repo) = repo_name {
        state_args.extend(["-R", repo]);
    }

    let state = gh_output_or_empty(&state_args);
    if let Some(normalized) = normalize_issue_state(&state) {
        return normalized.to_string();
    }

    let mut payload_args = vec!["issue", "view", issue_number, "--json", "state"];
    if let Some(repo) = repo_name {
        payload_args.extend(["-R", repo]);
    }

    let payload_raw = gh_output_or_empty(&payload_args);
    if let Some(normalized) = normalize_issue_state(&payload_raw) {
        return normalized.to_string();
    }
    if payload_raw.trim().is_empty() {
        return String::new();
    }

    #[derive(Debug, Deserialize)]
    struct IssueStatePayload {
        state: Option<String>,
    }
    match common_json::from_json_str::<IssueStatePayload>(&payload_raw) {
        Ok(payload) => payload
            .state
            .and_then(|value| normalize_issue_state(&value).map(str::to_string))
            .unwrap_or_default(),
        Err(_) => String::new(),
    }
}

fn normalize_issue_state(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    match trimmed {
        "OPEN" => Some("OPEN"),
        "CLOSED" => Some("CLOSED"),
        _ => None,
    }
}

fn gh_pr_body_or_empty(repo_name: &str, pr_number: &str) -> String {
    let body = gh_output_or_empty(&[
        "pr",
        "view",
        pr_number,
        "-R",
        repo_name,
        "--json",
        "body",
        "--jq",
        ".body // \"\"",
    ]);
    if !body.trim().is_empty() {
        return body;
    }

    let payload_raw =
        gh_output_or_empty(&["pr", "view", pr_number, "-R", repo_name, "--json", "body"]);
    if payload_raw.trim().is_empty() {
        return String::new();
    }
    #[derive(Debug, Deserialize)]
    struct PrBodyPayload {
        body: Option<String>,
    }
    match common_json::from_json_str::<PrBodyPayload>(&payload_raw) {
        Ok(payload) => payload.body.unwrap_or_default(),
        Err(_) => String::new(),
    }
}

type NeutralizeRef = (String, String);
type NeutralizeRefBuckets = (Vec<NeutralizeRef>, Vec<NeutralizeRef>);

fn collect_neutralize_refs(text: &str) -> NeutralizeRefBuckets {
    let re = Regex::new(r"(?i)\b(closes|fixes)\b\s+(rejected\s+)?[^#\s]*#([0-9]+)")
        .expect("static regex must compile");
    let mut closing_refs: Vec<(String, String)> = Vec::new();
    let mut pre_neutralized_refs: Vec<(String, String)> = Vec::new();
    let mut seen_closing = HashSet::new();
    let mut seen_pre = HashSet::new();

    for captures in re.captures_iter(text) {
        let Some(issue_number) = captures.get(3).map(|m| m.as_str()) else {
            continue;
        };
        let issue_key = format!("#{issue_number}");
        if captures.get(2).is_some() {
            if seen_pre.insert(issue_key.clone()) {
                pre_neutralized_refs.push(("Closes".to_string(), issue_key));
            }
        } else if seen_closing.insert(issue_key.clone()) {
            closing_refs.push(("Closes".to_string(), issue_key));
        }
    }

    (closing_refs, pre_neutralized_refs)
}

fn neutralize_reason_for_issue_cached(
    issue_number: &str,
    repo_name: &str,
    cache: &mut HashMap<String, String>,
) -> String {
    let cache_key = format!("#{issue_number}");
    if let Some(value) = cache.get(&cache_key) {
        return value.clone();
    }
    let reason = fetch_non_compliance_reason(issue_number, Some(repo_name)).unwrap_or_default();
    cache.insert(cache_key, reason.clone());
    reason
}

fn apply_rejected_marker(
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

fn remove_rejected_marker(
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

fn build_neutralize_comment_body(
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

fn upsert_pr_marker_comment(repo_name: &str, pr_number: &str, marker: &str, body: &str) -> i32 {
    let list_path = format!("repos/{repo_name}/issues/{pr_number}/comments");
    let marker_query = marker.replace('\\', "\\\\").replace('"', "\\\"");
    let jq_filter = format!(
        "map(select((.body // \"\") | contains(\"{marker_query}\"))) | sort_by(.updated_at) | last | .id // empty"
    );
    let comment_id = gh_output_or_empty(&["api", &list_path, "--paginate", "--jq", &jq_filter]);

    if comment_id.trim().is_empty() {
        execute_command({
            let mut cmd = gh_command(&["api", &list_path]);
            push_arg(&mut cmd, "-f");
            push_arg(&mut cmd, format!("body={body}"));
            cmd
        })
    } else {
        execute_command({
            let mut cmd = gh_command(&[
                "api",
                "-X",
                "PATCH",
                &format!("repos/{repo_name}/issues/comments/{}", comment_id.trim()),
            ]);
            push_arg(&mut cmd, "-f");
            push_arg(&mut cmd, format!("body={body}"));
            cmd
        })
    }
}

pub(crate) fn run_update(opts: UpdateOptions) -> i32 {
    let mut cmd = gh_issue_target_command("edit", &opts.issue, opts.repo.as_deref());
    for (flag, value) in &opts.edit_args {
        push_arg(&mut cmd, flag);
        push_arg(&mut cmd, value);
    }
    let status = execute_command(cmd);
    if status == 0 {
        println!("Issue #{} updated.", opts.issue);
    }
    status
}

pub(crate) fn run_repo_name() -> i32 {
    print_string_result(resolve_repo_name(None), 3)
}

pub(crate) fn run_current_login() -> i32 {
    let login = gh_output_or_empty(&["api", "user", "--jq", ".login"]);
    print_non_empty_lines(&login);
    0
}

pub(crate) fn run_is_root_parent(opts: IsRootParentOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(message) => {
            eprintln!("{message}");
            return 3;
        }
    };
    let (_, body, _, _) = gh_issue_autolink_payload(&repo_name, &opts.issue);
    let parent_value = extract_parent_field(&body)
        .unwrap_or_else(|| "none".to_string())
        .to_lowercase();

    if parent_value == "epic" {
        println!("true");
        return 0;
    }
    if parent_value == "base" {
        println!("false");
        return 0;
    }
    if parent_value.starts_with('#') {
        println!("false");
        return 0;
    }

    let (owner, repo_short) = split_repo_name(&repo_name);
    let has_children =
        !extract_subissue_refs_for_parent(&owner, &repo_short, &opts.issue).is_empty();
    println!("{}", if has_children { "true" } else { "false" });
    0
}

pub(crate) fn run_close(opts: CloseOptions) -> i32 {
    let mut cmd = gh_command(&["issue", "close", &opts.issue, "--reason", &opts.reason]);
    add_repo_arg(&mut cmd, opts.repo.as_deref());
    if let Some(comment) = &opts.comment {
        push_arg(&mut cmd, "--comment");
        push_arg(&mut cmd, comment);
    }
    let status = execute_command(cmd);
    if status == 0 {
        println!("Issue #{} closed (reason: {}).", opts.issue, opts.reason);
    }
    status
}

pub(crate) fn run_reopen(opts: IssueTarget) -> i32 {
    let cmd = gh_issue_target_command("reopen", &opts.issue, opts.repo.as_deref());
    let status = execute_command(cmd);
    if status == 0 {
        println!("Issue #{} reopened.", opts.issue);
    }
    status
}

pub(crate) fn run_delete(opts: IssueTarget) -> i32 {
    let mut cmd = gh_command(&["issue", "close", &opts.issue, "--reason", "not_planned"]);
    add_repo_arg(&mut cmd, opts.repo.as_deref());
    let status = execute_command(cmd);
    if status == 0 {
        println!(
            "Issue #{} soft-deleted (closed with reason: not_planned).",
            opts.issue
        );
    }
    status
}

pub(crate) fn run_reevaluate(opts: ReevaluateOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };

    let pulls_tsv = gh_output_or_empty(&[
        "api",
        &format!("repos/{repo_name}/pulls?state=open&per_page=100"),
        "--paginate",
        "--jq",
        ".[]. | [.number, (.body // \"\")] | @tsv",
    ]);

    let mut pr_numbers: Vec<String> = Vec::new();
    for line in pulls_tsv.lines() {
        let mut parts = line.splitn(2, '\t');
        let pr_num = parts.next().unwrap_or("").trim();
        let pr_body = parts.next().unwrap_or("");
        if pr_num.is_empty() {
            continue;
        }
        if pr_body_references_issue(pr_body, &opts.issue) {
            pr_numbers.push(pr_num.to_string());
        }
    }

    if pr_numbers.is_empty() {
        println!("No open PRs found referencing issue #{}.", opts.issue);
        return 0;
    }

    let mut evaluated_count = 0usize;
    for pr_num in pr_numbers {
        println!(
            "Re-evaluating PR #{} (references issue #{})...",
            pr_num, opts.issue
        );
        let status = run_neutralize(NeutralizeOptions {
            pr: pr_num.clone(),
            repo: Some(repo_name.clone()),
        });
        if status != 0 {
            return status;
        }
        evaluated_count += 1;
    }

    println!(
        "Re-evaluation complete. {} PR(s) evaluated.",
        evaluated_count
    );
    0
}

pub(crate) fn run_parent_guard(opts: ParentGuardOptions) -> i32 {
    let repo_name = match resolve_repo_name(None) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };
    let (repo_owner, repo_short_name) = split_repo_name(&repo_name);

    if let Some(issue_number) = opts.issue.as_deref() {
        return evaluate_parent_issue(
            opts.strict_guard,
            &repo_name,
            &repo_owner,
            &repo_short_name,
            issue_number,
        );
    }

    let Some(child_number) = opts.child.as_deref() else {
        eprintln!("--issue or --child is required");
        return 2;
    };
    let candidates =
        collect_parent_candidates(&repo_name, &repo_owner, &repo_short_name, child_number);
    for parent_number in candidates {
        if parent_number == child_number {
            continue;
        }
        let status = evaluate_parent_issue(
            opts.strict_guard,
            &repo_name,
            &repo_owner,
            &repo_short_name,
            &parent_number,
        );
        if status != 0 {
            return status;
        }
    }
    0
}

pub(crate) fn run_closure_hygiene(opts: ClosureHygieneOptions) -> i32 {
    let repo_name = match resolve_repo_name(opts.repo) {
        Ok(repo) => repo,
        Err(msg) => {
            eprintln!("{msg}");
            return 3;
        }
    };
    let (repo_owner, repo_short_name) = split_repo_name(&repo_name);

    let open_issue_numbers = gh_output_or_empty(&[
        "issue",
        "list",
        "--state",
        "open",
        "--limit",
        "300",
        "--json",
        "number",
        "--jq",
        ".[].number",
        "-R",
        &repo_name,
    ]);
    for issue_number in open_issue_numbers
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        let status = evaluate_parent_issue(
            false,
            &repo_name,
            &repo_owner,
            &repo_short_name,
            issue_number,
        );
        if status != 0 {
            return status;
        }
    }

    let milestones_tsv = gh_output_or_empty(&[
        "api",
        &format!("repos/{repo_name}/milestones?state=open"),
        "--paginate",
        "--jq",
        ".[] | [.number, (.title // \"\"), (.open_issues // 0)] | @tsv",
    ]);
    for line in milestones_tsv.lines() {
        let mut parts = line.splitn(3, '\t');
        let number = parts.next().unwrap_or("").trim();
        let title = parts.next().unwrap_or("").trim();
        let open_issues = parts.next().unwrap_or("").trim();
        if number.is_empty() || open_issues != "0" {
            continue;
        }
        let status = execute_command({
            let mut cmd = gh_command(&[
                "api",
                "-X",
                "PATCH",
                &format!("repos/{repo_name}/milestones/{number}"),
            ]);
            push_arg(&mut cmd, "-f");
            push_arg(&mut cmd, "state=closed");
            cmd
        });
        if status != 0 {
            return status;
        }
        println!("Closed milestone #{} ({}).", number, title);
    }

    println!("Closure hygiene completed.");
    0
}

fn evaluate_parent_issue(
    strict_guard: bool,
    repo_name: &str,
    repo_owner: &str,
    repo_short_name: &str,
    parent_number: &str,
) -> i32 {
    let (_, body, parent_state, _) = gh_issue_autolink_payload(repo_name, parent_number);
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
        let (child_title, _, child_state, _) = gh_issue_autolink_payload(repo_name, child_number);
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
    let status = run_upsert_marker_comment(UpsertMarkerCommentOptions {
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
        let close_status = run_close(CloseOptions {
            issue: parent_number.to_string(),
            repo: Some(repo_name.to_string()),
            reason: "completed".to_string(),
            comment: Some(
                "All required child issues are closed. Auto-closed by parent-issue-guard."
                    .to_string(),
            ),
        });
        if close_status != 0 {
            return close_status;
        }
        println!(
            "Closed parent issue #{} because all required children are closed.",
            parent_number
        );
    }

    if strict_guard && parent_state == "CLOSED" && open_count > 0 {
        let reopen_status = run_reopen(IssueTarget {
            issue: parent_number.to_string(),
            repo: Some(repo_name.to_string()),
        });
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

fn extract_subissue_refs_for_parent(
    repo_owner: &str,
    repo_short_name: &str,
    parent_number: &str,
) -> Vec<String> {
    let output = gh_output_or_empty(&[
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

fn collect_parent_candidates(
    repo_name: &str,
    repo_owner: &str,
    repo_short_name: &str,
    child_number: &str,
) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut seen = HashSet::new();

    let direct = gh_output_or_empty(&[
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
        let search = gh_output_or_empty(&[
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

pub(crate) fn run_required_fields_validate(opts: RequiredFieldsValidateOptions) -> i32 {
    let result = match opts.mode {
        RequiredFieldsValidationMode::Title => validate_title(&opts.title, &opts.labels_raw),
        RequiredFieldsValidationMode::Body => validate_body(&opts.body, &opts.labels_raw),
        RequiredFieldsValidationMode::Content => {
            validate_content(&opts.title, &opts.body, &opts.labels_raw)
        }
    };

    match result {
        Ok(entries) => {
            for entry in entries {
                println!("{}", entry.as_pipe_line());
            }
            0
        }
        Err(message) => {
            eprintln!("{message}");
            1
        }
    }
}

pub(crate) fn run_non_compliance_reason(opts: NonComplianceReasonOptions) -> i32 {
    print_string_result(
        non_compliance_reason_from_content(&opts.title, &opts.body, &opts.labels_raw),
        1,
    )
}

pub(crate) fn run_fetch_non_compliance_reason(opts: FetchNonComplianceReasonOptions) -> i32 {
    print_string_result(
        fetch_non_compliance_reason(&opts.issue, opts.repo.as_deref()),
        1,
    )
}

pub(crate) fn run_label_exists(opts: LabelExistsOptions) -> i32 {
    let labels = gh_output_or_empty(&[
        "label", "list", "-R", &opts.repo, "--limit", "1000", "--json", "name", "--jq", ".[].name",
    ]);
    let exists = labels.lines().any(|name| name.trim() == opts.label);
    println!("{}", if exists { "true" } else { "false" });
    0
}

pub(crate) fn run_open_numbers(opts: OpenNumbersOptions) -> i32 {
    let mut args: Vec<&str> = vec![
        "issue",
        "list",
        "--state",
        "open",
        "--limit",
        "300",
        "--json",
        "number",
        "--jq",
        ".[].number",
    ];
    if let Some(repo) = opts.repo.as_deref() {
        args.push("-R");
        args.push(repo);
    }
    print_non_empty_lines(&gh_output_or_empty(&args));
    0
}

pub(crate) fn run_open_snapshots(opts: OpenSnapshotsOptions) -> i32 {
    let limit = opts.limit.to_string();
    let mut args: Vec<&str> = vec![
        "issue",
        "list",
        "--state",
        "open",
        "--limit",
        &limit,
        "--json",
        "number,title,url,body,state,labels",
        "--jq",
        ".[] | \"\\(.number)|\\((.title // \"\") | @base64)|\\(.url // \"\")|\\((.body // \"\") | @base64)|\\(((.labels // []) | map(.name) | join(\"||\")) | @base64)|\\(.state // \"\")\"",
    ];
    if let Some(repo) = opts.repo.as_deref() {
        args.push("-R");
        args.push(repo);
    }
    print_non_empty_lines(&gh_output_or_empty(&args));
    0
}

pub(crate) fn run_extract_refs(opts: ExtractRefsOptions) -> i32 {
    let text = if let Some(raw) = opts.text {
        raw
    } else if let Some(path) = opts.file {
        match std::fs::read_to_string(&path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("failed to read --file '{}': {}", path, err);
                return 1;
            }
        }
    } else {
        eprintln!("extract-refs requires one input: --text or --file");
        return 1;
    };

    let re = match opts.profile {
        ExtractRefsProfile::Hook => {
            Regex::new(r"(?i)(closes|fixes|part\s+of|reopen|reopens)\s+#([0-9]+)")
        }
        ExtractRefsProfile::Audit => Regex::new(
            r"(?i)(closes|fixes|resolves|part\s+of|related\s+to|reopen|reopens)\s+#([0-9]+)",
        ),
    };
    let re = match re {
        Ok(value) => value,
        Err(err) => {
            eprintln!("failed to compile issue refs regex: {err}");
            return 1;
        }
    };

    let mut seen = HashSet::<String>::new();
    for caps in re.captures_iter(&text) {
        let Some(action_raw) = caps.get(1).map(|m| m.as_str()) else {
            continue;
        };
        let Some(issue_number) = caps.get(2).map(|m| m.as_str()) else {
            continue;
        };
        let action = action_raw.to_ascii_lowercase();
        let key = format!("{action}|{issue_number}");
        if seen.insert(key) {
            println!("{action}|{issue_number}");
        }
    }

    0
}

pub(crate) fn run_assignee_logins(opts: AssigneeLoginsOptions) -> i32 {
    let mut args: Vec<&str> = vec![
        "issue",
        "view",
        &opts.issue,
        "--json",
        "assignees",
        "--jq",
        ".assignees[].login",
    ];
    if let Some(repo) = opts.repo.as_deref() {
        args.push("-R");
        args.push(repo);
    }
    print_non_empty_lines(&gh_output_or_empty(&args));
    0
}

pub(crate) fn run_state(opts: StateOptions) -> i32 {
    let state = gh_issue_state_or_empty(opts.repo.as_deref(), &opts.issue);
    if !state.is_empty() {
        println!("{state}");
    }
    0
}

pub(crate) fn run_has_label(opts: HasLabelOptions) -> i32 {
    let mut args: Vec<&str> = vec![
        "issue",
        "view",
        &opts.issue,
        "--json",
        "labels",
        "--jq",
        ".labels[].name",
    ];
    if let Some(repo) = opts.repo.as_deref() {
        args.push("-R");
        args.push(repo);
    }
    let labels = gh_output_or_empty(&args);
    let exists = labels.lines().any(|name| name.trim() == opts.label);
    println!("{}", if exists { "true" } else { "false" });
    0
}

pub(crate) fn run_list_by_label(opts: ListByLabelOptions) -> i32 {
    let mut args: Vec<&str> = vec![
        "issue",
        "list",
        "--state",
        "open",
        "--limit",
        "300",
        "--label",
        &opts.label,
        "--json",
        "number,title,url",
        "--jq",
        ".[] | \"\\(.number)|\\(.title)|\\(.url)\"",
    ];
    if let Some(repo) = opts.repo.as_deref() {
        args.push("-R");
        args.push(repo);
    }
    print_non_empty_lines(&gh_output_or_empty(&args));
    0
}

pub(crate) fn run_field(opts: IssueFieldOptions) -> i32 {
    #[derive(Debug, Deserialize)]
    struct IssueFieldLabel {
        name: Option<String>,
    }
    #[derive(Debug, Deserialize)]
    struct IssueFieldPayload {
        title: Option<String>,
        body: Option<String>,
        labels: Option<Vec<IssueFieldLabel>>,
    }

    let mut args: Vec<&str> = vec!["issue", "view", &opts.issue, "--json", "title,body,labels"];
    if let Some(repo) = opts.repo.as_deref() {
        args.push("-R");
        args.push(repo);
    }

    let payload_raw = gh_output_or_empty(&args);
    if payload_raw.trim().is_empty() {
        println!();
        return 0;
    }
    let payload = match common_json::from_json_str::<IssueFieldPayload>(&payload_raw) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("failed to parse issue payload: {err}");
            return 1;
        }
    };

    match opts.name {
        IssueFieldName::Title => println!("{}", payload.title.unwrap_or_default()),
        IssueFieldName::Body => println!("{}", payload.body.unwrap_or_default()),
        IssueFieldName::LabelsRaw => {
            let labels = payload
                .labels
                .unwrap_or_default()
                .into_iter()
                .filter_map(|item| item.name)
                .collect::<Vec<_>>()
                .join("||");
            println!("{labels}");
        }
    }

    0
}

pub(crate) fn run_tasklist_refs(opts: TasklistRefsOptions) -> i32 {
    print_non_empty_lines(&extract_tasklist_refs(&opts.body).join("\n"));
    0
}

pub(crate) fn run_subissue_refs(opts: SubissueRefsOptions) -> i32 {
    let query = "query($owner:String!,$name:String!,$number:Int!){repository(owner:$owner,name:$name){issue(number:$number){subIssues(first:100){nodes{number}}}}}";
    let number_as_int = opts.issue.parse::<u32>().unwrap_or_default().to_string();
    let output = gh_output_or_empty(&[
        "api",
        "graphql",
        "-f",
        &format!("query={query}"),
        "-f",
        &format!("owner={}", opts.owner),
        "-f",
        &format!("name={}", opts.repo),
        "-F",
        &format!("number={number_as_int}"),
        "--jq",
        ".data.repository.issue.subIssues.nodes[]?.number | \"#\"+tostring",
    ]);
    print_non_empty_lines(&output);
    0
}

pub(crate) fn run_upsert_marker_comment(opts: UpsertMarkerCommentOptions) -> i32 {
    let comments_endpoint = format!("repos/{}/issues/{}/comments", opts.repo, opts.issue);
    let comments_json = gh_output_or_empty(&["api", &comments_endpoint]);
    let comments = parse_issue_comments(&comments_json);
    let existing_id = find_latest_matching_comment_id(&comments, &opts.marker);

    let had_existing_comment = existing_id.is_some();
    let status = upsert_issue_comment(
        &opts.repo,
        &comments_endpoint,
        &opts.body,
        existing_id.as_deref(),
    );

    if status != 0 {
        return status;
    }

    if opts.announce {
        if had_existing_comment {
            println!("Updated parent status comment on #{}.", opts.issue);
        } else {
            println!("Posted parent status comment on #{}.", opts.issue);
        }
    }

    0
}

fn execute_command(command: Vec<String>) -> i32 {
    let borrowed = command.iter().map(String::as_str).collect::<Vec<&str>>();
    match crate::gh_cli::status(&borrowed) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("Failed to execute command: {err}");
            1
        }
    }
}

fn print_string_result(result: Result<String, String>, error_code: i32) -> i32 {
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

fn gh_command(prefix: &[&str]) -> Vec<String> {
    prefix.iter().map(|value| (*value).to_string()).collect()
}

fn add_repo_arg(cmd: &mut Vec<String>, repo: Option<&str>) {
    if let Some(repo_name) = repo {
        push_arg(cmd, "-R");
        push_arg(cmd, repo_name);
    }
}

fn gh_issue_target_command(action: &str, issue: &str, repo: Option<&str>) -> Vec<String> {
    let mut cmd = gh_command(&["issue", action, issue]);
    add_repo_arg(&mut cmd, repo);
    cmd
}

fn gh_output_or_empty(args: &[&str]) -> String {
    gh_output(args, true).unwrap_or_default()
}

fn gh_output(args: &[&str], silence_stderr: bool) -> Result<String, String> {
    match crate::gh_cli::output_trim(args) {
        Ok(value) => Ok(value),
        Err(_) if silence_stderr => Ok(String::new()),
        Err(message) => Err(message),
    }
}

fn pr_body_references_issue(body: &str, issue_number: &str) -> bool {
    let pattern = format!(
        r"(?i)\b(closes|fixes)\b\s+(rejected\s+)?[^#\s]*#{}(?:\b|[^0-9])",
        regex::escape(issue_number)
    );
    Regex::new(&pattern)
        .map(|re| re.is_match(body))
        .unwrap_or(false)
}

fn extract_closing_issue_numbers(text: &str) -> Vec<String> {
    let re = Regex::new(r"(?i)\b(closes|fixes)\b\s+(?:rejected\s+)?[^#\s]*#([0-9]+)")
        .expect("static regex must compile");
    let mut seen: HashSet<String> = HashSet::new();
    let mut out = Vec::new();
    for captures in re.captures_iter(text) {
        let Some(num) = captures.get(2).map(|m| m.as_str().to_string()) else {
            continue;
        };
        if seen.insert(num.clone()) {
            out.push(num);
        }
    }
    out
}

fn extract_reopen_issue_numbers(text: &str) -> Vec<String> {
    let re = Regex::new(r"(?i)\breopen\b\s+(?:rejected\s+)?[^#\s]*#([0-9]+)")
        .expect("static regex must compile");
    let mut seen: HashSet<String> = HashSet::new();
    let mut out = Vec::new();
    for captures in re.captures_iter(text) {
        let Some(num) = captures.get(1).map(|m| m.as_str().to_string()) else {
            continue;
        };
        if seen.insert(num.clone()) {
            out.push(num);
        }
    }
    out
}

fn print_non_empty_lines(text: &str) {
    for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
        println!("{line}");
    }
}

fn upsert_issue_comment(
    repo: &str,
    comments_endpoint: &str,
    body: &str,
    comment_id: Option<&str>,
) -> i32 {
    let mut cmd = gh_command(&["api"]);
    if let Some(id) = comment_id {
        push_arg(&mut cmd, "-X");
        push_arg(&mut cmd, "PATCH");
        push_arg(&mut cmd, format!("repos/{repo}/issues/comments/{id}"));
    } else {
        push_arg(&mut cmd, comments_endpoint);
    }
    push_arg(&mut cmd, "-f");
    push_arg(&mut cmd, format!("body={body}"));
    execute_command(cmd)
}

fn push_arg<T: Into<String>>(cmd: &mut Vec<String>, value: T) {
    cmd.push(value.into());
}
