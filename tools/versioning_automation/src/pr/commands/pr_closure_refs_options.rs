//! tools/versioning_automation/src/pr/commands/pr_closure_refs_options.rs
use std::collections::HashSet;

use crate::pr::{DirectiveRecord, DirectiveRecordType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PrClosureRefsOptions {
    pub(crate) text: String,
}

impl PrClosureRefsOptions {
    pub(crate) fn run_closure_refs(self) -> i32 {
        let mut seen = HashSet::new();

        for record in DirectiveRecord::scan_directives(&self.text, false) {
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
}
