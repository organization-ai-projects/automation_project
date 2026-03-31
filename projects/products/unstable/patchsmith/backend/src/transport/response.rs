use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Response {
    PlanReport { plan_json: String },
    ApplyReport { report_json: String },
    VerifyReport { report_json: String },
    Error { message: String },
    Ok,
}
