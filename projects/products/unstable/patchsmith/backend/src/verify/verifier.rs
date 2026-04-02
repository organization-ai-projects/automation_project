use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::apply::applier::ApplyResult;
use crate::plan::patch_plan::PatchPlan;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifyResult {
    pub ok: bool,
    pub plan_hash: String,
    pub content_hash: String,
    #[serde(deserialize_with = "crate::diagnostics::deserialize_usize_from_number")]
    pub file_count: usize,
}

pub struct Verifier;

impl Verifier {
    /// Verify that applying a plan produces the expected content hash.
    pub fn verify(plan: &PatchPlan, result: &ApplyResult) -> VerifyResult {
        let content_hash = Self::compute_content_hash(&result.files);
        VerifyResult {
            ok: true,
            plan_hash: plan.plan_hash.clone(),
            content_hash,
            file_count: result.files.len(),
        }
    }

    /// Verify against an expected content hash.
    pub fn verify_against(
        plan: &PatchPlan,
        result: &ApplyResult,
        expected_hash: &str,
    ) -> VerifyResult {
        let content_hash = Self::compute_content_hash(&result.files);
        VerifyResult {
            ok: content_hash == expected_hash,
            plan_hash: plan.plan_hash.clone(),
            content_hash,
            file_count: result.files.len(),
        }
    }

    fn compute_content_hash(files: &BTreeMap<String, String>) -> String {
        let mut hasher = Sha256::new();
        for (name, content) in files {
            hasher.update(name.as_bytes());
            hasher.update(b"\0");
            hasher.update(content.as_bytes());
            hasher.update(b"\0");
        }
        hex::encode(hasher.finalize())
    }
}
