//! tools/versioning_automation/src/issues/neutralize_ref_buckets.rs
use std::collections::HashSet;

use crate::issues::NeutralizeRef;
use regex::Regex;

pub(crate) struct NeutralizeRefBuckets(
    pub(crate) Vec<NeutralizeRef>,
    pub(crate) Vec<NeutralizeRef>,
);

impl NeutralizeRefBuckets {
    pub(crate) fn collect_neutralize_refs(text: &str) -> Self {
        let re = Regex::new(r"(?i)\b(closes|fixes)\b\s+(rejected\s+)?[^#\s]*#([0-9]+)")
            .expect("static regex must compile");
        let mut closing_refs: Vec<(String, String)> = Vec::new();
        let mut pre_neutralized_refs: Vec<(String, String)> = Vec::new();
        let mut seen_closing = HashSet::new();
        let mut seen_pre = HashSet::new();

        for captures in re.captures_iter(text) {
            let Some(issue_number) = captures.get(3).map(|m| m.as_str()) else {
                continue;
            };
            let issue_key = format!("#{issue_number}");
            if captures.get(2).is_some() {
                if seen_pre.insert(issue_key.clone()) {
                    pre_neutralized_refs.push(("Closes".to_string(), issue_key));
                }
            } else if seen_closing.insert(issue_key.clone()) {
                closing_refs.push(("Closes".to_string(), issue_key));
            }
        }

        Self(
            closing_refs
                .into_iter()
                .map(|(action, issue)| NeutralizeRef(action, issue))
                .collect(),
            pre_neutralized_refs
                .into_iter()
                .map(|(action, issue)| NeutralizeRef(action, issue))
                .collect(),
        )
    }
}
