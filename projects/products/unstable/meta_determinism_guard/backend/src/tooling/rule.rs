use crate::tooling::severity::Severity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub pattern: String,
    pub severity: Severity,
}
