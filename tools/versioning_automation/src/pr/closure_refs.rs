//! tools/versioning_automation/src/pr/closure_refs.rs
use std::collections::HashSet;

use crate::pr::commands::PrClosureRefsOptions;
use crate::pr::domain::directives::directive_record_type::DirectiveRecordType;
use crate::pr::scan_directives;

pub(crate) fn run_closure_refs(opts: PrClosureRefsOptions) -> i32 {
    let mut seen = HashSet::new();

    for record in scan_directives(&opts.text, false) {
        if record.record_type != DirectiveRecordType::Event {
            continue;
        }

        if record.first == "Closes" {
            let key = format!("CLOSE|{}", record.second);
            if seen.insert(key) {
                println!("CLOSE|Closes|{}", record.second);
            }
        } else if record.first == "Closes rejected" {
            let key = format!("PRE|{}", record.second);
            if seen.insert(key) {
                println!("PRE|Closes|{}", record.second);
            }
        }
    }

    0
}
