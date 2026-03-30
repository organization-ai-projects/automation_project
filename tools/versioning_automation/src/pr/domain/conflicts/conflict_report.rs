//! tools/versioning_automation/src/pr/domain/conflicts/conflict_report.rs
use std::collections::{HashMap, HashSet};

use crate::pr::directive_conflict_guard::{BLOCK_END, BLOCK_START};
use crate::pr::domain::conflicts::{ResolvedConflict, UnresolvedConflict};
use crate::pr::{DirectiveRecord, DirectiveRecordType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConflictReport {
    pub(crate) resolved: Vec<ResolvedConflict>,
    pub(crate) unresolved: Vec<UnresolvedConflict>,
}

impl ConflictReport {
    pub(crate) fn build_conflict_report(text: &str, source_branch_count: u32) -> Self {
        let mut closing_requested = HashSet::new();
        let mut reopen_requested = HashSet::new();
        let mut explicit_decision: HashMap<String, String> = HashMap::new();
        let mut inferred_decision: HashMap<String, String> = HashMap::new();

        for record in DirectiveRecord::scan_directives(text, false) {
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
                    "Cancel-Closes" => {
                        closing_requested.remove(&record.second);
                        if inferred_decision.get(&record.second).map(String::as_str)
                            == Some("close")
                        {
                            inferred_decision.remove(&record.second);
                        }
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

        Self {
            resolved,
            unresolved,
        }
    }

    pub(crate) fn emit_plain(report: &Self) {
        for entry in &report.resolved {
            println!("RES|{}|{}|{}", entry.issue, entry.decision, entry.origin);
        }
        for entry in &report.unresolved {
            println!("UNRES|{}|{}", entry.issue, entry.reason);
        }
    }

    pub(crate) fn build_conflict_block(&self) -> Option<String> {
        if self.resolved.is_empty() && self.unresolved.is_empty() {
            return None;
        }

        let mut out = String::new();
        out.push_str(BLOCK_START);
        out.push('\n');
        out.push_str("### Issue Directive Decisions");
        out.push('\n');

        if !self.resolved.is_empty() {
            out.push('\n');
            out.push_str("Resolved decisions:");
            out.push('\n');
            let mut keys = self.resolved.iter().collect::<Vec<_>>();
            keys.sort_by_key(|entry| issue_number(&entry.issue));
            for entry in keys {
                out.push_str("- ");
                out.push_str(&entry.issue);
                out.push_str(" => ");
                out.push_str(&entry.decision);
                out.push_str(" (");
                out.push_str(&entry.origin);
                out.push_str(")\n");
            }
        }

        if !self.unresolved.is_empty() {
            out.push('\n');
            out.push_str("❌ Unresolved conflicts (merge blocked):");
            out.push('\n');
            let mut keys = self.unresolved.iter().collect::<Vec<_>>();
            keys.sort_by_key(|entry| issue_number(&entry.issue));
            for entry in keys {
                out.push_str("- ");
                out.push_str(&entry.issue);
                out.push_str(": ");
                out.push_str(&entry.reason);
                out.push('\n');
            }
            out.push('\n');
            out.push_str("Required decision format:\n");
            out.push_str("- `Directive Decision: #<issue> => close`\n");
            out.push_str("- `Directive Decision: #<issue> => reopen`\n");
        }

        out.push_str(BLOCK_END);
        Some(out)
    }
}

fn issue_number(issue_key: &str) -> u32 {
    issue_key
        .trim_start_matches('#')
        .parse::<u32>()
        .unwrap_or(u32::MAX)
}
