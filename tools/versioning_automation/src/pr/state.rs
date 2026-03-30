//! tools/versioning_automation/src/pr/state.rs
use std::collections::{HashMap, HashSet};

use crate::pr::{DirectiveRecord, DirectiveRecordType, commands::PrDirectivesStateOptions};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct State {
    pub(crate) explicit_decisions: HashMap<String, String>,
    pub(crate) inferred_decisions: HashMap<String, String>,
    pub(crate) action_records: Vec<DirectiveRecord>,
}

impl State {
    pub(crate) fn build_state(text: &str) -> Self {
        let records = DirectiveRecord::scan_directives(text, false);
        let explicit_decisions = collect_explicit_decisions(&records);
        let inferred_decisions = collect_inferred_decisions(&records, &explicit_decisions);
        let action_records = collect_action_records(&records);

        State {
            explicit_decisions,
            inferred_decisions,
            action_records,
        }
    }

    pub(crate) fn emit_plain(&self) {
        let mut explicit_keys: Vec<String> = self.explicit_decisions.keys().cloned().collect();
        explicit_keys.sort_by_key(|issue| issue_number(issue));
        for issue in explicit_keys {
            if let Some(decision) = self.explicit_decisions.get(&issue) {
                println!("DEC|{}|{}", issue, decision);
            }
        }

        let mut inferred_keys: Vec<String> = self.inferred_decisions.keys().cloned().collect();
        inferred_keys.sort_by_key(|issue| issue_number(issue));
        for issue in inferred_keys {
            if let Some(decision) = self.inferred_decisions.get(&issue) {
                println!("INF|{}|{}", issue, decision);
            }
        }

        for record in &self.action_records {
            match record.record_type {
                DirectiveRecordType::Event => println!("EV|{}|{}", record.first, record.second),
                DirectiveRecordType::Duplicate => {
                    println!("DUP|{}|{}", record.first, record.second)
                }
                DirectiveRecordType::Decision => {}
            }
        }
    }
}

fn collect_explicit_decisions(records: &[DirectiveRecord]) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for record in records {
        if record.record_type != DirectiveRecordType::Decision {
            continue;
        }
        if record.second != "close" && record.second != "reopen" {
            continue;
        }
        out.insert(record.first.clone(), record.second.clone());
    }
    out
}

fn collect_inferred_decisions(
    records: &[DirectiveRecord],
    explicit_decisions: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut out = HashMap::new();

    for record in records {
        if record.record_type != DirectiveRecordType::Event {
            continue;
        }

        if explicit_decisions.contains_key(&record.second) {
            continue;
        }

        match record.first.as_str() {
            "Closes" => {
                out.insert(record.second.clone(), "close".to_string());
            }
            "Reopen" => {
                out.insert(record.second.clone(), "reopen".to_string());
            }
            "Cancel-Closes" => {
                if out.get(&record.second).map(String::as_str) == Some("close") {
                    out.remove(&record.second);
                }
            }
            _ => {}
        }
    }

    out
}

fn collect_action_records(records: &[DirectiveRecord]) -> Vec<DirectiveRecord> {
    let mut seen_duplicates: HashSet<String> = HashSet::new();
    let mut ordered_issues: Vec<String> = Vec::new();
    let mut effective_actions: HashMap<String, String> = HashMap::new();
    let mut out = Vec::new();

    for record in records {
        match record.record_type {
            DirectiveRecordType::Event => match record.first.as_str() {
                "Closes" | "Reopen" => {
                    if !ordered_issues.contains(&record.second) {
                        ordered_issues.push(record.second.clone());
                    }
                    effective_actions.insert(record.second.clone(), record.first.clone());
                }
                "Cancel-Closes" => {
                    if effective_actions.get(&record.second).map(String::as_str) == Some("Closes") {
                        effective_actions.remove(&record.second);
                    }
                }
                _ => {}
            },
            DirectiveRecordType::Duplicate => {
                let key = format!("{}|{}", record.first, record.second);
                if seen_duplicates.insert(key) {
                    out.push(record.clone());
                }
            }
            DirectiveRecordType::Decision => {}
        }
    }

    for issue in ordered_issues {
        if let Some(action) = effective_actions.get(&issue) {
            out.push(DirectiveRecord {
                record_type: DirectiveRecordType::Event,
                first: action.clone(),
                second: issue,
            });
        }
    }

    out
}

fn issue_number(issue_key: &str) -> u32 {
    issue_key
        .trim_start_matches('#')
        .parse::<u32>()
        .unwrap_or(u32::MAX)
}

pub(crate) fn run_directives_state(opts: PrDirectivesStateOptions) -> i32 {
    let state = State::build_state(&opts.text);
    state.emit_plain();
    0
}
