use sha2::{Digest, Sha256};

use crate::diagnostics::error::SpaceDiploWarsError;
use crate::io::json_codec::JsonCodec;

use super::run_report::RunReport;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RunHash(pub String);

impl RunHash {
    /// Compute SHA-256 of canonical JSON bytes of the RunReport.
    pub fn compute(report: &RunReport) -> Result<Self, SpaceDiploWarsError> {
        let json = JsonCodec::encode(report)?;
        let bytes = json.as_bytes();
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        Ok(Self(hex::encode(result)))
    }
}
