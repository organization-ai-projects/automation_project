//! tools/versioning_automation/src/pr/duplicate_actions.rs
use std::collections::BTreeMap;

pub(crate) fn parse_duplicate_targets(text: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let Some((duplicate_raw, canonical_raw)) = trimmed.split_once('|') else {
            continue;
        };
        let Some(duplicate_issue_key) = normalize_issue_key(duplicate_raw) else {
            continue;
        };
        let Some(canonical_issue_key) = normalize_issue_key(canonical_raw) else {
            continue;
        };
        out.insert(duplicate_issue_key, canonical_issue_key);
    }
    out
}

fn normalize_issue_key(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    let digits = trimmed.strip_prefix('#')?;
    if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    Some(format!("#{digits}"))
}
