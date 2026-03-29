//! tools/versioning_automation/src/issues/commands/label_exists_options.rs
use crate::gh_cli::output_trim_or_empty;

#[derive(Debug, Clone)]
pub(crate) struct LabelExistsOptions {
    pub(crate) repo: String,
    pub(crate) label: String,
}

impl LabelExistsOptions {
    pub(crate) fn run_label_exists(self) -> i32 {
        let labels = output_trim_or_empty(&[
            "label", "list", "-R", &self.repo, "--limit", "1000", "--json", "name", "--jq",
            ".[].name",
        ]);
        let exists = labels.lines().any(|name| name.trim() == self.label);
        println!("{}", if exists { "true" } else { "false" });
        0
    }
}
