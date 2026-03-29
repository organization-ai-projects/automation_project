//! tools/versioning_automation/src/issues/commands/read_options.rs
use crate::gh_cli::{add_repo_arg, gh_command, push_arg, status_code_owned};

#[derive(Debug, Clone)]
pub(crate) struct ReadOptions {
    pub(crate) issue: Option<String>,
    pub(crate) repo: Option<String>,
    pub(crate) json: Option<String>,
    pub(crate) jq: Option<String>,
    pub(crate) template: Option<String>,
}

impl ReadOptions {
    pub(crate) fn run_read(self) -> i32 {
        let mut cmd = gh_command(&["issue"]);
        if let Some(issue) = &self.issue {
            push_arg(&mut cmd, "view");
            push_arg(&mut cmd, issue);
        } else {
            push_arg(&mut cmd, "list");
        }
        add_repo_arg(&mut cmd, self.repo.as_deref());
        if let Some(json) = &self.json {
            push_arg(&mut cmd, "--json");
            push_arg(&mut cmd, json);
        }
        if let Some(jq) = &self.jq {
            push_arg(&mut cmd, "--jq");
            push_arg(&mut cmd, jq);
        }
        if let Some(template) = &self.template {
            push_arg(&mut cmd, "--template");
            push_arg(&mut cmd, template);
        }
        status_code_owned(cmd)
    }
}
