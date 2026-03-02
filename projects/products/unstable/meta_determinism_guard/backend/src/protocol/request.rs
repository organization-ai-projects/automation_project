use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Request {
    ScanForbidden { root: String },
    CheckCanonicalJson { path: String },
    RunStabilityHarness { cmd: String, runs: u32 },
    GetReport,
}
