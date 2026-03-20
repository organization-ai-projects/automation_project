use crate::orchestrator::Version;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceImportPolicy {
    pub allow_schema_change: bool,
    pub allow_version_regression: bool,
    pub max_version_regression: Option<Version>,
    pub require_policy_match: bool,
}

impl GovernanceImportPolicy {
    pub fn strict() -> Self {
        Self {
            allow_schema_change: false,
            allow_version_regression: false,
            max_version_regression: None,
            require_policy_match: false,
        }
    }
}

impl Default for GovernanceImportPolicy {
    fn default() -> Self {
        Self::strict()
    }
}
