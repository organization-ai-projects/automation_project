use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ResponsePayload {
    Ok,
    Error { message: String },
    TestReport { report_json: String },
    Transcript { transcript_json: String },
}
