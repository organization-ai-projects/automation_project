//! tools/versioning_automation/src/pr/commands/pr_group_by_category_options.rs
use crate::pr::group_by_category::GroupByCategory;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrGroupByCategoryOptions {
    pub(crate) text: String,
    pub(crate) mode: String,
}

impl PrGroupByCategoryOptions {
    pub(crate) fn run_group_by_category(self) -> i32 {
        let mode = self.mode.as_str();
        if !matches!(mode, "resolved" | "reopen" | "conflict" | "directive") {
            eprintln!("--mode must be one of: resolved, reopen, conflict, directive");
            return 2;
        }

        let mut records = GroupByCategory::parse_records(&self.text);
        records.sort_by_key(|record| (record.0, record.3));

        let output = GroupByCategory::render_grouped_output(&records, mode);
        print!("{output}");
        0
    }
}
