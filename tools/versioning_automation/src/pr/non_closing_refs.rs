use std::collections::HashSet;

use crate::pr::commands::pr_non_closing_refs_options::PrNonClosingRefsOptions;
use crate::pr::domain::directives::directive_record_type::DirectiveRecordType;
use crate::pr::scan::scan_directives;

pub(crate) fn run_non_closing_refs(opts: PrNonClosingRefsOptions) -> i32 {
    let mut seen = HashSet::new();

    for record in scan_directives(&opts.text, false) {
        if record.record_type != DirectiveRecordType::Event {
            continue;
        }
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
