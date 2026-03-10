use std::collections::{HashMap, HashSet};

use crate::pr::commands::pr_directive_conflicts_options::PrDirectiveConflictsOptions;
use crate::pr::domain::conflicts::conflict_report::ConflictReport;
use crate::pr::domain::conflicts::resolved_conflict::ResolvedConflict;
use crate::pr::domain::conflicts::unresolved_conflict::UnresolvedConflict;
use crate::pr::domain::directives::directive_record_type::DirectiveRecordType;
use crate::pr::scan::scan_directives;

pub(crate) fn run_directive_conflicts(opts: PrDirectiveConflictsOptions) -> i32 {
    let report = build_conflict_report(&opts.text, opts.source_branch_count);
    emit_plain(&report);
    0
}

pub(crate) fn build_conflict_report(text: &str, source_branch_count: u32) -> ConflictReport {
    let mut closing_requested = HashSet::new();
    let mut reopen_requested = HashSet::new();
    let mut explicit_decision: HashMap<String, String> = HashMap::new();
    let mut inferred_decision: HashMap<String, String> = HashMap::new();

    for record in scan_directives(text, false) {
        match record.record_type {
            DirectiveRecordType::Event => match record.first.as_str() {
                "Closes" => {
                    closing_requested.insert(record.second.clone());
                    inferred_decision.insert(record.second, "close".to_string());
                }
                "Reopen" => {
                    reopen_requested.insert(record.second.clone());
                    inferred_decision.insert(record.second, "reopen".to_string());
                }
                _ => {}
            },
            DirectiveRecordType::Decision => {
                if record.second == "close" || record.second == "reopen" {
                    explicit_decision.insert(record.first, record.second);
                }
            }
            DirectiveRecordType::Duplicate => {}
        }
    }

    let mut conflicting_issues: Vec<String> = closing_requested
        .intersection(&reopen_requested)
        .cloned()
        .collect();
    conflicting_issues.sort_by_key(|issue| issue_number(issue));

    let allow_inferred_resolution = source_branch_count <= 1;
    let mut resolved = Vec::new();
    let mut unresolved = Vec::new();

    for issue in conflicting_issues {
        if let Some(decision) = explicit_decision.get(&issue) {
            resolved.push(ResolvedConflict {
                issue,
                decision: decision.clone(),
                origin: "explicit".to_string(),
            });
            continue;
        }

        if allow_inferred_resolution && let Some(decision) = inferred_decision.get(&issue) {
            resolved.push(ResolvedConflict {
                issue,
                decision: decision.clone(),
                origin: "inferred from latest directive".to_string(),
            });
            continue;
        }

        let reason = if allow_inferred_resolution {
            "Closes + Reopen detected without explicit decision.".to_string()
        } else {
            "Closes + Reopen detected across multiple source branches; explicit decision required."
                .to_string()
        };
        unresolved.push(UnresolvedConflict { issue, reason });
    }

    ConflictReport {
        resolved,
        unresolved,
    }
}

fn emit_plain(report: &ConflictReport) {
    for entry in &report.resolved {
        println!("RES|{}|{}|{}", entry.issue, entry.decision, entry.origin);
    }
    for entry in &report.unresolved {
        println!("UNRES|{}|{}", entry.issue, entry.reason);
    }
}

fn issue_number(issue_key: &str) -> u32 {
    issue_key
        .trim_start_matches('#')
        .parse::<u32>()
        .unwrap_or(u32::MAX)
}
