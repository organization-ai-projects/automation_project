use serde::{Deserialize, Serialize};

use crate::verify::verifier::VerifyResult;

/// Canonical, stable, deterministic report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchReport {
    pub plan_hash: String,
    pub content_hash: String,
    #[serde(deserialize_with = "crate::diagnostics::deserialize_usize_from_number")]
    pub file_count: usize,
    #[serde(deserialize_with = "crate::diagnostics::deserialize_usize_from_number")]
    pub op_count: usize,
    pub verified: bool,
}

impl PatchReport {
    pub fn from_verify(verify: &VerifyResult, op_count: usize) -> Self {
        Self {
            plan_hash: verify.plan_hash.clone(),
            content_hash: verify.content_hash.clone(),
            file_count: verify.file_count,
            op_count,
            verified: verify.ok,
        }
    }

    pub fn to_json(&self) -> Result<String, crate::diagnostics::error::PatchsmithError> {
        common_json::to_string(self).map_err(|e| {
            crate::diagnostics::error::PatchsmithError::Internal(format!(
                "report serialization: {e}"
            ))
        })
    }
}
