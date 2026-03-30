//! tools/versioning_automation/src/issues/commands/create_options.rs
use crate::gh_cli::{add_repo_arg, gh_command, push_arg, status_code_owned};

#[derive(Debug, Clone)]
pub(crate) struct CreateOptions {
    pub(crate) title: String,
    pub(crate) context: String,
    pub(crate) problem: String,
    pub(crate) acceptances: Vec<String>,
    pub(crate) parent: String,
    pub(crate) labels: Vec<String>,
    pub(crate) assignees: Vec<String>,
    pub(crate) related_issues: Vec<String>,
    pub(crate) related_prs: Vec<String>,
    pub(crate) repo: Option<String>,
    pub(crate) dry_run: bool,
}

impl CreateOptions {
    pub(crate) fn _run(&self) -> i32 {
        let body = Self::render_direct_issue_body(self);
        if self.dry_run {
            println!("Dry-run mode. Issue was not created.");
            println!("----- title -----");
            println!("{}", self.title);
            println!("----- body -----");
            println!("{}", body);
            return 0;
        }

        let mut cmd = gh_command(&["issue", "create"]);
        push_arg(&mut cmd, "--title");
        push_arg(&mut cmd, &self.title);
        push_arg(&mut cmd, "--body");
        push_arg(&mut cmd, &body);
        add_repo_arg(&mut cmd, self.repo.as_deref());
        for label in &self.labels {
            push_arg(&mut cmd, "--label");
            push_arg(&mut cmd, label);
        }
        for assignee in &self.assignees {
            push_arg(&mut cmd, "--assignee");
            push_arg(&mut cmd, assignee);
        }
        status_code_owned(cmd)
    }

    pub(crate) fn run_create(self) -> i32 {
        let body = Self::render_direct_issue_body(&self);
        if self.dry_run {
            println!("Dry-run mode. Issue was not created.");
            println!("----- title -----");
            println!("{}", self.title);
            println!("----- body -----");
            println!("{}", body);
            return 0;
        }

        let mut cmd = gh_command(&["issue", "create"]);
        push_arg(&mut cmd, "--title");
        push_arg(&mut cmd, &self.title);
        push_arg(&mut cmd, "--body");
        push_arg(&mut cmd, &body);
        add_repo_arg(&mut cmd, self.repo.as_deref());
        for label in &self.labels {
            push_arg(&mut cmd, "--label");
            push_arg(&mut cmd, label);
        }
        for assignee in &self.assignees {
            push_arg(&mut cmd, "--assignee");
            push_arg(&mut cmd, assignee);
        }
        status_code_owned(cmd)
    }

    pub(crate) fn render_direct_issue_body(&self) -> String {
        let mut body = String::new();
        body.push_str("## Context\n\n");
        body.push_str(&self.context);
        body.push_str("\n\n## Problem\n\n");
        body.push_str(&self.problem);
        body.push_str("\n\n## Acceptance Criteria\n\nDone when :\n\n");
        for acceptance in &self.acceptances {
            body.push_str("- [ ] ");
            body.push_str(acceptance);
            body.push('\n');
        }
        body.push_str("\n## Hierarchy\n\nParent: ");
        body.push_str(&self.parent);

        let related_issues = self
            .related_issues
            .iter()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>();
        let related_prs = self
            .related_prs
            .iter()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>();

        if !related_issues.is_empty() || !related_prs.is_empty() {
            body.push_str("\n\n## References\n");
            if !related_issues.is_empty() {
                body.push_str("\nRelated issue(s): ");
                body.push_str(&related_issues.join(" "));
            }
            if !related_prs.is_empty() {
                body.push_str("\nRelated PR(s): ");
                body.push_str(&related_prs.join(" "));
            }
        }

        body
    }
}
