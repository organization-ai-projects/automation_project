// projects/products/unstable/autonomous_dev_ai/src/symbolic/issue_category.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueCategory {
    Security,
    Features,
    BugFixes,
    Refactoring,
    Automation,
    Testing,
    Docs,
    Mixed,
    Unknown,
}
