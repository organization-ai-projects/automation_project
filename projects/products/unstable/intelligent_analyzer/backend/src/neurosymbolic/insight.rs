use serde::{Deserialize, Serialize};

use super::insight_kind::InsightKind;

/// An AI-generated insight about the analyzed source code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub kind: InsightKind,
    pub confidence: f64,
    pub message: String,
    pub strategy: String,
}

impl Insight {
    pub fn new(kind: InsightKind, confidence: f64, message: String, strategy: String) -> Self {
        Self {
            kind,
            confidence,
            message,
            strategy,
        }
    }
}
