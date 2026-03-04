use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ResponsePayload {
    Ok,
    Error {
        message: String,
    },
    Snapshot {
        hash: String,
        state_json: String,
    },
    Report {
        run_hash: String,
        report_json: String,
    },
    ReplayData {
        replay: String,
    },
}
