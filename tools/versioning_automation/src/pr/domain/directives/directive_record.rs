//! tools/versioning_automation/src/pr/domain/directives/directive_record.rs
use std::collections::HashSet;

use common_json::to_string_pretty;
use regex::Regex;
use serde::Serialize;

use crate::pr::DirectiveRecordType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub(crate) struct DirectiveRecord {
    pub(crate) record_type: DirectiveRecordType,
    pub(crate) first: String,
    pub(crate) second: String,
}

impl DirectiveRecord {
    pub(crate) fn scan_directives(text: &str, unique: bool) -> Vec<Self> {
        let mut matches: Vec<(usize, Self)> = Vec::new();

        let decision_re = Regex::new(
            r"(?i)directive[\s_-]*decision\s*:\s*[^#\s]*#([0-9]+)\s*=>\s*(close|reopen)",
        )
        .expect("valid regex");
        for caps in decision_re.captures_iter(text) {
            let full = caps.get(0).expect("full capture");
            let issue = format!("#{}", caps.get(1).expect("issue capture").as_str());
            let decision = caps
                .get(2)
                .expect("decision capture")
                .as_str()
                .to_lowercase();
            matches.push((
                full.start(),
                Self {
                    record_type: DirectiveRecordType::Decision,
                    first: issue,
                    second: decision,
                },
            ));
        }

        let duplicate_re =
            Regex::new(r"(?i)#([0-9]+)\s+duplicate\s+of\s+#([0-9]+)").expect("valid regex");
        for caps in duplicate_re.captures_iter(text) {
            let full = caps.get(0).expect("full capture");
            let duplicate = format!("#{}", caps.get(1).expect("duplicate capture").as_str());
            let canonical = format!("#{}", caps.get(2).expect("canonical capture").as_str());
            matches.push((
                full.start(),
                Self {
                    record_type: DirectiveRecordType::Duplicate,
                    first: duplicate,
                    second: canonical,
                },
            ));
        }

        let event_re = Regex::new(
        r"(?i)\b(cancel[\s_-]*closes|closes|fixes|reopen|reopens|part[[:space:]]+of)\b\s+(rejected\s+)?[^#\s]*#([0-9]+)",
    )
    .expect("valid regex");
        for caps in event_re.captures_iter(text) {
            let full = caps.get(0).expect("full capture");
            let token = caps.get(1).expect("token capture").as_str().to_lowercase();
            let has_rejected = caps.get(2).is_some();
            let action = if token == "closes" || token == "fixes" {
                if has_rejected {
                    "Closes rejected".to_string()
                } else {
                    "Closes".to_string()
                }
            } else if token == "part of" {
                "Part of".to_string()
            } else if token.starts_with("cancel") {
                "Cancel-Closes".to_string()
            } else {
                "Reopen".to_string()
            };
            let issue = format!("#{}", caps.get(3).expect("issue capture").as_str());
            matches.push((
                full.start(),
                Self {
                    record_type: DirectiveRecordType::Event,
                    first: action,
                    second: issue,
                },
            ));
        }

        matches.sort_by_key(|(start, _)| *start);

        let mut out = Vec::new();
        if unique {
            let mut seen = HashSet::new();
            for (_, record) in matches {
                let key = format!(
                    "{:?}|{}|{}",
                    record.record_type, record.first, record.second
                );
                if seen.insert(key) {
                    out.push(record);
                }
            }
        } else {
            for (_, record) in matches {
                out.push(record);
            }
        }

        out
    }

    pub(crate) fn emit_plain(records: &[Self]) {
        for record in records {
            let record_type = match record.record_type {
                DirectiveRecordType::Event => "EV",
                DirectiveRecordType::Decision => "DEC",
                DirectiveRecordType::Duplicate => "DUP",
            };
            println!("{}|{}|{}", record_type, record.first, record.second);
        }
    }

    pub(crate) fn emit_json(records: &[Self]) -> i32 {
        let payload = records.to_vec();
        match to_string_pretty(&payload) {
            Ok(json) => {
                println!("{json}");
                0
            }
            Err(err) => {
                eprintln!("failed to serialize directives as json: {err}");
                1
            }
        }
    }
}
