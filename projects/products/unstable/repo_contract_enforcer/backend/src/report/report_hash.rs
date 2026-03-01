use crate::report::report::Report;
use anyhow::Result;
use sha2::{Digest, Sha256};

pub struct ReportHash;

impl ReportHash {
    pub fn compute(report: &Report) -> Result<String> {
        let canonical = serde_json::to_vec(report)?;
        let digest = Sha256::digest(canonical);
        Ok(hex::encode(digest))
    }
}
