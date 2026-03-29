//! tools/versioning_automation/src/issues/commands/open_numbers_options.rs
use crate::{gh_cli::output_trim_or_empty, issues::execute::print_non_empty_lines};

#[derive(Debug, Clone)]
pub(crate) struct OpenNumbersOptions {
    pub(crate) repo: Option<String>,
}

impl OpenNumbersOptions {
    pub(crate) fn run_open_numbers(self) -> i32 {
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
        if let Some(repo) = self.repo.as_deref() {
            args.push("-R");
            args.push(repo);
        }
        print_non_empty_lines(&output_trim_or_empty(&args));
        0
    }
}
