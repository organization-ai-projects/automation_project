//! tools/versioning_automation/src/issues/execute.rs
use std::collections::HashSet;
use std::process::Command;

use regex::Regex;
use serde::Deserialize;

use crate::issues::commands::{
    AssigneeLoginsOptions, CloseOptions, CreateOptions, DoneStatusMode, DoneStatusOptions,
    FetchNonComplianceReasonOptions, HasLabelOptions, IssueFieldName, IssueFieldOptions,
    IssueTarget, LabelExistsOptions, ListByLabelOptions, NonComplianceReasonOptions,
    OpenNumbersOptions, ReadOptions, ReevaluateOptions, RequiredFieldsValidateOptions,
    RequiredFieldsValidationMode, StateOptions, SubissueRefsOptions, TasklistRefsOptions,
    UpdateOptions, UpsertMarkerCommentOptions,
};
use crate::issues::issue_comments::{find_latest_matching_comment_id, parse_issue_comments};
use crate::issues::render::render_direct_issue_body;
use crate::issues::required_fields::{
    fetch_non_compliance_reason, non_compliance_reason_from_content, validate_body,
    validate_content, validate_title,
};
use crate::issues::tasklist_refs::extract_tasklist_refs;
use crate::repo_name::resolve_repo_name;

#[derive(Debug, Deserialize)]
struct IssueFieldPayload {
    title: Option<String>,
    body: Option<String>,
    labels: Option<Vec<IssueFieldLabel>>,
}

#[derive(Debug, Deserialize)]
struct IssueFieldLabel {
    name: Option<String>,
}

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
    cmd.arg("--title").arg(&opts.title).arg("--body").arg(&body);
    add_repo_arg(&mut cmd, opts.repo.as_deref());
    for label in &opts.labels {
        cmd.arg("--label").arg(label);
    }
    for assignee in &opts.assignees {
        cmd.arg("--assignee").arg(assignee);
    }
    execute_command(cmd)
}

pub(crate) fn run_read(opts: ReadOptions) -> i32 {
    let mut cmd = gh_command(&["issue"]);
    if let Some(issue) = &opts.issue {
        cmd.arg("view").arg(issue);
    } else {
        cmd.arg("list");
    }
    add_repo_arg(&mut cmd, opts.repo.as_deref());
    if let Some(json) = &opts.json {
        cmd.arg("--json").arg(json);
    }
    if let Some(jq) = &opts.jq {
        cmd.arg("--jq").arg(jq);
    }
    if let Some(template) = &opts.template {
        cmd.arg("--template").arg(template);
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
                let state = gh_output_or_empty(&[
                    "issue",
                    "view",
                    &issue_number,
                    "-R",
                    &repo_name,
                    "--json",
                    "state",
                    "--jq",
                    ".state // \"\"",
                ]);
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
                    cmd.arg("--add-label").arg(&label_name);
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
                    cmd.arg("--remove-label").arg(&label_name);
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

pub(crate) fn run_update(opts: UpdateOptions) -> i32 {
    let mut cmd = gh_issue_target_command("edit", &opts.issue, opts.repo.as_deref());
    for (flag, value) in &opts.edit_args {
        cmd.arg(flag).arg(value);
    }
    execute_command(cmd)
}

pub(crate) fn run_repo_name() -> i32 {
    print_string_result(resolve_repo_name(None), 3)
}

pub(crate) fn run_close(opts: CloseOptions) -> i32 {
    let mut cmd = gh_issue_target_command("close", &opts.issue, opts.repo.as_deref());
    cmd.arg("--reason").arg(&opts.reason);
    if let Some(comment) = &opts.comment {
        cmd.arg("--comment").arg(comment);
    }
    execute_command(cmd)
}

pub(crate) fn run_reopen(opts: IssueTarget) -> i32 {
    let cmd = gh_issue_target_command("reopen", &opts.issue, opts.repo.as_deref());
    execute_command(cmd)
}

pub(crate) fn run_delete(opts: IssueTarget) -> i32 {
    let mut cmd = gh_issue_target_command("close", &opts.issue, opts.repo.as_deref());
    cmd.arg("--reason").arg("not_planned");
    execute_command(cmd)
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

    let neutralizer = "scripts/versioning/file_versioning/github/issues/neutralize/run.sh";
    let mut evaluated_count = 0usize;
    for pr_num in pr_numbers {
        println!(
            "Re-evaluating PR #{} (references issue #{})...",
            pr_num, opts.issue
        );
        let mut cmd = Command::new("bash");
        cmd.arg(neutralizer)
            .arg("--pr")
            .arg(&pr_num)
            .arg("--repo")
            .arg(&repo_name);
        match cmd.status() {
            Ok(status) if status.success() => {
                evaluated_count += 1;
            }
            Ok(status) => return status.code().unwrap_or(1),
            Err(err) => {
                eprintln!("Failed to execute neutralizer: {err}");
                return 1;
            }
        }
    }

    println!(
        "Re-evaluation complete. {} PR(s) evaluated.",
        evaluated_count
    );
    0
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
    let mut args: Vec<&str> = vec![
        "issue",
        "view",
        &opts.issue,
        "--json",
        "state",
        "--jq",
        ".state // \"\"",
    ];
    if let Some(repo) = opts.repo.as_deref() {
        args.push("-R");
        args.push(repo);
    }
    print_non_empty_lines(&gh_output_or_empty(&args));
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

fn execute_command(mut command: Command) -> i32 {
    match command.status() {
        Ok(status) => status.code().unwrap_or(1),
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

fn gh_command(prefix: &[&str]) -> Command {
    let mut cmd = Command::new("gh");
    cmd.args(prefix);
    cmd
}

fn add_repo_arg(cmd: &mut Command, repo: Option<&str>) {
    if let Some(repo_name) = repo {
        cmd.arg("-R").arg(repo_name);
    }
}

fn gh_issue_target_command(action: &str, issue: &str, repo: Option<&str>) -> Command {
    let mut cmd = gh_command(&["issue", action, issue]);
    add_repo_arg(&mut cmd, repo);
    cmd
}

fn gh_output_or_empty(args: &[&str]) -> String {
    gh_output(args, true).unwrap_or_default()
}

fn gh_output(args: &[&str], silence_stderr: bool) -> Result<String, String> {
    let mut cmd = gh_command(args);
    let output = cmd.output().map_err(|err| err.to_string())?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else if silence_stderr {
        Ok(String::new())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
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
        cmd.arg("-X")
            .arg("PATCH")
            .arg(format!("repos/{repo}/issues/comments/{id}"));
    } else {
        cmd.arg(comments_endpoint);
    }
    cmd.arg("-f").arg(format!("body={body}"));
    execute_command(cmd)
}
