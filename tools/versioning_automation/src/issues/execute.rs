//! tools/versioning_automation/src/issues/execute.rs
use std::process::Command;

use regex::Regex;

use crate::issues::commands::{
    CloseOptions, CreateOptions, FetchNonComplianceReasonOptions, IssueTarget, LabelExistsOptions,
    NonComplianceReasonOptions, ReadOptions, ReevaluateOptions, RequiredFieldsValidateOptions,
    RequiredFieldsValidationMode, UpdateOptions,
};
use crate::issues::render::render_direct_issue_body;
use crate::issues::required_fields::{
    fetch_non_compliance_reason, non_compliance_reason_from_content, validate_body,
    validate_content, validate_title,
};

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

    let mut cmd = Command::new("gh");
    cmd.arg("issue")
        .arg("create")
        .arg("--title")
        .arg(&opts.title)
        .arg("--body")
        .arg(&body);
    if let Some(repo) = &opts.repo {
        cmd.arg("-R").arg(repo);
    }
    for label in &opts.labels {
        cmd.arg("--label").arg(label);
    }
    execute_command(cmd)
}

pub(crate) fn run_read(opts: ReadOptions) -> i32 {
    let mut cmd = Command::new("gh");
    cmd.arg("issue");
    if let Some(issue) = &opts.issue {
        cmd.arg("view").arg(issue);
    } else {
        cmd.arg("list");
    }
    if let Some(repo) = &opts.repo {
        cmd.arg("-R").arg(repo);
    }
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

pub(crate) fn run_update(opts: UpdateOptions) -> i32 {
    let mut cmd = Command::new("gh");
    cmd.arg("issue").arg("edit").arg(&opts.issue);
    if let Some(repo) = &opts.repo {
        cmd.arg("-R").arg(repo);
    }
    for (flag, value) in &opts.edit_args {
        cmd.arg(flag).arg(value);
    }
    execute_command(cmd)
}

pub(crate) fn run_close(opts: CloseOptions) -> i32 {
    let mut cmd = Command::new("gh");
    cmd.arg("issue")
        .arg("close")
        .arg(&opts.issue)
        .arg("--reason")
        .arg(&opts.reason);
    if let Some(repo) = &opts.repo {
        cmd.arg("-R").arg(repo);
    }
    execute_command(cmd)
}

pub(crate) fn run_reopen(opts: IssueTarget) -> i32 {
    let mut cmd = Command::new("gh");
    cmd.arg("issue").arg("reopen").arg(&opts.issue);
    if let Some(repo) = &opts.repo {
        cmd.arg("-R").arg(repo);
    }
    execute_command(cmd)
}

pub(crate) fn run_delete(opts: IssueTarget) -> i32 {
    let mut cmd = Command::new("gh");
    cmd.arg("issue")
        .arg("close")
        .arg(&opts.issue)
        .arg("--reason")
        .arg("not_planned");
    if let Some(repo) = &opts.repo {
        cmd.arg("-R").arg(repo);
    }
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

    let pulls_tsv = gh_output(
        &[
            "api",
            &format!("repos/{repo_name}/pulls?state=open&per_page=100"),
            "--paginate",
            "--jq",
            ".[]. | [.number, (.body // \"\")] | @tsv",
        ],
        true,
    )
    .unwrap_or_default();

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
    match non_compliance_reason_from_content(&opts.title, &opts.body, &opts.labels_raw) {
        Ok(reason) => {
            println!("{reason}");
            0
        }
        Err(message) => {
            eprintln!("{message}");
            1
        }
    }
}

pub(crate) fn run_fetch_non_compliance_reason(opts: FetchNonComplianceReasonOptions) -> i32 {
    match fetch_non_compliance_reason(&opts.issue, opts.repo.as_deref()) {
        Ok(reason) => {
            println!("{reason}");
            0
        }
        Err(message) => {
            eprintln!("{message}");
            1
        }
    }
}

pub(crate) fn run_label_exists(opts: LabelExistsOptions) -> i32 {
    let labels = gh_output(
        &[
            "label", "list", "-R", &opts.repo, "--limit", "1000", "--json", "name", "--jq",
            ".[].name",
        ],
        true,
    )
    .unwrap_or_default();
    let exists = labels.lines().any(|name| name.trim() == opts.label);
    if exists {
        println!("true");
    } else {
        println!("false");
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

fn resolve_repo_name(explicit_repo: Option<String>) -> Result<String, String> {
    if let Some(repo) = explicit_repo.filter(|value| !value.trim().is_empty()) {
        return Ok(repo);
    }
    if let Ok(env_repo) = std::env::var("GH_REPO")
        && !env_repo.trim().is_empty()
    {
        return Ok(env_repo);
    }
    let resolved = gh_output(
        &[
            "repo",
            "view",
            "--json",
            "nameWithOwner",
            "--jq",
            ".nameWithOwner",
        ],
        true,
    )
    .unwrap_or_default();
    if resolved.trim().is_empty() {
        Err("Error: unable to determine repository.".to_string())
    } else {
        Ok(resolved)
    }
}

fn gh_output(args: &[&str], silence_stderr: bool) -> Result<String, String> {
    let mut cmd = Command::new("gh");
    cmd.args(args);
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
