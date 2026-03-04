use crate::diagnostics::backend_error::BackendError;
use crate::io::canonical_json::to_canonical_string;
use crate::report::run_report::RunReport;
use sha2::{Digest, Sha256};

pub struct RunHash;

impl RunHash {
    pub fn compute(report: &RunReport) -> Result<String, BackendError> {
        let canonical = to_canonical_string(report).map_err(BackendError::Codec)?;
        let digest = Sha256::digest(canonical.as_bytes());
        Ok(hex::encode(digest))
    }
}
