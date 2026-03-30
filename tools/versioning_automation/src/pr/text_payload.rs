//! tools/versioning_automation/src/pr/text_payload.rs
use std::collections::BTreeSet;

use regex::Regex;

use crate::pr::commands::PrTextPayloadOptions;
use crate::pr::{DirectiveRecord, DirectiveRecordType, State};
use crate::pr_remote_snapshot::PrRemoteSnapshot;
use crate::repo_name::resolve_repo_name;

pub(crate) fn run_text_payload(opts: PrTextPayloadOptions) -> i32 {
    let Ok(repo_name) = resolve_repo_name(opts.repo) else {
        println!();
        return 0;
    };

    let payload = load_pr_text_payload(&opts.pr_number, &repo_name).unwrap_or_default();
    print!("{payload}");
    0
}

pub(crate) fn load_pr_text_payload(pr_number: &str, repo_name: &str) -> Result<String, String> {
    let snapshot = PrRemoteSnapshot::load_pr_remote_snapshot(pr_number, repo_name)?;
    Ok(PrRemoteSnapshot::pr_text_payload_from_snapshot(&snapshot))
}

pub(crate) fn extract_effective_action_issue_numbers(
    payload: &str,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let (closes, reopens, _) = extract_effective_issue_ref_sets(payload);
    (closes, reopens)
}

pub(crate) fn extract_effective_issue_ref_records(payload: &str) -> Vec<DirectiveRecord> {
    let (closes, reopens, part_of) = extract_effective_issue_ref_sets(payload);
    let mut out = Vec::new();

    for issue in part_of {
        out.push(issue_ref_record("Part of", &issue));
    }
    for issue in closes {
        out.push(issue_ref_record("Closes", &issue));
    }
    for issue in reopens {
        out.push(issue_ref_record("Reopen", &issue));
    }

    out
}

pub(crate) fn extract_effective_issue_ref_sets(
    payload: &str,
) -> (BTreeSet<String>, BTreeSet<String>, BTreeSet<String>) {
    let mut closes = BTreeSet::new();
    let mut reopens = BTreeSet::new();
    let mut part_of = BTreeSet::new();
    let legacy_re =
        Regex::new(r"(?i)(fixes|resolves|related\s+to|reopens)\s+#([0-9]+)").expect("valid regex");

    for record in DirectiveRecord::scan_directives(payload, false) {
        if record.first == "Part of" {
            let issue_number = record.second.trim_start_matches('#').to_string();
            if !issue_number.is_empty() {
                part_of.insert(issue_number);
            }
        }
    }

    for cap in legacy_re.captures_iter(payload) {
        let keyword = cap
            .get(1)
            .map(|m| m.as_str().to_lowercase())
            .unwrap_or_default();
        let issue_number = cap
            .get(2)
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        if issue_number.is_empty() {
            continue;
        }
        match keyword.as_str() {
            "fixes" | "resolves" => {
                closes.insert(issue_number);
            }
            "reopens" => {
                reopens.insert(issue_number);
            }
            "related to" => {
                part_of.insert(issue_number);
            }
            _ => {}
        }
    }

    for record in State::build_state(payload).action_records {
        let issue_number = record.second.trim_start_matches('#').to_string();
        if issue_number.is_empty() {
            continue;
        }
        match record.first.as_str() {
            "Closes" => {
                closes.insert(issue_number.clone());
                reopens.remove(&issue_number);
            }
            "Reopen" => {
                reopens.insert(issue_number.clone());
                closes.remove(&issue_number);
            }
            _ => {}
        }
    }

    (closes, reopens, part_of)
}

fn issue_ref_record(action: &str, issue_number: &str) -> DirectiveRecord {
    DirectiveRecord {
        record_type: DirectiveRecordType::Event,
        first: action.to_string(),
        second: format!("#{issue_number}"),
    }
}
