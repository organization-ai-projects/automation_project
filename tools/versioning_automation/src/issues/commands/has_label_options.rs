//! tools/versioning_automation/src/issues/commands/has_label_options.rs
use crate::gh_cli::output_trim_or_empty;

#[derive(Debug, Clone)]
pub(crate) struct HasLabelOptions {
    pub(crate) issue: String,
    pub(crate) label: String,
    pub(crate) repo: Option<String>,
}

impl HasLabelOptions {
    pub(crate) fn run_has_label(self) -> i32 {
        let mut args: Vec<&str> = vec![
            "issue",
            "view",
            &self.issue,
            "--json",
            "labels",
            "--jq",
            ".labels[].name",
        ];
        if let Some(repo) = self.repo.as_deref() {
            args.push("-R");
            args.push(repo);
        }
        let labels = output_trim_or_empty(&args);
        let exists = labels.lines().any(|name| name.trim() == self.label);
        println!("{}", if exists { "true" } else { "false" });
        0
    }
}
