//! tools/versioning_automation/src/automation/audit_issue_status.rs
use std::collections::{self, BTreeSet};

use crate::pr::extract_effective_issue_ref_sets;
use common_json::Json;

type IssueRefSets = (BTreeSet<String>, BTreeSet<String>, BTreeSet<String>);
type AuditIssueStatusSections<'a> = (
    &'a [String],
    &'a [String],
    &'a [String],
    &'a [String],
    &'a [String],
);

pub(crate) fn extract_issue_refs_from_text(text: &str) -> Result<IssueRefSets, String> {
    Ok(extract_effective_issue_ref_sets(text))
}

pub(crate) fn render_issue_audit_report(
    repo: &str,
    range: &str,
    total_open: usize,
    sections: AuditIssueStatusSections<'_>,
) -> String {
    let (
        done_in_dev_items,
        would_close_items,
        would_reopen_items,
        part_only_items,
        unreferenced_items,
    ) = sections;
    let mut out = Vec::new();
    out.push("# Issue Status Audit".to_string());
    out.push("".to_string());
    out.push(format!("- Repository: `{repo}`"));
    out.push(format!("- Range: `{range}`"));
    out.push("".to_string());
    out.push("## Summary".to_string());
    out.push("".to_string());
    out.push(format!("- Open issues fetched: {total_open}"));
    out.push(format!(
        "- Would close on merge: {}",
        would_close_items.len()
    ));
    out.push(format!(
        "- Would reopen from current refs: {}",
        would_reopen_items.len()
    ));
    out.push(format!(
        "- Done in dev (label): {}",
        done_in_dev_items.len()
    ));
    out.push(format!(
        "- Part-of-only (not closing): {}",
        part_only_items.len()
    ));
    out.push(format!(
        "- Unreferenced in range: {}",
        unreferenced_items.len()
    ));
    out.push("".to_string());
    out.push("## Done In Dev (Label)".to_string());
    out.push("".to_string());
    if done_in_dev_items.is_empty() {
        out.push("- None".to_string());
    } else {
        out.extend(done_in_dev_items.iter().cloned());
    }
    out.push("".to_string());
    out.push("## Would Close On Merge".to_string());
    out.push("".to_string());
    if would_close_items.is_empty() {
        out.push("- None".to_string());
    } else {
        out.extend(would_close_items.iter().cloned());
    }
    out.push("".to_string());
    out.push("## Would Reopen".to_string());
    out.push("".to_string());
    if would_reopen_items.is_empty() {
        out.push("- None".to_string());
    } else {
        out.extend(would_reopen_items.iter().cloned());
    }
    out.push("".to_string());
    out.push("## Part-Of Only".to_string());
    out.push("".to_string());
    if part_only_items.is_empty() {
        out.push("- None".to_string());
    } else {
        out.extend(part_only_items.iter().cloned());
    }
    out.push("".to_string());
    out.push("## Unreferenced".to_string());
    out.push("".to_string());
    if unreferenced_items.is_empty() {
        out.push("- None".to_string());
    } else {
        out.extend(unreferenced_items.iter().cloned());
    }
    out.push("".to_string());
    out.join("\n")
}

fn parse_json_array(payload: &str, context: &str) -> Result<Vec<Json>, String> {
    let parsed: Json = common_json::from_json_str(payload)
        .map_err(|e| format!("Failed to parse {context}: {e}"))?;
    parsed
        .as_array()
        .cloned()
        .ok_or_else(|| format!("Expected JSON array for {context}"))
}

fn object_u64(object: &collections::HashMap<String, Json>, key: &str) -> u64 {
    object.get(key).and_then(Json::as_u64).unwrap_or(0)
}

fn object_string(object: &collections::HashMap<String, Json>, key: &str) -> String {
    object
        .get(key)
        .and_then(Json::as_str)
        .unwrap_or_default()
        .to_string()
}
