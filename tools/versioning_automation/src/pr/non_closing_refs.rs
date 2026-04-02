use std::collections::HashSet;

use crate::pr::commands::pr_non_closing_refs_options::PrNonClosingRefsOptions;
use crate::pr::text_payload::extract_effective_issue_ref_records;

pub(crate) fn run_non_closing_refs(opts: PrNonClosingRefsOptions) -> i32 {
    let mut seen = HashSet::new();

    for record in extract_effective_issue_ref_records(&opts.text) {
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
