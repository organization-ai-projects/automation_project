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
