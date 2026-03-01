use crate::config::enforcement_mode::EnforcementMode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub id: Option<String>,
    #[serde(flatten)]
    pub payload: RequestPayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum RequestPayload {
    Ping,
    CheckRepo {
        root_path: String,
        mode: EnforcementMode,
    },
    CheckProduct {
        product_path: String,
        mode: EnforcementMode,
    },
    GetReport {
        report_hash: String,
    },
    Shutdown,
}
