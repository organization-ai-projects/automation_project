// projects/libraries/versioning/src/modification_category.rs

use serde::{Deserialize, Serialize};

/// Categories for different types of modifications
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModificationCategory {
    BreakingModification,
    NewCapability,
    Enhancement,
    CorrectionApplied,
    SecurityUpdate,
    DeprecationNotice,
}

impl ModificationCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::BreakingModification => "Breaking Change",
            Self::NewCapability => "New Feature",
            Self::Enhancement => "Improvement",
            Self::CorrectionApplied => "Fix",
            Self::SecurityUpdate => "Security",
            Self::DeprecationNotice => "Deprecated",
        }
    }
}
