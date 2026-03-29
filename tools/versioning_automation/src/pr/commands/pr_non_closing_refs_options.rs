//! tools/versioning_automation/src/pr/commands/pr_non_closing_refs_options.rs
use std::collections::HashSet;

use crate::pr::extract_effective_issue_ref_records;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrNonClosingRefsOptions {
    pub(crate) text: String,
}

impl PrNonClosingRefsOptions {
    pub(crate) fn run_non_closing_refs(self) -> i32 {
        let mut seen = HashSet::new();

        for record in extract_effective_issue_ref_records(&self.text) {
            if record.first != "Part of" {
                continue;
            }

            let key = format!("PART|{}", record.second);
            if seen.insert(key) {
                println!("Part of|{}", record.second);
            }
        }

        0
    }
}
