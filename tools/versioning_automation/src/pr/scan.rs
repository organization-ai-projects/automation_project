use std::collections::HashSet;

use regex::Regex;

use crate::pr::model::directive_record::DirectiveRecord;
use crate::pr::model::directive_record_type::DirectiveRecordType;

struct ScanMatch {
    start: usize,
    record: DirectiveRecord,
}

pub(crate) fn scan_directives(text: &str, unique: bool) -> Vec<DirectiveRecord> {
    let mut matches = Vec::new();

    let decision_re =
        Regex::new(r"(?i)directive[\s_-]*decision\s*:\s*[^#\s]*#([0-9]+)\s*=>\s*(close|reopen)")
            .expect("valid regex");
    for caps in decision_re.captures_iter(text) {
        let full = caps.get(0).expect("full capture");
        let issue = format!("#{}", caps.get(1).expect("issue capture").as_str());
        let decision = caps
            .get(2)
            .expect("decision capture")
            .as_str()
            .to_lowercase();
        matches.push(ScanMatch {
            start: full.start(),
            record: DirectiveRecord {
                record_type: DirectiveRecordType::Decision,
                first: issue,
                second: decision,
            },
        });
    }

    let duplicate_re =
        Regex::new(r"(?i)#([0-9]+)\s+duplicate\s+of\s+#([0-9]+)").expect("valid regex");
    for caps in duplicate_re.captures_iter(text) {
        let full = caps.get(0).expect("full capture");
        let duplicate = format!("#{}", caps.get(1).expect("duplicate capture").as_str());
        let canonical = format!("#{}", caps.get(2).expect("canonical capture").as_str());
        matches.push(ScanMatch {
            start: full.start(),
            record: DirectiveRecord {
                record_type: DirectiveRecordType::Duplicate,
                first: duplicate,
                second: canonical,
            },
        });
    }

    let event_re = Regex::new(
        r"(?i)\b(closes|fixes|reopen|reopens|part[[:space:]]+of)\b\s+(rejected\s+)?[^#\s]*#([0-9]+)",
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
        } else {
            "Reopen".to_string()
        };
        let issue = format!("#{}", caps.get(3).expect("issue capture").as_str());
        matches.push(ScanMatch {
            start: full.start(),
            record: DirectiveRecord {
                record_type: DirectiveRecordType::Event,
                first: action,
                second: issue,
            },
        });
    }

    matches.sort_by_key(|m| m.start);

    let mut out = Vec::new();
    if unique {
        let mut seen = HashSet::new();
        for item in matches {
            let key = format!(
                "{:?}|{}|{}",
                item.record.record_type, item.record.first, item.record.second
            );
            if seen.insert(key) {
                out.push(item.record);
            }
        }
    } else {
        for item in matches {
            out.push(item.record);
        }
    }

    out
}
