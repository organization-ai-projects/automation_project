//! tools/versioning_automation/src/issues/commands/open_snapshots_options.rs
use crate::{gh_cli::output_trim_or_empty, issues::execute::print_non_empty_lines};

#[derive(Debug, Clone)]
pub(crate) struct OpenSnapshotsOptions {
    pub(crate) repo: Option<String>,
    pub(crate) limit: usize,
}

impl OpenSnapshotsOptions {
    pub(crate) fn run_open_snapshots(self) -> i32 {
        let limit = self.limit.to_string();
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
        if let Some(repo) = self.repo.as_deref() {
            args.push("-R");
            args.push(repo);
        }
        print_non_empty_lines(&output_trim_or_empty(&args));
        0
    }
}
