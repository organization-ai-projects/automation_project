use std::collections::{HashMap, HashSet};

use crate::pr::commands::pr_directives_state_options::PrDirectivesStateOptions;
use crate::pr::domain::directives::directive_record::DirectiveRecord;
use crate::pr::domain::directives::directive_record_type::DirectiveRecordType;
use crate::pr::scan::scan_directives;

pub(crate) fn run_directives_state(opts: PrDirectivesStateOptions) -> i32 {
    let state = build_state(&opts.text);
    emit_plain(&state);
    0
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct State {
    pub(crate) explicit_decisions: HashMap<String, String>,
    pub(crate) inferred_decisions: HashMap<String, String>,
    pub(crate) action_records: Vec<DirectiveRecord>,
}

pub(crate) fn build_state(text: &str) -> State {
    let records = scan_directives(text, false);
    let explicit_decisions = collect_explicit_decisions(&records);
    let inferred_decisions = collect_inferred_decisions(&records, &explicit_decisions);
    let action_records = collect_action_records(&records);

    State {
        explicit_decisions,
        inferred_decisions,
        action_records,
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

        let decision = match record.first.as_str() {
            "Closes" => "close",
            "Reopen" => "reopen",
            _ => continue,
        };

        if explicit_decisions.contains_key(&record.second) {
            continue;
        }

        if let Some(existing) = out.get(&record.second) {
            if existing != decision {
                if decision == "reopen" || existing == "reopen" {
                    out.insert(record.second.clone(), "reopen".to_string());
                } else {
                    out.insert(record.second.clone(), "conflict".to_string());
                }
            }
        } else {
            out.insert(record.second.clone(), decision.to_string());
        }
    }

    out
}

fn collect_action_records(records: &[DirectiveRecord]) -> Vec<DirectiveRecord> {
    let mut seen_close_refs: HashSet<String> = HashSet::new();
    let mut seen_reopen_refs: HashSet<String> = HashSet::new();
    let mut seen_duplicates: HashSet<String> = HashSet::new();
    let mut out = Vec::new();

    for record in records {
        match record.record_type {
            DirectiveRecordType::Event => {
                if record.first == "Closes" {
                    if seen_close_refs.insert(record.second.clone()) {
                        out.push(record.clone());
                    }
                } else if record.first == "Reopen" && seen_reopen_refs.insert(record.second.clone())
                {
                    out.push(record.clone());
                }
            }
            DirectiveRecordType::Duplicate => {
                let key = format!("{}|{}", record.first, record.second);
                if seen_duplicates.insert(key) {
                    out.push(record.clone());
                }
            }
            DirectiveRecordType::Decision => {}
        }
    }

    out
}

fn emit_plain(state: &State) {
    let mut explicit_keys: Vec<String> = state.explicit_decisions.keys().cloned().collect();
    explicit_keys.sort_by_key(|issue| issue_number(issue));
    for issue in explicit_keys {
        if let Some(decision) = state.explicit_decisions.get(&issue) {
            println!("DEC|{}|{}", issue, decision);
        }
    }

    let mut inferred_keys: Vec<String> = state.inferred_decisions.keys().cloned().collect();
    inferred_keys.sort_by_key(|issue| issue_number(issue));
    for issue in inferred_keys {
        if let Some(decision) = state.inferred_decisions.get(&issue) {
            println!("INF|{}|{}", issue, decision);
        }
    }

    for record in &state.action_records {
        match record.record_type {
            DirectiveRecordType::Event => println!("EV|{}|{}", record.first, record.second),
            DirectiveRecordType::Duplicate => println!("DUP|{}|{}", record.first, record.second),
            DirectiveRecordType::Decision => {}
        }
    }
}

fn issue_number(issue_key: &str) -> u32 {
    issue_key
        .trim_start_matches('#')
        .parse::<u32>()
        .unwrap_or(u32::MAX)
}
