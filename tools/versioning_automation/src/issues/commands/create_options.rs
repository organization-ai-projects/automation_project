//! tools/versioning_automation/src/issues/commands/create_options.rs

use crate::{
    gh_cli::{add_repo_arg, gh_command, push_arg, status_code_owned},
    issues::render::render_direct_issue_body,
};
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
        let body = render_direct_issue_body(self);
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
}
