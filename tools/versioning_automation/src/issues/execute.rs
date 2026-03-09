//! tools/versioning_automation/src/issues/execute.rs
use std::process::Command;

use crate::issues::model::{CloseOptions, CreateOptions, IssueTarget, ReadOptions, UpdateOptions};
use crate::issues::render::render_direct_issue_body;

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

fn execute_command(mut command: Command) -> i32 {
    match command.status() {
        Ok(status) => status.code().unwrap_or(1),
        Err(err) => {
            eprintln!("Failed to execute command: {err}");
            1
        }
    }
}
