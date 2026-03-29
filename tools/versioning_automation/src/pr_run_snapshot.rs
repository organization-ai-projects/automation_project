//! tools/versioning_automation/src/pr_run_snapshot.rs
use std::collections::BTreeMap;

use crate::category_resolver::resolve_issue_outcome_category;
use crate::compare_snapshot::CompareSnapshot;
use crate::pr::CommitInfo;
use crate::pr::IssueOutcomesSnapshot;

#[derive(Debug, Clone)]
pub(crate) struct PrRunSnapshot {
    pub(crate) compare: CompareSnapshot,
    pub(crate) validation_gate: String,
    pub(crate) duplicate_targets: BTreeMap<String, String>,
    pub(crate) issue_outcomes: IssueOutcomesSnapshot,
}

impl PrRunSnapshot {
    pub(crate) fn load_pr_run_snapshot(base_ref: &str, head_ref: &str) -> Result<Self, String> {
        let compare = CompareSnapshot::load_compare_snapshot(base_ref, head_ref)?;
        let validation_gate = CommitInfo::build_validation_gate(&compare.commits);
        let duplicate_targets = CommitInfo::collect_duplicate_targets(&compare.commits);
        let issue_outcomes = IssueOutcomesSnapshot::build_issue_outcomes_snapshot(
            &compare.commits,
            resolve_issue_outcome_category,
        );

        Ok(Self {
            compare,
            validation_gate,
            duplicate_targets,
            issue_outcomes,
        })
    }
}
